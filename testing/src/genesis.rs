// This file is part of Substrate.

// Copyright (C) 2019-2021 Parity Technologies (UK) Ltd.
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

//! Genesis Configuration.

use crate::keyring::*;
use sp_keyring::{Ed25519Keyring, Sr25519Keyring};
use xxnetwork_runtime::{
	GenesisConfig, BalancesConfig, SessionConfig, StakingConfig, SystemConfig,
	GrandpaConfig, SwapConfig, wasm_binary_unwrap,
	AccountId, StakerStatus, BabeConfig, BABE_GENESIS_EPOCH_CONFIG,
};
use xxnetwork_runtime::constants::currency::*;
use sp_core::ChangesTrieConfiguration;
use sp_runtime::Perbill;

/// Create genesis runtime configuration for tests.
pub fn config(support_changes_trie: bool, code: Option<&[u8]>) -> GenesisConfig {
	config_endowed(support_changes_trie, code, Default::default())
}

/// Create genesis runtime configuration for tests with some extra
/// endowed accounts.
pub fn config_endowed(
	support_changes_trie: bool,
	code: Option<&[u8]>,
	extra_endowed: Vec<AccountId>,
) -> GenesisConfig {

	let mut endowed = vec![
		(alice(), 111 * UNITS),
		(bob(), 100 * UNITS),
		(charlie(), 100_000_000 * UNITS),
		(dave(), 111 * UNITS),
		(eve(), 101 * UNITS),
		(ferdie(), 100 * UNITS),
	];

	endowed.extend(
		extra_endowed.into_iter().map(|endowed| (endowed, 100*UNITS))
	);

	GenesisConfig {
		system: SystemConfig {
			changes_trie_config: if support_changes_trie { Some(ChangesTrieConfiguration {
				digest_interval: 2,
				digest_levels: 2,
			}) } else { None },
			code: code.map(|x| x.to_vec()).unwrap_or_else(|| wasm_binary_unwrap().to_vec()),
		},
		babe: BabeConfig {
			authorities: vec![],
			epoch_config: Some(BABE_GENESIS_EPOCH_CONFIG),
		},
		balances: BalancesConfig {
			balances: endowed,
		},
		staking: StakingConfig {
			stakers: vec![
				(dave(), alice(), 111 * UNITS, StakerStatus::Validator(Default::default())),
				(eve(), bob(), 100 * UNITS, StakerStatus::Validator(Default::default())),
				(ferdie(), charlie(), 100 * UNITS, StakerStatus::Validator(Default::default()))
			],
			validator_count: 3,
			minimum_validator_count: 0,
			slash_reward_fraction: Perbill::from_percent(10),
			invulnerables: vec![alice(), bob(), charlie()],
			.. Default::default()
		},
		session: SessionConfig {
			keys: vec![
				(dave(), alice(), to_session_keys(
					&Ed25519Keyring::Alice,
					&Sr25519Keyring::Alice,
				)),
				(eve(), bob(), to_session_keys(
					&Ed25519Keyring::Bob,
					&Sr25519Keyring::Bob,
				)),
				(ferdie(), charlie(), to_session_keys(
					&Ed25519Keyring::Charlie,
					&Sr25519Keyring::Charlie,
				)),
			]
		},
		grandpa: GrandpaConfig {
			authorities: vec![],
		},
		im_online: Default::default(),
		authority_discovery: Default::default(),
		democracy: Default::default(),
		council: Default::default(),
		technical_committee: Default::default(),
		elections: Default::default(),
		technical_membership: Default::default(),
		treasury: Default::default(),
		claims: Default::default(),
		vesting: Default::default(),
		swap: SwapConfig {
			threshold: 1,
			..Default::default()
		},
		xx_cmix: Default::default(),
		xx_economics: Default::default(),
		xx_custody: Default::default(),
	}
}
