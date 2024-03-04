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

mod reserve_transfer;
mod send;
mod set_xcm_versions;
mod swap;
mod teleport;

use crate::*;
emulated_integration_tests_common::include_penpal_create_foreign_asset_on_asset_hub!(
	PenpalA,
	AssetHubKusama,
	KUSAMA_ED,
	system_parachains_constants::kusama::fee::WeightToFee
);

#[test]
fn assets_can_be_claimed() {
	use asset_hub_kusama_runtime::ExistentialDeposit;
	use integration_tests_helpers::test_parachain_can_claim_assets;
	use xcm_executor::traits::DropAssets;

	let amount = ExistentialDeposit::get();
	let assets: Assets = (Parent, amount).into();

	test_parachain_can_claim_assets!(AssetHubKusama, RuntimeCall, NetworkId::Kusama, assets, amount);
}
