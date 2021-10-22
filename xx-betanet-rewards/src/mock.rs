// This file is part of Substrate.

// Copyright (C) 2018-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Added as part the code review and testing
// by ChainSafe Systems Aug 2021

use crate as xx_betanet_rewards;
use crate::*;

use frame_support::{
    parameter_types,
    ord_parameter_types,
    traits::{GenesisBuild, OnInitialize},
    PalletId,
    weights::constants::RocksDbWeight,
};
use frame_system::EnsureSignedBy;
use pallet_staking::EraIndex;
use sp_runtime::{
    testing::{Header, TestXt, H256},
    traits::{IdentityLookup, ConvertInto},
};
use sp_io::hashing::keccak_256;
use libsecp256k1::{SecretKey, PublicKey, Message, sign};
use claims::{EthereumAddress, EcdsaSignature, to_ascii_hex};

/// The AccountId alias in this test module.
pub(crate) type AccountId = u64;
pub(crate) type AccountIndex = u64;
pub(crate) type BlockNumber = u64;
pub(crate) type Balance = u128;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        Vesting: pallet_vesting::{Pallet, Call, Storage, Config<T>, Event<T>},
        Claims: claims::{Pallet, Call, Storage, Config<T>, Event<T>, ValidateUnsigned},
        XXEconomics: xx_economics::{Pallet, Call, Storage, Config<T>, Event<T>},
        XXBetanetRewards: xx_betanet_rewards::{Pallet, Call, Storage, Config<T>, Event<T>},
    }
);


parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub BlockWeights: frame_system::limits::BlockWeights =
        frame_system::limits::BlockWeights::simple_max(
            frame_support::weights::constants::WEIGHT_PER_SECOND * 2
        );
    pub const MaxLocks: u32 = 1024;
    pub static ExistentialDeposit: Balance = 1;
    pub static SlashDeferDuration: EraIndex = 0;
    pub static Period: BlockNumber = 5;
    pub static Offset: BlockNumber = 0;
}

impl frame_system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = RocksDbWeight;
    type Origin = Origin;
    type Index = AccountIndex;
    type BlockNumber = BlockNumber;
    type Call = Call;
    type Hash = H256;
    type Hashing = ::sp_runtime::traits::BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
}
impl pallet_balances::Config for Test {
    type MaxLocks = MaxLocks;
    type Balance = Balance;
    type Event = Event;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
}

parameter_types! {
    pub const MinVestedTransfer: u64 = 0;
}

impl pallet_vesting::Config for Test {
    type Event = Event;
    type Currency = Balances;
    type BlockNumberToBalance = ConvertInto;
    type MinVestedTransfer = MinVestedTransfer;
    type WeightInfo = ();
    const MAX_VESTING_SCHEDULES: u32 = 28;
}

parameter_types!{
    pub Prefix: &'static [u8] = b"Pay RUSTs to the TEST account:";
}

ord_parameter_types!{
    pub const Six: AccountId = 6;
}

impl claims::Config for Test {
    type Event = Event;
    type VestingSchedule = Vesting;
    type Prefix = Prefix;
    type MoveClaimOrigin = EnsureSignedBy<Six, AccountId>;
    type RewardHandler = XXBetanetRewards;
    type WeightInfo = claims::weights::TestWeightInfo;
}

pub struct MockCustodyHandler;

impl pallet_staking::CustodyHandler<AccountId, Balance> for MockCustodyHandler {
    fn is_custody_account(_: &AccountId) -> bool { false }
    fn total_custody() -> Balance { Balance::zero() }
}

parameter_types! {
    pub const RewardsPoolId: PalletId = PalletId(*b"xx/rwrds");
    pub const EraDuration: BlockNumber = 10; // 10 blocks per era
}

impl xx_economics::Config for Test {
    type Event = Event;
    type Currency = Balances;
    type CustodyHandler = MockCustodyHandler;
    type RewardsPoolId = RewardsPoolId;
    type RewardRemainder = ();
    type EraDuration = EraDuration;
    type AdminOrigin = EnsureSignedBy<Six, AccountId>;
    type WeightInfo = ();
}

parameter_types! {
	pub const BetanetStakingRewardsBlock: BlockNumber = 10;
	pub const OneMonthVest: BlockNumber = 432_000;
	pub const ThreeMonthVest: BlockNumber = 3*OneMonthVest::get();
	pub const SixMonthVest: BlockNumber = 6*OneMonthVest::get();
	pub const NineMonthVest: BlockNumber = 9*OneMonthVest::get();
	pub const OneYearVest: BlockNumber = 5_259_600;
	pub const Decimals: Balance = 1_000_000_000;
	pub const RewardBalance: Balance = 1_000_000 * Decimals::get();
	pub const RewardAmount: Balance = 10_000 * Decimals::get();
	pub const ClaimBalance: Balance = 25_000 * Decimals::get();
	pub const ClaimRewardAmount: Balance = 5_000 * Decimals::get();
	// ClaimBalance / OneYearVest
	pub const ClaimVestingPerBlock: Balance = 4_753_213;
	// ClaimBalance + ClaimRewardAmount / OneYearVest
	pub const ClaimVestingWithRewardsPerBlock: Balance = 5_703_856;
	// Rewards Pool balance
	pub const RewardsPoolBalance: Balance = 100_000_000 * Decimals::get();
}

impl xx_betanet_rewards::Config for Test {
    type Event = Event;
    type EnactmentBlock = BetanetStakingRewardsBlock;
    type Reward = XXEconomics;
}

pub type Extrinsic = TestXt<Call, ()>;

impl<LocalCall> frame_system::offchain::SendTransactionTypes<LocalCall> for Test
where
    Call: From<LocalCall>,
{
    type OverarchingCall = Call;
    type Extrinsic = Extrinsic;
}

pub(crate) fn public(secret: &SecretKey) -> PublicKey {
    PublicKey::from_secret_key(secret)
}

pub(crate) fn eth(secret: &SecretKey) -> EthereumAddress {
    let mut res = EthereumAddress::default();
    res.0.copy_from_slice(&keccak_256(&public(secret).serialize()[1..65])[12..]);
    res
}

pub(crate) fn ethereum_signable_message<T: claims::Config> (what: &[u8], extra: &[u8]) -> Vec<u8> {
    let prefix = T::Prefix::get();
    let mut l = prefix.len() + what.len() + extra.len();
    let mut rev = Vec::new();
    while l > 0 {
        rev.push(b'0' + (l % 10) as u8);
        l /= 10;
    }
    let mut v = b"\x19Ethereum Signed Message:\n".to_vec();
    v.extend(rev.into_iter().rev());
    v.extend_from_slice(&prefix[..]);
    v.extend_from_slice(what);
    v.extend_from_slice(extra);
    v
}

pub(crate) fn sig<T: Config>(secret: &SecretKey, what: &[u8], extra: &[u8]) -> EcdsaSignature {
    let msg = keccak_256(&ethereum_signable_message::<Test>(&to_ascii_hex(what)[..], extra));
    let (sig, recovery_id) = sign(&Message::parse(&msg), secret);
    let mut r = [0u8; 65];
    r[0..64].copy_from_slice(&sig.serialize()[..]);
    r[64] = recovery_id.serialize();
    EcdsaSignature(r)
}

pub(crate) fn alice() -> SecretKey {
    SecretKey::parse(&keccak_256(b"Alice")).unwrap()
}

pub(crate) fn bob() -> SecretKey {
    SecretKey::parse(&keccak_256(b"Bob")).unwrap()
}

pub(crate) fn charlie() -> SecretKey {
    SecretKey::parse(&keccak_256(b"Charlie")).unwrap()
}

pub(crate) fn dave() -> SecretKey {
    SecretKey::parse(&keccak_256(b"Dave")).unwrap()
}

pub(crate) fn eve() -> SecretKey {
    SecretKey::parse(&keccak_256(b"Eve")).unwrap()
}

pub(crate) fn frank() -> SecretKey {
    SecretKey::parse(&keccak_256(b"Frank")).unwrap()
}

pub struct ExtBuilder {
    with_claims: bool,
    with_vesting: bool,
}

impl Default for ExtBuilder {
    fn default() -> Self {
        Self {
            with_claims: false,
            with_vesting: false,
        }
    }
}

impl ExtBuilder {

    pub fn with_claims(mut self) -> Self {
        self.with_claims = true;
        self
    }

    pub fn with_vesting(mut self) -> Self {
        self.with_vesting = true;
        self
    }

    pub fn build(self) -> sp_io::TestExternalities {
        sp_tracing::try_init_simple();
        let mut storage = frame_system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();

        pallet_balances::GenesisConfig::<Test> {
            balances: vec![
                // 5 reward options + default
                (10, RewardBalance::get()),
                (20, RewardBalance::get()),
                (30, RewardBalance::get()),
                (40, RewardBalance::get()),
                (50, RewardBalance::get()),
                (60, RewardBalance::get()),
            ],
        }.assimilate_storage(&mut storage).unwrap();

        if self.with_vesting {
            // add vesting of 1 year to reward accounts
            pallet_vesting::GenesisConfig::<Test> {
                vesting: vec![
                    (10, 0, OneYearVest::get(), 0),
                    (20, 0, OneYearVest::get(), 0),
                    (30, 0, OneYearVest::get(), 0),
                    (40, 0, OneYearVest::get(), 0),
                    (50, 0, OneYearVest::get(), 0),
                    (60, 0, OneYearVest::get(), 0),
                ]
            }.assimilate_storage(&mut storage).unwrap();
        }

        if self.with_claims {
            claims::GenesisConfig::<Test> {
                claims: vec![
                    (eth(&alice()), ClaimBalance::get(), Some(ClaimRewardAmount::get()), None, None),
                    (eth(&bob()), ClaimBalance::get(), Some(ClaimRewardAmount::get()), None, None),
                    (eth(&charlie()), ClaimBalance::get(), Some(ClaimRewardAmount::get()), None, None),
                    (eth(&dave()), ClaimBalance::get(), Some(ClaimRewardAmount::get()), None, None),
                    (eth(&eve()), ClaimBalance::get(), None, None, None),
                    (eth(&frank()), ClaimBalance::get(), None, None, None),
                ],
                vesting: vec![
                    (eth(&bob()), (ClaimBalance::get(), ClaimVestingPerBlock::get(), 0)),
                    (eth(&dave()), (ClaimBalance::get(), ClaimVestingPerBlock::get(), 0)),
                ],
            }.assimilate_storage(&mut storage).unwrap();
        }

        xx_betanet_rewards::GenesisConfig::<Test> {
            accounts: vec![
                (10, UserInfo::<Balance> {
                    principal: RewardBalance::get(),
                    reward: RewardAmount::get(),
                    option: RewardOption::Vesting6Month,
                }),
                (20, UserInfo::<Balance> {
                    principal: RewardBalance::get(),
                    reward: RewardAmount::get(),
                    option: RewardOption::Vesting6Month,
                }),
                (30, UserInfo::<Balance> {
                    principal: RewardBalance::get(),
                    reward: RewardAmount::get(),
                    option: RewardOption::Vesting6Month,
                }),
                (40, UserInfo::<Balance> {
                    principal: RewardBalance::get(),
                    reward: RewardAmount::get(),
                    option: RewardOption::Vesting6Month,
                }),
                (50, UserInfo::<Balance> {
                    principal: RewardBalance::get(),
                    reward: RewardAmount::get(),
                    option: RewardOption::Vesting6Month,
                }),
                (60, UserInfo::<Balance> {
                    principal: RewardBalance::get(),
                    reward: RewardAmount::get(),
                    option: RewardOption::Vesting6Month,
                }),
            ],
        }.assimilate_storage(&mut storage).unwrap();

        xx_economics::GenesisConfig::<Test> {
            balance: RewardsPoolBalance::get(),
            ..Default::default()
        }.assimilate_storage(&mut storage).unwrap();

        let ext = sp_io::TestExternalities::from(storage);
        ext
    }
    pub fn build_and_execute(self, test: impl FnOnce() -> ()) {
        let mut ext = self.build();

        ext.execute_with(|| {
            System::set_block_number(1);
        });

        ext.execute_with(test);
    }
}

pub(crate) fn run_to_block(n: BlockNumber) {
    for b in (System::block_number() + 1)..=n {
        System::set_block_number(b);
        XXBetanetRewards::on_initialize(b);
    }
}

pub(crate) fn xx_betanet_rewards_events() -> Vec<xx_betanet_rewards::Event<Test>> {
    System::events()
        .into_iter()
        .map(|r| r.event)
        .filter_map(|e| {
            if let Event::XXBetanetRewards(inner) = e {
                Some(inner)
            } else {
                None
            }
        })
        .collect()
}

pub(crate) fn confirm_reward_result(who: &AccountId, option: RewardOption, claim: bool, vesting: bool) -> Balance {
    // Compute reward
    let reward = if claim {
        (option.extra_rewards() * ClaimRewardAmount::get()) + (option.rewards() * ClaimRewardAmount::get())
    } else {
        (option.extra_rewards() * RewardAmount::get()) + (option.rewards() * RewardAmount::get())
    };

    // Check balance
    let expected_balance = if claim {
        ClaimBalance::get()
    } else {
        RewardBalance::get()
    };
    assert_eq!(
        Balances::free_balance(who),
        expected_balance + reward,
    );

    // Check vesting schedule
    match option {
        RewardOption::NoVesting => (),
        _ => {
            let locked = if vesting {
                reward
            } else {
                option.principal_lock() * expected_balance + reward
            };
            let per_block = locked / option.vesting_period() as Balance;
            let schedules = Vesting::vesting(who);
            assert_eq!(
                schedules.unwrap().last().unwrap(),
                &pallet_vesting::VestingInfo::new(
                    locked,
                    per_block,
                    BetanetStakingRewardsBlock::get()),
            );
        }
    }

    // Return reward
    reward
}

pub(crate) fn confirm_claim_rewards_added(who: &AccountId) {
    assert_eq!(
        <Accounts<Test>>::get(who),
        UserInfo {
            principal: ClaimBalance::get(),
            reward: ClaimRewardAmount::get(),
            option: RewardOption::Vesting6Month,
        }
    );
}


pub(crate) fn confirm_leftover_claim_rewards_added(who: &EthereumAddress, vesting: bool) -> Balance {
    // Check reward was added to claim value
    assert_eq!(
        <claims::Claims<Test>>::get(who).unwrap(),
        ClaimBalance::get()+ClaimRewardAmount::get(),
    );

    // Check claim vesting schedule was changed to include reward amount
    if vesting {
        assert_eq!(
            <claims::Vesting<Test>>::get(who).unwrap(),
            (ClaimBalance::get()+ClaimRewardAmount::get(), ClaimVestingWithRewardsPerBlock::get(), 0u64),
        );
    }
    // Leftover claims always get default option
    ClaimRewardAmount::get()
}

pub(crate) fn confirm_leftover_claim_without_rewards(who: &EthereumAddress) {
    // Check claim value didn't change
    assert_eq!(
        <claims::Claims<Test>>::get(who).unwrap(),
        ClaimBalance::get(),
    );
}
