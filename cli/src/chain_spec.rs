// This file is part of Substrate.

// Copyright (C) 2018-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Substrate chain configurations.

use sc_chain_spec::{ChainSpecExtension, ChainType, ChainSpec};
use sp_core::{Pair, Public, sr25519};
use serde::{Serialize, Deserialize};
pub use xxnetwork_runtime as xxnetwork;
pub use protonet_runtime as protonet;
pub use phoenixx_runtime as phoenixx;
use phoenixx::constants::currency::*;
use hex_literal::hex;
use grandpa_primitives::{AuthorityId as GrandpaId};
use sp_consensus_babe::{AuthorityId as BabeId};
use pallet_im_online::sr25519::{AuthorityId as ImOnlineId};
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_runtime::{Perbill, traits::{Verify, IdentifyAccount}};

pub use node_primitives::{AccountId, Balance, Block, Signature, Hash};

type AccountPublic = <Signature as Verify>::Signer;

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
	/// Block numbers with known hashes.
	pub fork_blocks: sc_client_api::ForkBlocks<Block>,
	/// Known bad block hashes.
	pub bad_blocks: sc_client_api::BadBlocks<Block>,
	/// The light sync state extension used by the sync-state rpc.
	pub light_sync_state: sc_sync_state_rpc::LightSyncStateExtension,
}

/// The `ChainSpec` parameterized for the `xxnetwork` runtime.
pub type XXNetworkChainSpec = sc_service::GenericChainSpec<xxnetwork::GenesisConfig, Extensions>;

/// The `ChainSpec` parameterized for the `protonet` runtime.
pub type ProtonetChainSpec = sc_service::GenericChainSpec<protonet::GenesisConfig, Extensions>;

/// The `ChainSpec` parameterized for the `phoenixx` runtime.
pub type PhoenixxChainSpec = sc_service::GenericChainSpec<phoenixx::GenesisConfig, Extensions>;

/// Genesis config for `xxnetwork` mainnet
pub fn xxnetwork_config() -> Result<XXNetworkChainSpec, String> {
	XXNetworkChainSpec::from_json_bytes(&include_bytes!("../res/xxnetwork.json")[..])
}

/// Genesis config for `protonet` testnet
pub fn protonet_config() -> Result<ProtonetChainSpec, String> {
	ProtonetChainSpec::from_json_bytes(&include_bytes!("../res/protonet.json")[..])
}

/// Genesis config for `phoenixx` testnet
pub fn phoenixx_config() -> Result<PhoenixxChainSpec, String> {
	PhoenixxChainSpec::from_json_bytes(&include_bytes!("../res/phoenixx.json")[..])
}

/// Can be called for a `Configuration` to identify which network the configuration targets.
pub trait IdentifyVariant {
	/// Returns if this is a configuration for the `xxnetwork` network.
	fn is_xxnetwork(&self) -> bool;

	/// Returns if this is a configuration for the `protonet` network.
	fn is_protonet(&self) -> bool;

	/// Returns if this is a configuration for the `phoenixx` network.
	fn is_phoenixx(&self) -> bool;
}

impl IdentifyVariant for Box<dyn ChainSpec> {
	fn is_xxnetwork(&self) -> bool {
		self.id().starts_with("xxnetwork")
	}
	fn is_protonet(&self) -> bool {
		self.id().starts_with("xxprotonet")
	}
	fn is_phoenixx(&self) -> bool {
		self.id().starts_with("phoenixx")
	}
}

fn xxnetwork_session_keys(
	grandpa: GrandpaId,
	babe: BabeId,
	im_online: ImOnlineId,
	authority_discovery: AuthorityDiscoveryId,
) -> xxnetwork::SessionKeys {
	xxnetwork::SessionKeys { grandpa, babe, im_online, authority_discovery }
}

fn protonet_session_keys(
	grandpa: GrandpaId,
	babe: BabeId,
	im_online: ImOnlineId,
	authority_discovery: AuthorityDiscoveryId,
) -> protonet::SessionKeys {
	protonet::SessionKeys { grandpa, babe, im_online, authority_discovery }
}

fn phoenixx_session_keys(
	grandpa: GrandpaId,
	babe: BabeId,
	im_online: ImOnlineId,
	authority_discovery: AuthorityDiscoveryId,
) -> phoenixx::SessionKeys {
	phoenixx::SessionKeys { grandpa, babe, im_online, authority_discovery }
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn authority_keys_from_seed(seed: &str) -> (
	AccountId,
	AccountId,
	GrandpaId,
	BabeId,
	ImOnlineId,
	AuthorityDiscoveryId,
) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<BabeId>(seed),
		get_from_seed::<ImOnlineId>(seed),
		get_from_seed::<AuthorityDiscoveryId>(seed),
	)
}

/// Helper function to create GenesisConfig for testing of the `phoenixx` network
pub fn phoenixx_testnet_genesis(
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		GrandpaId,
		BabeId,
		ImOnlineId,
		AuthorityDiscoveryId,
	)>,
	initial_nominators: Vec<AccountId>,
	endowed_accounts: Option<Vec<AccountId>>,
) -> phoenixx::GenesisConfig {
	let mut endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
		vec![
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			get_account_id_from_seed::<sr25519::Public>("Bob"),
			get_account_id_from_seed::<sr25519::Public>("Charlie"),
			get_account_id_from_seed::<sr25519::Public>("Dave"),
			get_account_id_from_seed::<sr25519::Public>("Eve"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie"),
			get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
			get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
			get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
			get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
			get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
		]
	});
	// endow all authorities and nominators.
	initial_authorities.iter().map(|x| &x.0).chain(initial_nominators.iter()).for_each(|x| {
		if !endowed_accounts.contains(&x) {
			endowed_accounts.push(x.clone())
		}
	});

	// stakers: all validators and nominators.
	let mut rng = rand::thread_rng();
	let stakers = initial_authorities
		.iter()
		.enumerate()
		.map(|(i, x)| (x.0.clone(), x.1.clone(), STASH, phoenixx::StakerStatus::Validator(Some(Hash::repeat_byte(i as u8)))))
		.chain(initial_nominators.iter().map(|x| {
			use rand::{seq::SliceRandom, Rng};
			let limit = (phoenixx::MAX_NOMINATIONS as usize).min(initial_authorities.len());
			let count = rng.gen::<usize>() % limit;
			let nominations = initial_authorities
				.as_slice()
				.choose_multiple(&mut rng, count)
				.into_iter()
				.map(|choice| choice.0.clone())
				.collect::<Vec<_>>();
			(x.clone(), x.clone(), STASH, phoenixx::StakerStatus::Nominator(nominations))
		}))
		.collect::<Vec<_>>();

	let num_endowed_accounts = endowed_accounts.len();

	const ENDOWMENT: Balance = 10_000_000 * UNITS;
	const STASH: Balance = ENDOWMENT / 1000;

	phoenixx::GenesisConfig {
		system: phoenixx::SystemConfig {
			code: phoenixx::wasm_binary_unwrap().to_vec(),
			changes_trie_config: Default::default(),
		},
		balances: phoenixx::BalancesConfig {
			balances: endowed_accounts.iter().cloned()
				.map(|x| (x, ENDOWMENT))
				.collect()
		},
		session: phoenixx::SessionConfig {
			keys: initial_authorities.iter().map(|x| {
				(x.0.clone(), x.0.clone(), phoenixx_session_keys(
					x.2.clone(),
					x.3.clone(),
					x.4.clone(),
					x.5.clone(),
				))
			}).collect::<Vec<_>>(),
		},
		staking: phoenixx::StakingConfig {
			validator_count: initial_authorities.len() as u32 * 2,
			minimum_validator_count: initial_authorities.len() as u32,
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			stakers,
			.. Default::default()
		},
		democracy: phoenixx::DemocracyConfig::default(),
		elections: phoenixx::ElectionsConfig {
			members: endowed_accounts.iter()
						.take((num_endowed_accounts + 1) / 2)
						.cloned()
						.map(|member| (member, STASH))
						.collect(),
		},
		council: phoenixx::CouncilConfig::default(),
		technical_committee: phoenixx::TechnicalCommitteeConfig {
			members: endowed_accounts.iter()
						.take((num_endowed_accounts + 1) / 2)
						.cloned()
						.collect(),
			phantom: Default::default(),
		},
		babe: phoenixx::BabeConfig {
			authorities: vec![],
			epoch_config: Some(xxnetwork_runtime::BABE_GENESIS_EPOCH_CONFIG),
		},
		im_online: phoenixx::ImOnlineConfig {
			keys: vec![],
		},
		authority_discovery: phoenixx::AuthorityDiscoveryConfig {
			keys: vec![],
		},
		grandpa: phoenixx::GrandpaConfig {
			authorities: vec![],
		},
		technical_membership: Default::default(),
		treasury: Default::default(),
		vesting: Default::default(),
		claims: Default::default(),
		swap: phoenixx::SwapConfig {
			swap_fee: 1 * UNITS,
			fee_destination: get_account_id_from_seed::<sr25519::Public>("Alice"),
			chains: vec![1],
			relayers: vec![get_account_id_from_seed::<sr25519::Public>("Alice")],
			resources: vec![
				 // SHA2_256("xx coin") [0:31] | 0x00
				(hex!["26c3ecba0b7cea7c131a6aedf4774f96216318a2ae74926cd0e01832a0b0b500"],
				 // Swap.transfer method
				 hex!["537761702e7472616e73666572"].iter().cloned().collect())
			],
			threshold: 1,
			balance: 100 * UNITS,
		},
		xx_cmix: phoenixx::XXCmixConfig {
			admin_permission: 0,
			cmix_address_space: 18,
			cmix_hashes: Default::default(),
			scheduling_account: get_account_id_from_seed::<sr25519::Public>("Alice"),
			cmix_variables: Default::default(),
		},
		xx_economics: phoenixx::XXEconomicsConfig {
			balance: 10 * UNITS,
			inflation_params: Default::default(),
			interest_points: vec![Default::default()],
			ideal_stake_rewards: 10 * UNITS,
			liquidity_rewards: 100 * UNITS,
		},
		xx_custody: phoenixx::XXCustodyConfig {
			team_allocations: vec![],
			custodians: vec![],
		},
		xx_betanet_rewards: Default::default(),
		xx_public: Default::default(),
		assets: Default::default(),
	}
}

fn phoenixx_development_config_genesis() -> phoenixx::GenesisConfig {
	phoenixx_testnet_genesis(
		vec![
			authority_keys_from_seed("Alice"),
		],
		vec![],
		None,
	)
}

/// `phoenixx` development config (single validator Alice)
pub fn phoenixx_development_config() -> PhoenixxChainSpec {
	PhoenixxChainSpec::from_genesis(
		"phoenixx Development",
		"phoenixx_dev",
		ChainType::Development,
		phoenixx_development_config_genesis,
		vec![],
		None,
		None,
		None,
		Default::default(),
	)
}

/// Helper function to create GenesisConfig for testing of the `protonet` network
pub fn protonet_testnet_genesis(
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		GrandpaId,
		BabeId,
		ImOnlineId,
		AuthorityDiscoveryId,
	)>,
	initial_nominators: Vec<AccountId>,
	endowed_accounts: Option<Vec<AccountId>>,
) -> protonet::GenesisConfig {
	let mut endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
		vec![
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			get_account_id_from_seed::<sr25519::Public>("Bob"),
			get_account_id_from_seed::<sr25519::Public>("Charlie"),
			get_account_id_from_seed::<sr25519::Public>("Dave"),
			get_account_id_from_seed::<sr25519::Public>("Eve"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie"),
			get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
			get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
			get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
			get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
			get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
		]
	});
	// endow all authorities and nominators.
	initial_authorities.iter().map(|x| &x.0).chain(initial_nominators.iter()).for_each(|x| {
		if !endowed_accounts.contains(&x) {
			endowed_accounts.push(x.clone())
		}
	});

	// stakers: all validators and nominators.
	let mut rng = rand::thread_rng();
	let stakers = initial_authorities
		.iter()
		.enumerate()
		.map(|(i, x)| (x.0.clone(), x.1.clone(), STASH, protonet::StakerStatus::Validator(Some(Hash::repeat_byte(i as u8)))))
		.chain(initial_nominators.iter().map(|x| {
			use rand::{seq::SliceRandom, Rng};
			let limit = (protonet::MAX_NOMINATIONS as usize).min(initial_authorities.len());
			let count = rng.gen::<usize>() % limit;
			let nominations = initial_authorities
				.as_slice()
				.choose_multiple(&mut rng, count)
				.into_iter()
				.map(|choice| choice.0.clone())
				.collect::<Vec<_>>();
			(x.clone(), x.clone(), STASH, protonet::StakerStatus::Nominator(nominations))
		}))
		.collect::<Vec<_>>();

	let num_endowed_accounts = endowed_accounts.len();

	const ENDOWMENT: Balance = 10_000_000 * UNITS;
	const STASH: Balance = ENDOWMENT / 1000;

	protonet::GenesisConfig {
		system: protonet::SystemConfig {
			code: protonet::wasm_binary_unwrap().to_vec(),
			changes_trie_config: Default::default(),
		},
		balances: protonet::BalancesConfig {
			balances: endowed_accounts.iter().cloned()
				.map(|x| (x, ENDOWMENT))
				.collect()
		},
		session: protonet::SessionConfig {
			keys: initial_authorities.iter().map(|x| {
				(x.0.clone(), x.0.clone(), protonet_session_keys(
					x.2.clone(),
					x.3.clone(),
					x.4.clone(),
					x.5.clone(),
				))
			}).collect::<Vec<_>>(),
		},
		staking: protonet::StakingConfig {
			validator_count: initial_authorities.len() as u32 * 2,
			minimum_validator_count: initial_authorities.len() as u32,
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			stakers,
			.. Default::default()
		},
		democracy: protonet::DemocracyConfig::default(),
		elections: protonet::ElectionsConfig {
			members: endowed_accounts.iter()
				.take((num_endowed_accounts + 1) / 2)
				.cloned()
				.map(|member| (member, STASH))
				.collect(),
		},
		council: protonet::CouncilConfig::default(),
		technical_committee: protonet::TechnicalCommitteeConfig {
			members: endowed_accounts.iter()
				.take((num_endowed_accounts + 1) / 2)
				.cloned()
				.collect(),
			phantom: Default::default(),
		},
		babe: protonet::BabeConfig {
			authorities: vec![],
			epoch_config: Some(xxnetwork_runtime::BABE_GENESIS_EPOCH_CONFIG),
		},
		im_online: protonet::ImOnlineConfig {
			keys: vec![],
		},
		authority_discovery: protonet::AuthorityDiscoveryConfig {
			keys: vec![],
		},
		grandpa: protonet::GrandpaConfig {
			authorities: vec![],
		},
		technical_membership: Default::default(),
		treasury: Default::default(),
		vesting: Default::default(),
		claims: Default::default(),
		swap: protonet::SwapConfig {
			swap_fee: 1 * UNITS,
			fee_destination: get_account_id_from_seed::<sr25519::Public>("Alice"),
			chains: vec![1],
			relayers: vec![get_account_id_from_seed::<sr25519::Public>("Alice")],
			resources: vec![
				// SHA2_256("xx coin") [0:31] | 0x00
				(hex!["26c3ecba0b7cea7c131a6aedf4774f96216318a2ae74926cd0e01832a0b0b500"],
				 // Swap.transfer method
				 hex!["537761702e7472616e73666572"].iter().cloned().collect())
			],
			threshold: 1,
			balance: 100 * UNITS,
		},
		xx_cmix: protonet::XXCmixConfig {
			admin_permission: 0,
			cmix_address_space: 18,
			cmix_hashes: Default::default(),
			scheduling_account: get_account_id_from_seed::<sr25519::Public>("Alice"),
			cmix_variables: Default::default(),
		},
		xx_economics: protonet::XXEconomicsConfig {
			balance: 10 * UNITS,
			inflation_params: Default::default(),
			interest_points: vec![Default::default()],
			ideal_stake_rewards: 10 * UNITS,
			liquidity_rewards: 100 * UNITS,
		},
		xx_custody: protonet::XXCustodyConfig {
			team_allocations: vec![],
			custodians: vec![],
		},
		xx_betanet_rewards: Default::default(),
		xx_public: Default::default(),
		assets: Default::default(),
	}
}

fn protonet_development_config_genesis() -> protonet::GenesisConfig {
	protonet_testnet_genesis(
		vec![
			authority_keys_from_seed("Alice"),
		],
		vec![],
		None,
	)
}

/// `protonet` development config (single validator Alice)
pub fn protonet_development_config() -> ProtonetChainSpec {
	ProtonetChainSpec::from_genesis(
		"xx protonet Development",
		"xxprotonet_dev",
		ChainType::Development,
		protonet_development_config_genesis,
		vec![],
		None,
		None,
		None,
		Default::default(),
	)
}

/// Helper function to create GenesisConfig for testing of `xxnetwork`
pub fn xxnetwork_testnet_genesis(
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		GrandpaId,
		BabeId,
		ImOnlineId,
		AuthorityDiscoveryId,
	)>,
	initial_nominators: Vec<AccountId>,
	endowed_accounts: Option<Vec<AccountId>>,
) -> xxnetwork::GenesisConfig {
	let mut endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
		vec![
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			get_account_id_from_seed::<sr25519::Public>("Bob"),
			get_account_id_from_seed::<sr25519::Public>("Charlie"),
			get_account_id_from_seed::<sr25519::Public>("Dave"),
			get_account_id_from_seed::<sr25519::Public>("Eve"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie"),
			get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
			get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
			get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
			get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
			get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
		]
	});
	// endow all authorities and nominators.
	initial_authorities.iter().map(|x| &x.0).chain(initial_nominators.iter()).for_each(|x| {
		if !endowed_accounts.contains(&x) {
			endowed_accounts.push(x.clone())
		}
	});

	// stakers: all validators and nominators.
	let mut rng = rand::thread_rng();
	let stakers = initial_authorities
		.iter()
		.enumerate()
		.map(|(i, x)| (x.0.clone(), x.1.clone(), STASH, xxnetwork::StakerStatus::Validator(Some(Hash::repeat_byte(i as u8)))))
		.chain(initial_nominators.iter().map(|x| {
			use rand::{seq::SliceRandom, Rng};
			let limit = (xxnetwork::MAX_NOMINATIONS as usize).min(initial_authorities.len());
			let count = rng.gen::<usize>() % limit;
			let nominations = initial_authorities
				.as_slice()
				.choose_multiple(&mut rng, count)
				.into_iter()
				.map(|choice| choice.0.clone())
				.collect::<Vec<_>>();
			(x.clone(), x.clone(), STASH, xxnetwork::StakerStatus::Nominator(nominations))
		}))
		.collect::<Vec<_>>();

	let num_endowed_accounts = endowed_accounts.len();

	const ENDOWMENT: Balance = 10_000_000 * UNITS;
	const STASH: Balance = ENDOWMENT / 1000;
	const TEAM_ALLOCATION: Balance = 10_000_000 * UNITS;

	xxnetwork::GenesisConfig {
		system: xxnetwork::SystemConfig {
			code: xxnetwork::wasm_binary_unwrap().to_vec(),
			changes_trie_config: Default::default(),
		},
		balances: xxnetwork::BalancesConfig {
			balances: endowed_accounts.iter().cloned()
				.map(|x| (x, ENDOWMENT))
				.collect()
		},
		session: xxnetwork::SessionConfig {
			keys: initial_authorities.iter().map(|x| {
				(x.0.clone(), x.0.clone(), xxnetwork_session_keys(
					x.2.clone(),
					x.3.clone(),
					x.4.clone(),
					x.5.clone(),
				))
			}).collect::<Vec<_>>(),
		},
		staking: xxnetwork::StakingConfig {
			validator_count: initial_authorities.len() as u32 * 2,
			minimum_validator_count: initial_authorities.len() as u32,
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			stakers,
			.. Default::default()
		},
		democracy: xxnetwork::DemocracyConfig::default(),
		elections: xxnetwork::ElectionsConfig {
			members: endowed_accounts.iter()
				.take((num_endowed_accounts + 1) / 2)
				.cloned()
				.map(|member| (member, STASH))
				.collect(),
		},
		council: xxnetwork::CouncilConfig::default(),
		technical_committee: xxnetwork::TechnicalCommitteeConfig {
			members: endowed_accounts.iter()
				.take((num_endowed_accounts + 1) / 2)
				.cloned()
				.collect(),
			phantom: Default::default(),
		},
		babe: xxnetwork::BabeConfig {
			authorities: vec![],
			epoch_config: Some(xxnetwork_runtime::BABE_GENESIS_EPOCH_CONFIG),
		},
		im_online: xxnetwork::ImOnlineConfig {
			keys: vec![],
		},
		authority_discovery: xxnetwork::AuthorityDiscoveryConfig {
			keys: vec![],
		},
		grandpa: xxnetwork::GrandpaConfig {
			authorities: vec![],
		},
		technical_membership: Default::default(),
		treasury: Default::default(),
		vesting: Default::default(),
		claims: Default::default(),
		swap: xxnetwork::SwapConfig {
			swap_fee: 1 * UNITS,
			fee_destination: get_account_id_from_seed::<sr25519::Public>("Alice"),
			chains: vec![1],
			relayers: vec![get_account_id_from_seed::<sr25519::Public>("Alice")],
			resources: vec![
				// SHA2_256("xx coin") [0:31] | 0x00
				(hex!["26c3ecba0b7cea7c131a6aedf4774f96216318a2ae74926cd0e01832a0b0b500"],
				 // Swap.transfer method
				 hex!["537761702e7472616e73666572"].iter().cloned().collect())
			],
			threshold: 1,
			balance: 100 * UNITS,
		},
		xx_cmix: xxnetwork::XXCmixConfig {
			admin_permission: 0,
			cmix_address_space: 18,
			cmix_hashes: Default::default(),
			scheduling_account: get_account_id_from_seed::<sr25519::Public>("Alice"),
			cmix_variables: Default::default(),
		},
		xx_economics: xxnetwork::XXEconomicsConfig {
			balance: 10 * UNITS,
			inflation_params: Default::default(),
			interest_points: vec![Default::default()],
			ideal_stake_rewards: 10 * UNITS,
			liquidity_rewards: 100 * UNITS,
		},
		xx_custody: xxnetwork::XXCustodyConfig {
			team_allocations: vec![
				(get_account_id_from_seed::<sr25519::Public>("Alice"), TEAM_ALLOCATION),
				(get_account_id_from_seed::<sr25519::Public>("Bob"), TEAM_ALLOCATION),
			],
			custodians: vec![
				(get_account_id_from_seed::<sr25519::Public>("Charlie"), ())
			],
		},
		xx_betanet_rewards: Default::default(),
		xx_public: Default::default(),
		assets: Default::default(),
	}
}

fn xxnetwork_development_config_genesis() -> xxnetwork::GenesisConfig {
	xxnetwork_testnet_genesis(
		vec![
			authority_keys_from_seed("Alice"),
		],
		vec![],
		None,
	)
}

/// `xxnetwork` development config (single validator Alice)
pub fn xxnetwork_development_config() -> XXNetworkChainSpec {
	XXNetworkChainSpec::from_genesis(
		"xx network Development",
		"xxnetwork_dev",
		ChainType::Development,
		xxnetwork_development_config_genesis,
		vec![],
		None,
		None,
		None,
		Default::default(),
	)
}
