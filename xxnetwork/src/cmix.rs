use sp_runtime::{Permill, RuntimeDebug};
use codec::{Encode, Decode};
use sp_std::prelude::*;

/// CMIX software hashes
#[derive(PartialEq, Eq, Clone, Encode, Decode, Default, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct SoftwareHashes<Hash> {
    /// Server binary
    server: Hash,
    /// FatBin library
    fatbin: Hash,
    /// Libpow library
    libpow: Hash,
    /// Gateway binary
    gateway: Hash,
    /// Scheduling server binary
    scheduling: Hash,
    /// Wrapper script
    wrapper: Hash,
    /// User discovery bot binary
    udb: Hash,
    /// Notifications bot binary
    notifications: Hash,
    /// Extra
    extra: Option<Vec<Hash>>,
}

/// Country code type
type CountryCode = [u8; 2];

/// Geographic bin type
type GeoBin = u8;

/// Points multiplier type
type PointsMultiplier = u16;

/// Reward Points
#[derive(PartialEq, Eq, Clone, Encode, Decode, Default, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct RewardPoints {
    /// Points per successful real-time round
    success: u32,
    /// Points per failed real-time round (negative)
    failure: u32,
    /// Points per block produced
    block: u32,
}

/// Performance measurement variables
#[derive(PartialEq, Eq, Clone, Encode, Decode, Default, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct Performance {
    /// Period for round performance data collection
    period: u64,
    /// Reward points
    points: RewardPoints,
    /// List of countries and their geographic bins
    countries: Vec<(CountryCode, GeoBin)>,
    /// List of geographic bins and their points multiplier
    multipliers: Vec<(GeoBin, PointsMultiplier)>,
}

/// Round Timeouts
#[derive(PartialEq, Eq, Clone, Encode, Decode, Default, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct Timeouts {
    /// Round precomputation timeout
    precomputation: u64,
    /// Round realtime timeout
    realtime: u64,
    /// Round advertisement time
    advertisement: u64,
}

/// Scheduling variables
#[derive(PartialEq, Eq, Clone, Encode, Decode, Default, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct Scheduling {
    /// Mix team size
    team_size: u8,
    /// Mix batch size
    batch_size: u32,
    /// Minimum delay between round assignments
    min_delay: u64,
    /// Minimum number of nodes in the waiting pool before rounds can be scheduled
    pool_threshold: Permill,
}

/// User Registration variables
#[derive(PartialEq, Eq, Clone, Encode, Decode, Default, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct UserRegistration {
    /// Maximum number of user registrations per period
    max: u32,
    /// Period of user registration
    period: u64,
}

/// CMIX Variables
#[derive(PartialEq, Eq, Clone, Encode, Decode, Default, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct Variables {
    /// Performance measurement variables
    performance: Performance,
    /// Round timeouts
    timeouts: Timeouts,
    /// Scheduling
    scheduling: Scheduling,
    /// User registration
    registration: UserRegistration,
}

impl Variables {
    pub fn get_block_points(&self) -> u32 {
        self.performance.points.block
    }
}
