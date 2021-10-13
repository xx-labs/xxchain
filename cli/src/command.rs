// This file is part of Substrate.

// Copyright (C) 2017-2021 Parity Technologies (UK) Ltd.
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

use crate::{chain_spec, service, Cli, Subcommand};
use sc_cli::{Result, SubstrateCli, RuntimeVersion, Role, ChainSpec};
use sc_service::PartialComponents;
use crate::service::{new_partial, new_partial_protonet, new_partial_phoenixx};
use crate::chain_spec::IdentifyVariant;
use node_executor::{XXNetworkExecutorDispatch, ProtonetExecutorDispatch, PhoenixxExecutorDispatch};

impl SubstrateCli for Cli {
	fn impl_name() -> String {
		"xxlabs xxnetwork".into()
	}

	fn impl_version() -> String {
		env!("SUBSTRATE_CLI_IMPL_VERSION").into()
	}

	fn description() -> String {
		env!("CARGO_PKG_DESCRIPTION").into()
	}

	fn author() -> String {
		env!("CARGO_PKG_AUTHORS").into()
	}

	fn support_url() -> String {
		"https://github.com/xx-labs/xxchain/issues/new".into()
	}

	fn copyright_start_year() -> i32 {
		2020
	}

	fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
		let id = if id == "" {
			// For now default to protonet testnet if no chain provided
			"protonet"
		} else { id };
		Ok(match id {
			"xxnetwork" => Box::new(chain_spec::xxnetwork_config()?),
			"xxnetwork-dev" => Box::new(chain_spec::xxnetwork_development_config()),
			"protonet" => Box::new(chain_spec::protonet_config()?),
			"protonet-dev" => Box::new(chain_spec::protonet_development_config()),
			"phoenixx" => Box::new(chain_spec::phoenixx_config()?),
			"phoenixx-dev" | "dev" => Box::new(chain_spec::phoenixx_development_config()),
			path => {
				let path = std::path::PathBuf::from(path);

				let chain_spec = Box::new(chain_spec::PhoenixxChainSpec::from_json_file(path.clone())?) as Box<dyn sc_chain_spec::ChainSpec>;

				// When the file name starts with the name of one of the known chains,
				// we use the chain spec for the specific chain.
				if chain_spec.is_xxnetwork() {
					Box::new(chain_spec::XXNetworkChainSpec::from_json_file(path)?)
				} else if chain_spec.is_protonet() {
					Box::new(chain_spec::ProtonetChainSpec::from_json_file(path)?)
				} else {
					chain_spec
				}
			},
		})
	}

	fn native_runtime_version(spec: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
		if spec.is_phoenixx() {
			return &chain_spec::phoenixx::VERSION
		}

		if spec.is_protonet() {
			return &chain_spec::protonet::VERSION
		}

		&chain_spec::xxnetwork::VERSION
	}
}

/// Parse command line arguments into service configuration.
pub fn run() -> Result<()> {
	let cli = Cli::from_args();

	match &cli.subcommand {
		None => {
			let runner = cli.create_runner(&cli.run)?;
			runner.run_node_until_exit(|config| async move {
				match config.role {
					Role::Light => service::new_light(config),
					_ => service::new_full(config),
				}.map_err(sc_cli::Error::Service)
			})
		}
		Some(Subcommand::Inspect(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;
			if chain_spec.is_phoenixx() {
				return runner.sync_run(|config|
					cmd.run::<
						chain_spec::phoenixx::Block,
						chain_spec::phoenixx::RuntimeApi,
						PhoenixxExecutorDispatch>(config)
				)
			}
			if chain_spec.is_protonet() {
				return runner.sync_run(|config|
					cmd.run::<
						chain_spec::protonet::Block,
						chain_spec::protonet::RuntimeApi,
						ProtonetExecutorDispatch>(config)
				)
			}
			runner.sync_run(|config| cmd.run::<
				chain_spec::xxnetwork::Block,
				chain_spec::xxnetwork::RuntimeApi,
				XXNetworkExecutorDispatch>(config))
		}
		Some(Subcommand::Benchmark(cmd)) => {
			if cfg!(feature = "runtime-benchmarks") {
				let runner = cli.create_runner(cmd)?;
				let chain_spec = &runner.config().chain_spec;
				if chain_spec.is_phoenixx() {
					return runner.sync_run(|config|
						cmd.run::<chain_spec::phoenixx::Block, PhoenixxExecutorDispatch>(config))
				}
				if chain_spec.is_protonet() {
					return runner.sync_run(|config|
						cmd.run::<chain_spec::protonet::Block, ProtonetExecutorDispatch>(config))
				}
				runner.sync_run(|config|
					cmd.run::<chain_spec::xxnetwork::Block, XXNetworkExecutorDispatch>(config))
			} else {
				Err("Benchmarking wasn't enabled when building the node. \
				You can enable it with `--features runtime-benchmarks`.".into())
			}
		}
		Some(Subcommand::Key(cmd)) => cmd.run(&cli),
		Some(Subcommand::Sign(cmd)) => cmd.run(),
		Some(Subcommand::Verify(cmd)) => cmd.run(),
		Some(Subcommand::Vanity(cmd)) => cmd.run(),
		Some(Subcommand::BuildSpec(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
		},
		Some(Subcommand::CheckBlock(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;
			if chain_spec.is_phoenixx() {
				return runner.async_run(|config| {
					let PartialComponents { client, task_manager, import_queue, ..}
						= new_partial_phoenixx(&config)?;
					Ok((cmd.run(client, import_queue), task_manager))
				})
			}
			if chain_spec.is_protonet() {
				return runner.async_run(|config| {
					let PartialComponents { client, task_manager, import_queue, ..}
						= new_partial_protonet(&config)?;
					Ok((cmd.run(client, import_queue), task_manager))
				})
			}
			runner.async_run(|config| {
				let PartialComponents { client, task_manager, import_queue, ..}
					= new_partial(&config)?;
				Ok((cmd.run(client, import_queue), task_manager))
			})
		},
		Some(Subcommand::ExportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;
			if chain_spec.is_phoenixx() {
				return runner.async_run(|config| {
					let PartialComponents { client, task_manager, ..}
						= new_partial_phoenixx(&config)?;
					Ok((cmd.run(client, config.database), task_manager))
				})
			}
			if chain_spec.is_protonet() {
				return runner.async_run(|config| {
					let PartialComponents { client, task_manager, ..}
						= new_partial_protonet(&config)?;
					Ok((cmd.run(client, config.database), task_manager))
				})
			}
			runner.async_run(|config| {
				let PartialComponents { client, task_manager, ..}
					= new_partial(&config)?;
				Ok((cmd.run(client, config.database), task_manager))
			})
		},
		Some(Subcommand::ExportState(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;
			if chain_spec.is_phoenixx() {
				return runner.async_run(|config| {
					let PartialComponents { client, task_manager, ..}
						= new_partial_phoenixx(&config)?;
					Ok((cmd.run(client, config.chain_spec), task_manager))
				})
			}
			if chain_spec.is_protonet() {
				return runner.async_run(|config| {
					let PartialComponents { client, task_manager, ..}
						= new_partial_protonet(&config)?;
					Ok((cmd.run(client, config.chain_spec), task_manager))
				})
			}
			runner.async_run(|config| {
				let PartialComponents { client, task_manager, ..}
					= new_partial(&config)?;
				Ok((cmd.run(client, config.chain_spec), task_manager))
			})
		},
		Some(Subcommand::ImportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;
			if chain_spec.is_phoenixx() {
				return runner.async_run(|config| {
					let PartialComponents { client, task_manager, import_queue, ..}
						= new_partial_phoenixx(&config)?;
					Ok((cmd.run(client, import_queue), task_manager))
				})
			}
			if chain_spec.is_protonet() {
				return runner.async_run(|config| {
					let PartialComponents { client, task_manager, import_queue, ..}
						= new_partial_protonet(&config)?;
					Ok((cmd.run(client, import_queue), task_manager))
				})
			}
			runner.async_run(|config| {
				let PartialComponents { client, task_manager, import_queue, ..}
					= new_partial(&config)?;
				Ok((cmd.run(client, import_queue), task_manager))
			})
		},
		Some(Subcommand::PurgeChain(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.database))
		},
		Some(Subcommand::Revert(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;
			if chain_spec.is_phoenixx() {
				return runner.async_run(|config| {
					let PartialComponents { client, task_manager, backend, ..}
						= new_partial_phoenixx(&config)?;
					Ok((cmd.run(client, backend), task_manager))
				})
			}
			if chain_spec.is_protonet() {
				return runner.async_run(|config| {
					let PartialComponents { client, task_manager, backend, ..}
						= new_partial_protonet(&config)?;
					Ok((cmd.run(client, backend), task_manager))
				})
			}
			runner.async_run(|config| {
				let PartialComponents { client, task_manager, backend, ..}
					= new_partial(&config)?;
				Ok((cmd.run(client, backend), task_manager))
			})
		},
		#[cfg(feature = "try-runtime")]
		Some(Subcommand::TryRuntime(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;
			runner.async_run(|config| {
				// we don't need any of the components of new_partial, just a runtime, or a task
				// manager to do `async_run`.
				let registry = config.prometheus_config.as_ref().map(|cfg| &cfg.registry);
				let task_manager = sc_service::TaskManager::new(
					config.task_executor.clone(),
					registry,
				).map_err(|e| sc_cli::Error::Service(sc_service::Error::Prometheus(e)))?;
				if chain_spec.is_phoenixx() {
					return Ok((cmd.run::<chain_spec::phoenixx::Block, PhoenixxExecutorDispatch>(config), task_manager))
				}
				if chain_spec.is_protonet() {
					return Ok((cmd.run::<chain_spec::protonet::Block, ProtonetExecutorDispatch>(config), task_manager))
				}
				Ok((cmd.run::<chain_spec::xxnetwork::Block, XXNetworkExecutorDispatch>(config), task_manager))
			})
		}
	}
}
