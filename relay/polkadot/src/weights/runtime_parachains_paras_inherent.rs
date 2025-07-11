// Copyright (C) Parity Technologies and the various Polkadot contributors, see Contributions.md
// for a list of specific contributors.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for `runtime_parachains :: paras_inherent`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 47.2.0
//! DATE: 2025-06-16, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ggwpez-ref-hw`, CPU: `AMD EPYC 7232P 8-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: 1024

// Executed Command:
// frame-omni-bencher
// v1
// benchmark
// pallet
// --runtime=target/production/wbuild/polkadot-runtime/polkadot_runtime.compact.compressed.wasm
// --header=.github/scripts/cmd/file_header.txt
// --output=./relay/polkadot/src/weights/
// --all
// --quiet

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `runtime_parachains :: paras_inherent`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> runtime_parachains :: paras_inherent::WeightInfo for WeightInfo<T> {
	/// Storage: `ParaInherent::Included` (r:1 w:1)
	/// Proof: `ParaInherent::Included` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `System::ParentHash` (r:1 w:0)
	/// Proof: `System::ParentHash` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `ParasShared::AllowedRelayParents` (r:1 w:1)
	/// Proof: `ParasShared::AllowedRelayParents` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParaScheduler::ClaimQueue` (r:1 w:0)
	/// Proof: `ParaScheduler::ClaimQueue` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParasShared::CurrentSessionIndex` (r:1 w:0)
	/// Proof: `ParasShared::CurrentSessionIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParaScheduler::ValidatorGroups` (r:1 w:0)
	/// Proof: `ParaScheduler::ValidatorGroups` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParasShared::ActiveValidatorKeys` (r:1 w:0)
	/// Proof: `ParasShared::ActiveValidatorKeys` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Babe::AuthorVrfRandomness` (r:1 w:0)
	/// Proof: `Babe::AuthorVrfRandomness` (`max_values`: Some(1), `max_size`: Some(33), added: 528, mode: `MaxEncodedLen`)
	/// Storage: `ParaInherent::OnChainVotes` (r:1 w:1)
	/// Proof: `ParaInherent::OnChainVotes` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParasDisputes::Frozen` (r:1 w:0)
	/// Proof: `ParasDisputes::Frozen` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParaInclusion::V1` (r:1 w:0)
	/// Proof: `ParaInclusion::V1` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParaScheduler::SessionStartBlock` (r:1 w:0)
	/// Proof: `ParaScheduler::SessionStartBlock` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Initializer::BufferedSessionChanges` (r:1 w:0)
	/// Proof: `Initializer::BufferedSessionChanges` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParasShared::ActiveValidatorIndices` (r:1 w:0)
	/// Proof: `ParasShared::ActiveValidatorIndices` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Session::DisabledValidators` (r:1 w:0)
	/// Proof: `Session::DisabledValidators` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn enter_empty() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `37563`
		//  Estimated: `41028`
		// Minimum execution time: 211_430_000 picoseconds.
		Weight::from_parts(223_330_000, 0)
			.saturating_add(Weight::from_parts(0, 41028))
			.saturating_add(T::DbWeight::get().reads(15))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `ParaInherent::Included` (r:1 w:1)
	/// Proof: `ParaInherent::Included` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `System::ParentHash` (r:1 w:0)
	/// Proof: `System::ParentHash` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `ParasShared::AllowedRelayParents` (r:1 w:1)
	/// Proof: `ParasShared::AllowedRelayParents` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParaScheduler::ClaimQueue` (r:1 w:1)
	/// Proof: `ParaScheduler::ClaimQueue` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParasShared::CurrentSessionIndex` (r:1 w:0)
	/// Proof: `ParasShared::CurrentSessionIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParaScheduler::ValidatorGroups` (r:1 w:0)
	/// Proof: `ParaScheduler::ValidatorGroups` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParasShared::ActiveValidatorKeys` (r:1 w:0)
	/// Proof: `ParasShared::ActiveValidatorKeys` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Babe::AuthorVrfRandomness` (r:1 w:0)
	/// Proof: `Babe::AuthorVrfRandomness` (`max_values`: Some(1), `max_size`: Some(33), added: 528, mode: `MaxEncodedLen`)
	/// Storage: `ParaSessionInfo::Sessions` (r:1 w:0)
	/// Proof: `ParaSessionInfo::Sessions` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParasDisputes::Disputes` (r:1 w:1)
	/// Proof: `ParasDisputes::Disputes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParasDisputes::BackersOnDisputes` (r:1 w:1)
	/// Proof: `ParasDisputes::BackersOnDisputes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParasDisputes::Included` (r:1 w:1)
	/// Proof: `ParasDisputes::Included` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParaSessionInfo::AccountKeys` (r:1 w:0)
	/// Proof: `ParaSessionInfo::AccountKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Session::Validators` (r:1 w:0)
	/// Proof: `Session::Validators` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Staking::ActiveEra` (r:1 w:0)
	/// Proof: `Staking::ActiveEra` (`max_values`: Some(1), `max_size`: Some(13), added: 508, mode: `MaxEncodedLen`)
	/// Storage: `Staking::ErasRewardPoints` (r:1 w:1)
	/// Proof: `Staking::ErasRewardPoints` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParaInherent::OnChainVotes` (r:1 w:1)
	/// Proof: `ParaInherent::OnChainVotes` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParasDisputes::Frozen` (r:1 w:0)
	/// Proof: `ParasDisputes::Frozen` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParaInclusion::V1` (r:2 w:1)
	/// Proof: `ParaInclusion::V1` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DownwardMessageQueues` (r:1 w:1)
	/// Proof: `Dmp::DownwardMessageQueues` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DeliveryFeeFactor` (r:1 w:1)
	/// Proof: `Dmp::DeliveryFeeFactor` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpChannelDigests` (r:1 w:1)
	/// Proof: `Hrmp::HrmpChannelDigests` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::FutureCodeUpgrades` (r:1 w:0)
	/// Proof: `Paras::FutureCodeUpgrades` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::Paras` (r:1 w:0)
	/// Proof: `Registrar::Paras` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParaScheduler::SessionStartBlock` (r:1 w:0)
	/// Proof: `ParaScheduler::SessionStartBlock` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Initializer::BufferedSessionChanges` (r:1 w:0)
	/// Proof: `Initializer::BufferedSessionChanges` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParasShared::ActiveValidatorIndices` (r:1 w:0)
	/// Proof: `ParasShared::ActiveValidatorIndices` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Session::DisabledValidators` (r:1 w:0)
	/// Proof: `Session::DisabledValidators` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `CoretimeAssignmentProvider::CoreDescriptors` (r:1 w:1)
	/// Proof: `CoretimeAssignmentProvider::CoreDescriptors` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpWatermarks` (r:0 w:1)
	/// Proof: `Hrmp::HrmpWatermarks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::Heads` (r:0 w:1)
	/// Proof: `Paras::Heads` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::UpgradeGoAheadSignal` (r:0 w:1)
	/// Proof: `Paras::UpgradeGoAheadSignal` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::MostRecentContext` (r:0 w:1)
	/// Proof: `Paras::MostRecentContext` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `v` is `[400, 1024]`.
	fn enter_variable_disputes(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `117466`
		//  Estimated: `123406`
		// Minimum execution time: 22_594_028_000 picoseconds.
		Weight::from_parts(696_843_916, 0)
			.saturating_add(Weight::from_parts(0, 123406))
			// Standard Error: 8_758
			.saturating_add(Weight::from_parts(54_802_336, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(30))
			.saturating_add(T::DbWeight::get().writes(17))
	}
	/// Storage: `ParaInherent::Included` (r:1 w:1)
	/// Proof: `ParaInherent::Included` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `System::ParentHash` (r:1 w:0)
	/// Proof: `System::ParentHash` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `ParasShared::AllowedRelayParents` (r:1 w:1)
	/// Proof: `ParasShared::AllowedRelayParents` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParaScheduler::ClaimQueue` (r:1 w:0)
	/// Proof: `ParaScheduler::ClaimQueue` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParasShared::CurrentSessionIndex` (r:1 w:0)
	/// Proof: `ParasShared::CurrentSessionIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParaScheduler::ValidatorGroups` (r:1 w:0)
	/// Proof: `ParaScheduler::ValidatorGroups` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParasShared::ActiveValidatorKeys` (r:1 w:0)
	/// Proof: `ParasShared::ActiveValidatorKeys` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Babe::AuthorVrfRandomness` (r:1 w:0)
	/// Proof: `Babe::AuthorVrfRandomness` (`max_values`: Some(1), `max_size`: Some(33), added: 528, mode: `MaxEncodedLen`)
	/// Storage: `ParaInherent::OnChainVotes` (r:1 w:1)
	/// Proof: `ParaInherent::OnChainVotes` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParasDisputes::Frozen` (r:1 w:0)
	/// Proof: `ParasDisputes::Frozen` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParaInclusion::V1` (r:2 w:1)
	/// Proof: `ParaInclusion::V1` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParaScheduler::SessionStartBlock` (r:1 w:0)
	/// Proof: `ParaScheduler::SessionStartBlock` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Initializer::BufferedSessionChanges` (r:1 w:0)
	/// Proof: `Initializer::BufferedSessionChanges` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParasShared::ActiveValidatorIndices` (r:1 w:0)
	/// Proof: `ParasShared::ActiveValidatorIndices` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Session::DisabledValidators` (r:1 w:0)
	/// Proof: `Session::DisabledValidators` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn enter_bitfields() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `74971`
		//  Estimated: `80911`
		// Minimum execution time: 521_861_000 picoseconds.
		Weight::from_parts(537_981_000, 0)
			.saturating_add(Weight::from_parts(0, 80911))
			.saturating_add(T::DbWeight::get().reads(16))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `ParaInherent::Included` (r:1 w:1)
	/// Proof: `ParaInherent::Included` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `System::ParentHash` (r:1 w:0)
	/// Proof: `System::ParentHash` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `ParasShared::AllowedRelayParents` (r:1 w:1)
	/// Proof: `ParasShared::AllowedRelayParents` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParaScheduler::ClaimQueue` (r:1 w:1)
	/// Proof: `ParaScheduler::ClaimQueue` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParasShared::CurrentSessionIndex` (r:1 w:0)
	/// Proof: `ParasShared::CurrentSessionIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParaScheduler::ValidatorGroups` (r:1 w:0)
	/// Proof: `ParaScheduler::ValidatorGroups` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParasShared::ActiveValidatorKeys` (r:1 w:0)
	/// Proof: `ParasShared::ActiveValidatorKeys` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Babe::AuthorVrfRandomness` (r:1 w:0)
	/// Proof: `Babe::AuthorVrfRandomness` (`max_values`: Some(1), `max_size`: Some(33), added: 528, mode: `MaxEncodedLen`)
	/// Storage: `ParaInherent::OnChainVotes` (r:1 w:1)
	/// Proof: `ParaInherent::OnChainVotes` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParasDisputes::Frozen` (r:1 w:0)
	/// Proof: `ParasDisputes::Frozen` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParaInclusion::V1` (r:2 w:1)
	/// Proof: `ParaInclusion::V1` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParaSessionInfo::AccountKeys` (r:1 w:0)
	/// Proof: `ParaSessionInfo::AccountKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Session::Validators` (r:1 w:0)
	/// Proof: `Session::Validators` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Staking::ActiveEra` (r:1 w:0)
	/// Proof: `Staking::ActiveEra` (`max_values`: Some(1), `max_size`: Some(13), added: 508, mode: `MaxEncodedLen`)
	/// Storage: `Staking::ErasRewardPoints` (r:1 w:1)
	/// Proof: `Staking::ErasRewardPoints` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DownwardMessageQueues` (r:1 w:1)
	/// Proof: `Dmp::DownwardMessageQueues` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DeliveryFeeFactor` (r:1 w:1)
	/// Proof: `Dmp::DeliveryFeeFactor` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpChannelDigests` (r:1 w:1)
	/// Proof: `Hrmp::HrmpChannelDigests` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::FutureCodeUpgrades` (r:1 w:0)
	/// Proof: `Paras::FutureCodeUpgrades` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::Paras` (r:1 w:0)
	/// Proof: `Registrar::Paras` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParasDisputes::Disputes` (r:1 w:0)
	/// Proof: `ParasDisputes::Disputes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParaScheduler::SessionStartBlock` (r:1 w:0)
	/// Proof: `ParaScheduler::SessionStartBlock` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Initializer::BufferedSessionChanges` (r:1 w:0)
	/// Proof: `Initializer::BufferedSessionChanges` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::CurrentCodeHash` (r:1 w:0)
	/// Proof: `Paras::CurrentCodeHash` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::ParaLifecycles` (r:1 w:0)
	/// Proof: `Paras::ParaLifecycles` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `MessageQueue::BookStateFor` (r:1 w:0)
	/// Proof: `MessageQueue::BookStateFor` (`max_values`: None, `max_size`: Some(55), added: 2530, mode: `MaxEncodedLen`)
	/// Storage: `ParasShared::ActiveValidatorIndices` (r:1 w:0)
	/// Proof: `ParasShared::ActiveValidatorIndices` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Session::DisabledValidators` (r:1 w:0)
	/// Proof: `Session::DisabledValidators` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `CoretimeAssignmentProvider::CoreDescriptors` (r:1 w:1)
	/// Proof: `CoretimeAssignmentProvider::CoreDescriptors` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParasDisputes::Included` (r:0 w:1)
	/// Proof: `ParasDisputes::Included` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpWatermarks` (r:0 w:1)
	/// Proof: `Hrmp::HrmpWatermarks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::Heads` (r:0 w:1)
	/// Proof: `Paras::Heads` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::UpgradeGoAheadSignal` (r:0 w:1)
	/// Proof: `Paras::UpgradeGoAheadSignal` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::MostRecentContext` (r:0 w:1)
	/// Proof: `Paras::MostRecentContext` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `v` is `[2, 5]`.
	fn enter_backed_candidates_variable(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76410`
		//  Estimated: `82350`
		// Minimum execution time: 1_697_997_000 picoseconds.
		Weight::from_parts(1_635_000_527, 0)
			.saturating_add(Weight::from_parts(0, 82350))
			// Standard Error: 974_235
			.saturating_add(Weight::from_parts(54_129_674, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(30))
			.saturating_add(T::DbWeight::get().writes(15))
	}
	/// Storage: `ParaInherent::Included` (r:1 w:1)
	/// Proof: `ParaInherent::Included` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `System::ParentHash` (r:1 w:0)
	/// Proof: `System::ParentHash` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `ParasShared::AllowedRelayParents` (r:1 w:1)
	/// Proof: `ParasShared::AllowedRelayParents` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParaScheduler::ClaimQueue` (r:1 w:1)
	/// Proof: `ParaScheduler::ClaimQueue` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParasShared::CurrentSessionIndex` (r:1 w:0)
	/// Proof: `ParasShared::CurrentSessionIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParaScheduler::ValidatorGroups` (r:1 w:0)
	/// Proof: `ParaScheduler::ValidatorGroups` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParasShared::ActiveValidatorKeys` (r:1 w:0)
	/// Proof: `ParasShared::ActiveValidatorKeys` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Babe::AuthorVrfRandomness` (r:1 w:0)
	/// Proof: `Babe::AuthorVrfRandomness` (`max_values`: Some(1), `max_size`: Some(33), added: 528, mode: `MaxEncodedLen`)
	/// Storage: `ParaInherent::OnChainVotes` (r:1 w:1)
	/// Proof: `ParaInherent::OnChainVotes` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParasDisputes::Frozen` (r:1 w:0)
	/// Proof: `ParasDisputes::Frozen` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParaInclusion::V1` (r:2 w:1)
	/// Proof: `ParaInclusion::V1` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParaSessionInfo::AccountKeys` (r:1 w:0)
	/// Proof: `ParaSessionInfo::AccountKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Session::Validators` (r:1 w:0)
	/// Proof: `Session::Validators` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Staking::ActiveEra` (r:1 w:0)
	/// Proof: `Staking::ActiveEra` (`max_values`: Some(1), `max_size`: Some(13), added: 508, mode: `MaxEncodedLen`)
	/// Storage: `Staking::ErasRewardPoints` (r:1 w:1)
	/// Proof: `Staking::ErasRewardPoints` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DownwardMessageQueues` (r:1 w:1)
	/// Proof: `Dmp::DownwardMessageQueues` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DeliveryFeeFactor` (r:1 w:1)
	/// Proof: `Dmp::DeliveryFeeFactor` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpChannelDigests` (r:1 w:1)
	/// Proof: `Hrmp::HrmpChannelDigests` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::FutureCodeUpgrades` (r:1 w:0)
	/// Proof: `Paras::FutureCodeUpgrades` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::Paras` (r:1 w:0)
	/// Proof: `Registrar::Paras` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParasDisputes::Disputes` (r:1 w:0)
	/// Proof: `ParasDisputes::Disputes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParaScheduler::SessionStartBlock` (r:1 w:0)
	/// Proof: `ParaScheduler::SessionStartBlock` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Initializer::BufferedSessionChanges` (r:1 w:0)
	/// Proof: `Initializer::BufferedSessionChanges` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::CurrentCodeHash` (r:1 w:0)
	/// Proof: `Paras::CurrentCodeHash` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::FutureCodeHash` (r:1 w:0)
	/// Proof: `Paras::FutureCodeHash` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::UpgradeRestrictionSignal` (r:1 w:0)
	/// Proof: `Paras::UpgradeRestrictionSignal` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::ParaLifecycles` (r:1 w:0)
	/// Proof: `Paras::ParaLifecycles` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `MessageQueue::BookStateFor` (r:1 w:0)
	/// Proof: `MessageQueue::BookStateFor` (`max_values`: None, `max_size`: Some(55), added: 2530, mode: `MaxEncodedLen`)
	/// Storage: `ParasShared::ActiveValidatorIndices` (r:1 w:0)
	/// Proof: `ParasShared::ActiveValidatorIndices` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Session::DisabledValidators` (r:1 w:0)
	/// Proof: `Session::DisabledValidators` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `CoretimeAssignmentProvider::CoreDescriptors` (r:1 w:1)
	/// Proof: `CoretimeAssignmentProvider::CoreDescriptors` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParasDisputes::Included` (r:0 w:1)
	/// Proof: `ParasDisputes::Included` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpWatermarks` (r:0 w:1)
	/// Proof: `Hrmp::HrmpWatermarks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::Heads` (r:0 w:1)
	/// Proof: `Paras::Heads` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::UpgradeGoAheadSignal` (r:0 w:1)
	/// Proof: `Paras::UpgradeGoAheadSignal` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::MostRecentContext` (r:0 w:1)
	/// Proof: `Paras::MostRecentContext` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn enter_backed_candidate_code_upgrade() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76423`
		//  Estimated: `82363`
		// Minimum execution time: 47_923_393_000 picoseconds.
		Weight::from_parts(49_723_235_000, 0)
			.saturating_add(Weight::from_parts(0, 82363))
			.saturating_add(T::DbWeight::get().reads(32))
			.saturating_add(T::DbWeight::get().writes(15))
	}
}
