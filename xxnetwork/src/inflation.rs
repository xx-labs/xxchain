use super::{Config, Module, LiquidityRewards};
use pallet_staking::EraPayout;
use pallet_staking_reward_fn::compute_inflation;
use sp_runtime::traits::{Zero, AtLeast32BitUnsigned, Saturating};
use sp_runtime::{Perbill, RuntimeDebug, SaturatedConversion};
use codec::{Encode, Decode};
use sp_std::{prelude::*, fmt::Debug};
use frame_support::{StorageValue, traits::Get};

/// Inflation fixed parameters
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct InflationFixedParams {
    /// Minimum inflation
    #[codec(compact)]
    pub min_inflation: Perbill,
    /// Ideal stake
    #[codec(compact)]
    pub ideal_stake: Perbill,
    /// Inflation curve falloff
    #[codec(compact)]
    pub falloff: Perbill,
}

/// Default inflation fixed params of:
/// min inflation = 2.5%
/// ideal stake = 50%
/// falloff = 50%
impl Default for InflationFixedParams {
    fn default() -> Self {
        InflationFixedParams {
            min_inflation: Perbill::from_rational(1u32, 40u32),
            ideal_stake: Perbill::from_rational(1u32, 2u32),
            falloff: Perbill::from_rational(1u32, 2u32)
        }
    }
}

/// Ideal interest point
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct IdealInterestPoint<B> {
    /// Block number
    pub block: B,
    /// Ideal inflation
    #[codec(compact)]
    pub interest: Perbill,
}

/// Default ideal interest point of:
/// block = 0
/// interest = 16.66666%
impl<B: Zero> Default for IdealInterestPoint<B> {
    fn default() -> Self {
        IdealInterestPoint {
            block: B::zero(),
            interest: Perbill::from_rational(1u32, 6u32),
        }
    }
}

/// Ideal interest curve example:
// 100% ___
//         \
//      |   \
//      |    \
//      |     \
//      |      \
//      |       \
//      |        \
//      |         \
//      |          \
//      |           \
//      |            \
//      |         20% \_____________
//      |                           |
//      |             |         15% |_____________
//      |             |                           |
//      |             |             |         10% |______________ ...
//      |             |             |
//      |             |             |             |
//      0           1 year       2 years       3 years

/// Points that encode the example curve
//  (0              , 100%)
//  (block(1 year)  ,  20%)
//  (block(2 year)  ,  20%)
//  (block(2 year)+1,  15%)
//  (block(3 year)  ,  15%)
//  (block(3 year)+1,  10%)

/// Implement Inflation sub module functions
impl<T: Config> Module<T> {
    /// Get the ideal interest according to block number
    fn get_ideal_interest(block: T::BlockNumber) -> Perbill {
        let points = Self::interest_points();
        match points.iter().position(|p| p.block >= block) {
            // If position found, get points from index-1 and index
            Some(index) =>
                Self::compute_ideal_interest(
                    block,
                    points[index-1].clone(),
                    points[index].clone(),
                ),
            // If none found, get last point and return interest
            None =>
                points.last().unwrap().interest,
        }
    }

    /// Compute ideal interest from two points
    /// There are only two options: linear decreasing or constant
    fn compute_ideal_interest(
        block: T::BlockNumber,
        start: IdealInterestPoint<T::BlockNumber>,
        end: IdealInterestPoint<T::BlockNumber>) -> Perbill {
        // Compute interest difference (start-end to ensure result is positive)
        let diff = start.interest.clone().saturating_sub(end.interest);
        // If difference is zero, must be constant part, take interest from start (or end)
        if diff.is_zero() {
            return start.interest
        }
        // Compute block ratio
        let block_diff = end.block - start.block;
        let half_era_blocks = Perbill::from_rational(1u32,2u32) * T::EraDuration::get();
        let ratio = Perbill::from_rational(block - half_era_blocks, block_diff);
        // Compute interest = start - diff*ratio
        start.interest.saturating_sub(ratio * diff)
    }

    /// Update liquidity rewards balance
    fn update_liquidity_rewards(portion: Perbill, interest: Perbill) {
        // Calculate ideal rewards based on interest and ideal stake
        let payout = portion * (interest * Self::ideal_stake_rewards());
        // Update balance
        <LiquidityRewards<T>>::mutate(|balance| *balance = balance.saturating_sub(payout));
    }

    /// Compute total stakeable
    fn compute_total_stakeable
        <Balance: AtLeast32BitUnsigned>
    (issuance: Balance) -> Balance {
        let unstakeable =
            // Balance of Rewards Pool
            Self::rewards_balance()
            // add total balance under custody
            + Self::total_custody()
            // add liquidity rewards balance
            + Self::liquidity_rewards();
        issuance
            // Ugly conversion from associated type BalanceOf<T> to u128 back to Balance parameter
            - Balance::try_from(unstakeable.saturated_into::<u128>()).ok().unwrap_or(Zero::zero())
    }
}

/// Implement EraPayout trait
impl<
    T: Config,
    Balance: AtLeast32BitUnsigned + Clone + Debug,
> EraPayout<Balance> for Module<T> {
    fn era_payout(
        total_staked: Balance,
        total_issuance: Balance,
        era_duration_millis: u64,
    ) -> (Balance, Balance) {
        // Get inflation fixed params
        let params = Self::inflation_params();

        // Get the block number from the FRAME System module.
        let block = <frame_system::Pallet<T>>::block_number();

        // Get ideal interest for given block
        let ideal_interest = Self::get_ideal_interest(block);

        // Compute max inflation
        let max = ideal_interest.clone() * params.ideal_stake.clone();

        // Compute total stakeable amount
        let total_stakeable = Self::compute_total_stakeable::<Balance>(total_issuance);

        // Ensure stake is at most total_stakeable
        let stake = total_staked.min(total_stakeable.clone());

        // Compute stake ratio
        let stake_ratio = Perbill::from_rational(stake, total_stakeable.clone());

        // Compute inflation
        let inflation_ratio = compute_inflation(stake_ratio, params.ideal_stake, params.falloff);
        let inflation = params.min_inflation.clone().saturating_add(
            inflation_ratio * (max.clone().saturating_sub(params.min_inflation)));

        // Milliseconds per year for the Julian year (365.25 days).
        const MILLISECONDS_PER_YEAR: u64 = 1000 * 3600 * 24 * 36525 / 100;
        let portion = Perbill::from_rational(era_duration_millis, MILLISECONDS_PER_YEAR);

        // Update liquidity rewards balance
        Self::update_liquidity_rewards(portion, ideal_interest);

        // Compute validator payout
        let validator_payout = portion * (inflation * total_stakeable.clone());
        let max_payout = portion * (max * total_stakeable);
        let rest = max_payout.saturating_sub(validator_payout.clone());

        (validator_payout, rest)
    }
}
