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

//! Autogenerated weights for `pallet_broker`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2025-01-08, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ggwpez-ref-hw`, CPU: `AMD EPYC 7232P 8-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("./coretime-polkadot-chain-spec.json")`, DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot-parachain
// benchmark
// pallet
// --chain=./coretime-polkadot-chain-spec.json
// --steps=50
// --repeat=20
// --pallet=pallet_broker
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./coretime-polkadot-weights/
// --header=./file_header.txt

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_broker`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_broker::WeightInfo for WeightInfo<T> {
	/// Storage: `Broker::Configuration` (r:0 w:1)
	/// Proof: `Broker::Configuration` (`max_values`: Some(1), `max_size`: Some(31), added: 526, mode: `MaxEncodedLen`)
	fn configure() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_270_000 picoseconds.
		Weight::from_parts(3_440_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Broker::Reservations` (r:1 w:1)
	/// Proof: `Broker::Reservations` (`max_values`: Some(1), `max_size`: Some(12021), added: 12516, mode: `MaxEncodedLen`)
	fn reserve() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `10888`
		//  Estimated: `13506`
		// Minimum execution time: 31_941_000 picoseconds.
		Weight::from_parts(32_480_000, 0)
			.saturating_add(Weight::from_parts(0, 13506))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Broker::Reservations` (r:1 w:1)
	/// Proof: `Broker::Reservations` (`max_values`: Some(1), `max_size`: Some(12021), added: 12516, mode: `MaxEncodedLen`)
	fn unreserve() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `12090`
		//  Estimated: `13506`
		// Minimum execution time: 30_910_000 picoseconds.
		Weight::from_parts(31_250_000, 0)
			.saturating_add(Weight::from_parts(0, 13506))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Broker::Leases` (r:1 w:1)
	/// Proof: `Broker::Leases` (`max_values`: Some(1), `max_size`: Some(441), added: 936, mode: `MaxEncodedLen`)
	/// Storage: `ParachainSystem::ValidationData` (r:1 w:0)
	/// Proof: `ParachainSystem::ValidationData` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::LastRelayChainBlockNumber` (r:1 w:0)
	/// Proof: `ParachainSystem::LastRelayChainBlockNumber` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn set_lease() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `506`
		//  Estimated: `1991`
		// Minimum execution time: 18_541_000 picoseconds.
		Weight::from_parts(18_980_000, 0)
			.saturating_add(Weight::from_parts(0, 1991))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Broker::Configuration` (r:1 w:0)
	/// Proof: `Broker::Configuration` (`max_values`: Some(1), `max_size`: Some(31), added: 526, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Leases` (r:1 w:1)
	/// Proof: `Broker::Leases` (`max_values`: Some(1), `max_size`: Some(441), added: 936, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Reservations` (r:1 w:0)
	/// Proof: `Broker::Reservations` (`max_values`: Some(1), `max_size`: Some(12021), added: 12516, mode: `MaxEncodedLen`)
	/// Storage: `ParachainSystem::UpwardDeliveryFeeFactor` (r:1 w:0)
	/// Proof: `ParachainSystem::UpwardDeliveryFeeFactor` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::SupportedVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::HostConfiguration` (r:1 w:0)
	/// Proof: `ParachainSystem::HostConfiguration` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::PendingUpwardMessages` (r:1 w:1)
	/// Proof: `ParachainSystem::PendingUpwardMessages` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::ValidationData` (r:1 w:0)
	/// Proof: `ParachainSystem::ValidationData` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::LastRelayChainBlockNumber` (r:1 w:0)
	/// Proof: `ParachainSystem::LastRelayChainBlockNumber` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Broker::InstaPoolIo` (r:3 w:3)
	/// Proof: `Broker::InstaPoolIo` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Broker::AutoRenewals` (r:1 w:1)
	/// Proof: `Broker::AutoRenewals` (`max_values`: Some(1), `max_size`: Some(1002), added: 1497, mode: `MaxEncodedLen`)
	/// Storage: `Broker::SaleInfo` (r:0 w:1)
	/// Proof: `Broker::SaleInfo` (`max_values`: Some(1), `max_size`: Some(57), added: 552, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Status` (r:0 w:1)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Workplan` (r:0 w:65)
	/// Proof: `Broker::Workplan` (`max_values`: None, `max_size`: Some(1216), added: 3691, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[0, 1000]`.
	fn start_sales(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `12607`
		//  Estimated: `15061 + n * (2 ±0)`
		// Minimum execution time: 61_311_000 picoseconds.
		Weight::from_parts(172_346_933, 0)
			.saturating_add(Weight::from_parts(0, 15061))
			// Standard Error: 3_027
			.saturating_add(Weight::from_parts(28_474, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(13))
			.saturating_add(T::DbWeight::get().writes(62))
			.saturating_add(Weight::from_parts(0, 2).saturating_mul(n.into()))
	}
	/// Storage: `Broker::Status` (r:1 w:0)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `Broker::SaleInfo` (r:1 w:1)
	/// Proof: `Broker::SaleInfo` (`max_values`: Some(1), `max_size`: Some(57), added: 552, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Regions` (r:0 w:1)
	/// Proof: `Broker::Regions` (`max_values`: None, `max_size`: Some(86), added: 2561, mode: `MaxEncodedLen`)
	fn purchase() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `371`
		//  Estimated: `3593`
		// Minimum execution time: 68_511_000 picoseconds.
		Weight::from_parts(69_461_000, 0)
			.saturating_add(Weight::from_parts(0, 3593))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Broker::Configuration` (r:1 w:0)
	/// Proof: `Broker::Configuration` (`max_values`: Some(1), `max_size`: Some(31), added: 526, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Status` (r:1 w:0)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `Broker::SaleInfo` (r:1 w:1)
	/// Proof: `Broker::SaleInfo` (`max_values`: Some(1), `max_size`: Some(57), added: 552, mode: `MaxEncodedLen`)
	/// Storage: `Broker::PotentialRenewals` (r:1 w:2)
	/// Proof: `Broker::PotentialRenewals` (`max_values`: None, `max_size`: Some(1233), added: 3708, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Workplan` (r:0 w:1)
	/// Proof: `Broker::Workplan` (`max_values`: None, `max_size`: Some(1216), added: 3691, mode: `MaxEncodedLen`)
	fn renew() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `624`
		//  Estimated: `4698`
		// Minimum execution time: 123_691_000 picoseconds.
		Weight::from_parts(128_801_000, 0)
			.saturating_add(Weight::from_parts(0, 4698))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `Broker::Regions` (r:1 w:1)
	/// Proof: `Broker::Regions` (`max_values`: None, `max_size`: Some(86), added: 2561, mode: `MaxEncodedLen`)
	fn transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `358`
		//  Estimated: `3551`
		// Minimum execution time: 20_320_000 picoseconds.
		Weight::from_parts(20_640_000, 0)
			.saturating_add(Weight::from_parts(0, 3551))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Broker::Regions` (r:1 w:2)
	/// Proof: `Broker::Regions` (`max_values`: None, `max_size`: Some(86), added: 2561, mode: `MaxEncodedLen`)
	fn partition() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `358`
		//  Estimated: `3551`
		// Minimum execution time: 22_010_000 picoseconds.
		Weight::from_parts(22_490_000, 0)
			.saturating_add(Weight::from_parts(0, 3551))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Broker::Regions` (r:1 w:3)
	/// Proof: `Broker::Regions` (`max_values`: None, `max_size`: Some(86), added: 2561, mode: `MaxEncodedLen`)
	fn interlace() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `358`
		//  Estimated: `3551`
		// Minimum execution time: 23_650_000 picoseconds.
		Weight::from_parts(24_150_000, 0)
			.saturating_add(Weight::from_parts(0, 3551))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Broker::Configuration` (r:1 w:0)
	/// Proof: `Broker::Configuration` (`max_values`: Some(1), `max_size`: Some(31), added: 526, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Status` (r:1 w:0)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Regions` (r:1 w:1)
	/// Proof: `Broker::Regions` (`max_values`: None, `max_size`: Some(86), added: 2561, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Workplan` (r:1 w:1)
	/// Proof: `Broker::Workplan` (`max_values`: None, `max_size`: Some(1216), added: 3691, mode: `MaxEncodedLen`)
	fn assign() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `939`
		//  Estimated: `4681`
		// Minimum execution time: 42_731_000 picoseconds.
		Weight::from_parts(43_490_000, 0)
			.saturating_add(Weight::from_parts(0, 4681))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Broker::Status` (r:1 w:0)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Regions` (r:1 w:1)
	/// Proof: `Broker::Regions` (`max_values`: None, `max_size`: Some(86), added: 2561, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Workplan` (r:1 w:1)
	/// Proof: `Broker::Workplan` (`max_values`: None, `max_size`: Some(1216), added: 3691, mode: `MaxEncodedLen`)
	/// Storage: `Broker::InstaPoolIo` (r:2 w:2)
	/// Proof: `Broker::InstaPoolIo` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Broker::InstaPoolContribution` (r:0 w:1)
	/// Proof: `Broker::InstaPoolContribution` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	fn pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1005`
		//  Estimated: `5996`
		// Minimum execution time: 50_921_000 picoseconds.
		Weight::from_parts(51_350_000, 0)
			.saturating_add(Weight::from_parts(0, 5996))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `Broker::InstaPoolContribution` (r:1 w:1)
	/// Proof: `Broker::InstaPoolContribution` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `Broker::InstaPoolHistory` (r:3 w:1)
	/// Proof: `Broker::InstaPoolHistory` (`max_values`: None, `max_size`: Some(45), added: 2520, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `m` is `[1, 3]`.
	fn claim_revenue(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `671`
		//  Estimated: `6196 + m * (2520 ±0)`
		// Minimum execution time: 83_730_000 picoseconds.
		Weight::from_parts(83_014_268, 0)
			.saturating_add(Weight::from_parts(0, 6196))
			// Standard Error: 32_427
			.saturating_add(Weight::from_parts(1_972_070, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(m.into())))
			.saturating_add(T::DbWeight::get().writes(5))
			.saturating_add(Weight::from_parts(0, 2520).saturating_mul(m.into()))
	}
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn purchase_credit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `103`
		//  Estimated: `3593`
		// Minimum execution time: 51_241_000 picoseconds.
		Weight::from_parts(51_481_000, 0)
			.saturating_add(Weight::from_parts(0, 3593))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Broker::Status` (r:1 w:0)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Regions` (r:1 w:1)
	/// Proof: `Broker::Regions` (`max_values`: None, `max_size`: Some(86), added: 2561, mode: `MaxEncodedLen`)
	fn drop_region() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `466`
		//  Estimated: `3551`
		// Minimum execution time: 60_091_000 picoseconds.
		Weight::from_parts(69_601_000, 0)
			.saturating_add(Weight::from_parts(0, 3551))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Broker::Configuration` (r:1 w:0)
	/// Proof: `Broker::Configuration` (`max_values`: Some(1), `max_size`: Some(31), added: 526, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Status` (r:1 w:0)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `Broker::InstaPoolContribution` (r:1 w:1)
	/// Proof: `Broker::InstaPoolContribution` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	fn drop_contribution() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `463`
		//  Estimated: `3533`
		// Minimum execution time: 76_201_000 picoseconds.
		Weight::from_parts(99_411_000, 0)
			.saturating_add(Weight::from_parts(0, 3533))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Broker::Configuration` (r:1 w:0)
	/// Proof: `Broker::Configuration` (`max_values`: Some(1), `max_size`: Some(31), added: 526, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Status` (r:1 w:0)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `Broker::InstaPoolHistory` (r:1 w:1)
	/// Proof: `Broker::InstaPoolHistory` (`max_values`: None, `max_size`: Some(45), added: 2520, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn drop_history() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `979`
		//  Estimated: `3593`
		// Minimum execution time: 93_970_000 picoseconds.
		Weight::from_parts(110_981_000, 0)
			.saturating_add(Weight::from_parts(0, 3593))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Broker::Status` (r:1 w:0)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `Broker::PotentialRenewals` (r:1 w:1)
	/// Proof: `Broker::PotentialRenewals` (`max_values`: None, `max_size`: Some(1233), added: 3708, mode: `MaxEncodedLen`)
	fn drop_renewal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `890`
		//  Estimated: `4698`
		// Minimum execution time: 59_681_000 picoseconds.
		Weight::from_parts(61_900_000, 0)
			.saturating_add(Weight::from_parts(0, 4698))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `ParachainSystem::UpwardDeliveryFeeFactor` (r:1 w:0)
	/// Proof: `ParachainSystem::UpwardDeliveryFeeFactor` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::SupportedVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::HostConfiguration` (r:1 w:0)
	/// Proof: `ParachainSystem::HostConfiguration` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::PendingUpwardMessages` (r:1 w:1)
	/// Proof: `ParachainSystem::PendingUpwardMessages` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `n` is `[0, 1000]`.
	fn request_core_count(_n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `3507`
		// Minimum execution time: 25_870_000 picoseconds.
		Weight::from_parts(26_991_865, 0)
			.saturating_add(Weight::from_parts(0, 3507))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Broker::CoreCountInbox` (r:1 w:1)
	/// Proof: `Broker::CoreCountInbox` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[0, 1000]`.
	fn process_core_count(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `266`
		//  Estimated: `1487`
		// Minimum execution time: 11_340_000 picoseconds.
		Weight::from_parts(11_660_727, 0)
			.saturating_add(Weight::from_parts(0, 1487))
			// Standard Error: 9
			.saturating_add(Weight::from_parts(25, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Broker::RevenueInbox` (r:1 w:1)
	/// Proof: `Broker::RevenueInbox` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	/// Storage: `Broker::InstaPoolHistory` (r:1 w:1)
	/// Proof: `Broker::InstaPoolHistory` (`max_values`: None, `max_size`: Some(45), added: 2520, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn process_revenue() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `500`
		//  Estimated: `6196`
		// Minimum execution time: 66_040_000 picoseconds.
		Weight::from_parts(66_940_000, 0)
			.saturating_add(Weight::from_parts(0, 6196))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `Broker::InstaPoolIo` (r:3 w:3)
	/// Proof: `Broker::InstaPoolIo` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Reservations` (r:1 w:0)
	/// Proof: `Broker::Reservations` (`max_values`: Some(1), `max_size`: Some(12021), added: 12516, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Leases` (r:1 w:1)
	/// Proof: `Broker::Leases` (`max_values`: Some(1), `max_size`: Some(441), added: 936, mode: `MaxEncodedLen`)
	/// Storage: `Broker::AutoRenewals` (r:1 w:1)
	/// Proof: `Broker::AutoRenewals` (`max_values`: Some(1), `max_size`: Some(1002), added: 1497, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Configuration` (r:1 w:0)
	/// Proof: `Broker::Configuration` (`max_values`: Some(1), `max_size`: Some(31), added: 526, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Status` (r:1 w:0)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `Broker::PotentialRenewals` (r:100 w:200)
	/// Proof: `Broker::PotentialRenewals` (`max_values`: None, `max_size`: Some(1233), added: 3708, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:101 w:100)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Broker::SaleInfo` (r:0 w:1)
	/// Proof: `Broker::SaleInfo` (`max_values`: Some(1), `max_size`: Some(57), added: 552, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Workplan` (r:0 w:1000)
	/// Proof: `Broker::Workplan` (`max_values`: None, `max_size`: Some(1216), added: 3691, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[0, 1000]`.
	fn rotate_sale(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `32704`
		//  Estimated: `227970 + n * (206 ±13)`
		// Minimum execution time: 31_430_000 picoseconds.
		Weight::from_parts(2_878_405_756, 0)
			.saturating_add(Weight::from_parts(0, 227970))
			// Standard Error: 180_740
			.saturating_add(Weight::from_parts(4_738_530, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(122))
			.saturating_add(T::DbWeight::get().writes(175))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 206).saturating_mul(n.into()))
	}
	/// Storage: `Broker::InstaPoolIo` (r:1 w:0)
	/// Proof: `Broker::InstaPoolIo` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Broker::InstaPoolHistory` (r:0 w:1)
	/// Proof: `Broker::InstaPoolHistory` (`max_values`: None, `max_size`: Some(45), added: 2520, mode: `MaxEncodedLen`)
	fn process_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `3493`
		// Minimum execution time: 9_210_000 picoseconds.
		Weight::from_parts(9_700_000, 0)
			.saturating_add(Weight::from_parts(0, 3493))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Broker::Workplan` (r:1 w:1)
	/// Proof: `Broker::Workplan` (`max_values`: None, `max_size`: Some(1216), added: 3691, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Workload` (r:1 w:1)
	/// Proof: `Broker::Workload` (`max_values`: None, `max_size`: Some(1212), added: 3687, mode: `MaxEncodedLen`)
	/// Storage: `ParachainSystem::UpwardDeliveryFeeFactor` (r:1 w:0)
	/// Proof: `ParachainSystem::UpwardDeliveryFeeFactor` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::SupportedVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::HostConfiguration` (r:1 w:0)
	/// Proof: `ParachainSystem::HostConfiguration` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::PendingUpwardMessages` (r:1 w:1)
	/// Proof: `ParachainSystem::PendingUpwardMessages` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn process_core_schedule() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1289`
		//  Estimated: `4754`
		// Minimum execution time: 41_740_000 picoseconds.
		Weight::from_parts(42_420_000, 0)
			.saturating_add(Weight::from_parts(0, 4754))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `ParachainSystem::UpwardDeliveryFeeFactor` (r:1 w:0)
	/// Proof: `ParachainSystem::UpwardDeliveryFeeFactor` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::SupportedVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::HostConfiguration` (r:1 w:0)
	/// Proof: `ParachainSystem::HostConfiguration` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::PendingUpwardMessages` (r:1 w:1)
	/// Proof: `ParachainSystem::PendingUpwardMessages` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn request_revenue_info_at() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `3507`
		// Minimum execution time: 20_361_000 picoseconds.
		Weight::from_parts(20_841_000, 0)
			.saturating_add(Weight::from_parts(0, 3507))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Broker::CoreCountInbox` (r:0 w:1)
	/// Proof: `Broker::CoreCountInbox` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	fn notify_core_count() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_980_000 picoseconds.
		Weight::from_parts(3_120_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Broker::RevenueInbox` (r:0 w:1)
	/// Proof: `Broker::RevenueInbox` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	fn notify_revenue() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_220_000 picoseconds.
		Weight::from_parts(3_380_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Broker::Status` (r:1 w:1)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Configuration` (r:1 w:0)
	/// Proof: `Broker::Configuration` (`max_values`: Some(1), `max_size`: Some(31), added: 526, mode: `MaxEncodedLen`)
	/// Storage: `Broker::CoreCountInbox` (r:1 w:0)
	/// Proof: `Broker::CoreCountInbox` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `Broker::RevenueInbox` (r:1 w:0)
	/// Proof: `Broker::RevenueInbox` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	/// Storage: `ParachainSystem::ValidationData` (r:1 w:0)
	/// Proof: `ParachainSystem::ValidationData` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn do_tick_base() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `408`
		//  Estimated: `1893`
		// Minimum execution time: 19_780_000 picoseconds.
		Weight::from_parts(20_160_000, 0)
			.saturating_add(Weight::from_parts(0, 1893))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Broker::Leases` (r:1 w:1)
	/// Proof: `Broker::Leases` (`max_values`: Some(1), `max_size`: Some(441), added: 936, mode: `MaxEncodedLen`)
	fn swap_leases() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `502`
		//  Estimated: `1926`
		// Minimum execution time: 11_190_000 picoseconds.
		Weight::from_parts(11_451_000, 0)
			.saturating_add(Weight::from_parts(0, 1926))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Broker::SaleInfo` (r:1 w:1)
	/// Proof: `Broker::SaleInfo` (`max_values`: Some(1), `max_size`: Some(57), added: 552, mode: `MaxEncodedLen`)
	/// Storage: `Broker::PotentialRenewals` (r:1 w:2)
	/// Proof: `Broker::PotentialRenewals` (`max_values`: None, `max_size`: Some(1233), added: 3708, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Configuration` (r:1 w:0)
	/// Proof: `Broker::Configuration` (`max_values`: Some(1), `max_size`: Some(31), added: 526, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Status` (r:1 w:0)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Broker::AutoRenewals` (r:1 w:1)
	/// Proof: `Broker::AutoRenewals` (`max_values`: Some(1), `max_size`: Some(1002), added: 1497, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Workplan` (r:0 w:1)
	/// Proof: `Broker::Workplan` (`max_values`: None, `max_size`: Some(1216), added: 3691, mode: `MaxEncodedLen`)
	fn enable_auto_renew() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2895`
		//  Estimated: `6196`
		// Minimum execution time: 177_001_000 picoseconds.
		Weight::from_parts(187_351_000, 0)
			.saturating_add(Weight::from_parts(0, 6196))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: `Broker::AutoRenewals` (r:1 w:1)
	/// Proof: `Broker::AutoRenewals` (`max_values`: Some(1), `max_size`: Some(1002), added: 1497, mode: `MaxEncodedLen`)
	fn disable_auto_renew() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1307`
		//  Estimated: `2487`
		// Minimum execution time: 23_470_000 picoseconds.
		Weight::from_parts(24_930_000, 0)
			.saturating_add(Weight::from_parts(0, 2487))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn on_new_timeslice() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `174`
		//  Estimated: `3593`
		// Minimum execution time: 6_990_000 picoseconds.
		Weight::from_parts(7_230_000, 0)
			.saturating_add(Weight::from_parts(0, 3593))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: `Broker::Leases` (r:1 w:1)
	/// Proof: `Broker::Leases` (`max_values`: Some(1), `max_size`: Some(81), added: 576, mode: `MaxEncodedLen`)
	fn remove_lease() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `150`
		//  Estimated: `1566`
		// Minimum execution time: 10_319_000 picoseconds.
		Weight::from_parts(10_718_000, 0)
			.saturating_add(Weight::from_parts(0, 1566))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Broker::SaleInfo` (r:1 w:0)
	/// Proof: `Broker::SaleInfo` (`max_values`: Some(1), `max_size`: Some(57), added: 552, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Reservations` (r:1 w:1)
	/// Proof: `Broker::Reservations` (`max_values`: Some(1), `max_size`: Some(12021), added: 12516, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Status` (r:1 w:0)
	/// Proof: `Broker::Status` (`max_values`: Some(1), `max_size`: Some(18), added: 513, mode: `MaxEncodedLen`)
	/// Storage: `Broker::Workplan` (r:0 w:2)
	/// Proof: `Broker::Workplan` (`max_values`: None, `max_size`: Some(1216), added: 3691, mode: `MaxEncodedLen`)
	fn force_reserve() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `11141`
		//  Estimated: `13506`
		// Minimum execution time: 41_171_000 picoseconds.
		Weight::from_parts(42_826_000, 0)
			.saturating_add(Weight::from_parts(0, 13506))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Broker::Workplan` (r:1 w:1)
	/// Proof: `Broker::Workplan` (`max_values`: None, `max_size`: Some(1216), added: 3691, mode: `MaxEncodedLen`)
	fn remove_assignment() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `602`
		//  Estimated: `4681`
		// Minimum execution time: 21_397_000 picoseconds.
		Weight::from_parts(21_865_000, 0)
			.saturating_add(Weight::from_parts(0, 4681))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
