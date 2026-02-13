// Copyright (C) Parity Technologies (UK) Ltd.
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

use super::*;
use alloc::vec::Vec;
use assets_common::{
	local_and_foreign_assets::ForeignAssetReserveData,
	migrations::foreign_assets_reserves::ForeignAssetsReservesProvider,
};
use xcm::latest::prelude::*;

/// Unreleased single-block migrations. Add new ones here:
pub type Unreleased = ();

/// Migrations/checks that do not need to be versioned and can run on every update.
pub type Permanent = pallet_xcm::migration::MigrateToLatestXcmVersion<Runtime>;

/// All single block migrations that will run on the next runtime upgrade.
pub type SingleBlockMigrations = (Unreleased, Permanent);

/// MBM migrations to apply on runtime upgrade.
pub type MbmMigrations = (
	assets_common::migrations::foreign_assets_reserves::ForeignAssetsReservesMigration<
		Runtime,
		crate::ForeignAssetsInstance,
		AssetHubPolkadotForeignAssetsReservesProvider,
	>,
);

/// Provides reserve data for all existing foreign assets on Polkadot Asset Hub.
///
/// Rules (matching current hardcoded XCM config):
/// - Sibling parachain assets `(1, [Parachain(id)])` → reserve is the sibling, teleportable
/// - Kusama ecosystem assets `StartsWith(2, [GlobalConsensus(Kusama)])` → reserve is AH Kusama, not
///   teleportable
/// - Ethereum ecosystem assets `StartsWith(2, [GlobalConsensus(Ethereum{chain_id:1})])` → reserve
///   is Ethereum, not teleportable
pub struct AssetHubPolkadotForeignAssetsReservesProvider;

impl ForeignAssetsReservesProvider for AssetHubPolkadotForeignAssetsReservesProvider {
	type ReserveData = ForeignAssetReserveData;

	fn reserves_for(asset_id: &Location) -> Vec<Self::ReserveData> {
		use bp_bridge_hub_polkadot::snowbridge::EthereumNetwork;
		use xcm_config::bridging::to_kusama::{AssetHubKusama, KsmLocation};

		let ethereum_location = Location::new(2, [GlobalConsensus(EthereumNetwork::get())]);

		match asset_id.unpack() {
			// Sibling parachain asset: reserve is the originating parachain, teleportable.
			(1, [Parachain(id)]) => {
				let reserve = Location::new(1, [Parachain(*id)]);
				alloc::vec![(reserve, true).into()]
			},
			// Kusama ecosystem asset: reserve is Asset Hub Kusama, not teleportable.
			(2, _) if asset_id.starts_with(&KsmLocation::get()) => {
				let reserve = AssetHubKusama::get();
				alloc::vec![(reserve, false).into()]
			},
			// Ethereum ecosystem asset: reserve is Ethereum, not teleportable.
			(2, _) if asset_id.starts_with(&ethereum_location) => {
				alloc::vec![(ethereum_location, false).into()]
			},
			_ => {
				log::warn!(
					target: "runtime::migrations",
					"No reserve data for foreign asset {:?}",
					asset_id,
				);
				alloc::vec![]
			},
		}
	}

	#[cfg(feature = "try-runtime")]
	fn check_reserves_for(asset_id: &Location, reserves: Vec<Self::ReserveData>) -> bool {
		let expected = Self::reserves_for(asset_id);
		expected == reserves
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use assets_common::local_and_foreign_assets::ForeignAssetReserveData;

	#[test]
	fn sibling_parachain_asset_reserve() {
		let asset = Location::new(1, [Parachain(2000)]);
		let reserves = AssetHubPolkadotForeignAssetsReservesProvider::reserves_for(&asset);
		assert_eq!(reserves.len(), 1);
		let expected: ForeignAssetReserveData = (Location::new(1, [Parachain(2000)]), true).into();
		assert_eq!(reserves[0], expected);
	}

	#[test]
	fn kusama_ecosystem_asset_reserve() {
		let asset = Location::new(2, [GlobalConsensus(NetworkId::Kusama), Parachain(1000)]);
		let reserves = AssetHubPolkadotForeignAssetsReservesProvider::reserves_for(&asset);
		assert_eq!(reserves.len(), 1);
		let expected_reserve = Location::new(
			2,
			[
				GlobalConsensus(NetworkId::Kusama),
				Parachain(kusama_runtime_constants::system_parachain::ASSET_HUB_ID),
			],
		);
		let expected: ForeignAssetReserveData = (expected_reserve, false).into();
		assert_eq!(reserves[0], expected);
	}

	#[test]
	fn ethereum_ecosystem_asset_reserve() {
		use bp_bridge_hub_polkadot::snowbridge::EthereumNetwork;

		let asset = Location::new(
			2,
			[
				GlobalConsensus(NetworkId::Ethereum { chain_id: 1 }),
				AccountKey20 { network: None, key: [0u8; 20] },
			],
		);
		let reserves = AssetHubPolkadotForeignAssetsReservesProvider::reserves_for(&asset);
		assert_eq!(reserves.len(), 1);
		let expected_reserve = Location::new(2, [GlobalConsensus(EthereumNetwork::get())]);
		let expected: ForeignAssetReserveData = (expected_reserve, false).into();
		assert_eq!(reserves[0], expected);
	}

	#[test]
	fn unknown_asset_returns_empty() {
		// Parent location (relay chain native token) is not a foreign asset with a known reserve.
		let asset = Location::new(1, Here);
		let reserves = AssetHubPolkadotForeignAssetsReservesProvider::reserves_for(&asset);
		assert!(reserves.is_empty());

		// Here location.
		let asset = Location::new(0, Here);
		let reserves = AssetHubPolkadotForeignAssetsReservesProvider::reserves_for(&asset);
		assert!(reserves.is_empty());
	}
}
