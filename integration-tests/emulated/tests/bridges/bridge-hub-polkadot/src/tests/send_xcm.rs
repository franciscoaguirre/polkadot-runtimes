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

use crate::tests::*;

#[test]
fn send_xcm_from_polkadot_relay_to_kusama_asset_hub_should_fail_on_not_applicable() {
	// Init tests variables
	// XcmPallet send arguments
	let sudo_origin = <Polkadot as Chain>::RuntimeOrigin::root();
	let destination = Polkadot::child_location_of(BridgeHubPolkadot::para_id()).into();
	let weight_limit = WeightLimit::Unlimited;
	let check_origin = None;

	let remote_xcm = Xcm(vec![ClearOrigin]);

	let xcm = VersionedXcm::from(Xcm(vec![
		UnpaidExecution { weight_limit, check_origin },
		ExportMessage {
			network: KusamaId,
			destination: [Parachain(AssetHubKusama::para_id().into())].into(),
			xcm: remote_xcm,
		},
	]));

	// Polkadot Global Consensus
	// Send XCM message from Relay Chain to Bridge Hub source Parachain
	Polkadot::execute_with(|| {
		Dmp::make_parachain_reachable(BridgeHubPolkadot::para_id());
		assert_ok!(<Polkadot as PolkadotPallet>::XcmPallet::send(
			sudo_origin,
			bx!(destination),
			bx!(xcm),
		));

		type RuntimeEvent = <Polkadot as Chain>::RuntimeEvent;

		assert_expected_events!(
			Polkadot,
			vec![
				RuntimeEvent::XcmPallet(pallet_xcm::Event::Sent { .. }) => {},
			]
		);
	});
	// Receive XCM message in Bridge Hub source Parachain, it should fail, because we don't have
	// opened bridge/lane.
	assert_bridge_hub_polkadot_message_accepted(false);
}

#[test]
fn send_xcm_through_opened_lane_with_different_xcm_version_on_hops_works() {
	// Initially set only default version on all runtimes
	let newer_xcm_version = xcm::prelude::XCM_VERSION;
	let older_xcm_version = newer_xcm_version - 1;

	AssetHubKusama::force_default_xcm_version(Some(older_xcm_version));
	BridgeHubKusama::force_default_xcm_version(Some(older_xcm_version));
	BridgeHubPolkadot::force_default_xcm_version(Some(older_xcm_version));
	AssetHubPolkadot::force_default_xcm_version(Some(older_xcm_version));

	// prepare data
	let destination = asset_hub_kusama_location();
	let native_token = Location::parent();
	let amount = ASSET_HUB_POLKADOT_ED * 1_000;

	// fund the AHK's SA on BHK for paying bridge transport fees
	BridgeHubPolkadot::fund_para_sovereign(AssetHubPolkadot::para_id(), 10_000_000_000_000u128);
	// fund sender
	AssetHubPolkadot::fund_accounts(vec![(AssetHubPolkadotSender::get(), amount * 10)]);

	// send XCM from AssetHubPolkadot - fails - destination version not known
	assert_err!(
		send_assets_from_asset_hub_polkadot(
			destination.clone(),
			(native_token.clone(), amount).into(),
			0
		),
		DispatchError::Module(sp_runtime::ModuleError {
			index: 31,
			error: [1, 0, 0, 0],
			message: Some("SendFailure")
		})
	);

	// set destination version
	AssetHubPolkadot::force_xcm_version(destination.clone(), newer_xcm_version);

	// set version with `ExportMessage` for BridgeHubPolkadot
	AssetHubPolkadot::force_xcm_version(
		ParentThen(Parachain(BridgeHubPolkadot::para_id().into()).into()).into(),
		newer_xcm_version,
	);
	// send XCM from AssetHubPolkadot - ok
	assert_ok!(send_assets_from_asset_hub_polkadot(
		destination.clone(),
		(native_token.clone(), amount).into(),
		0
	));

	// `ExportMessage` on local BridgeHub - fails - remote BridgeHub version not known
	assert_bridge_hub_polkadot_message_accepted(false);

	// set version for remote BridgeHub on BridgeHubPolkadot
	BridgeHubPolkadot::force_xcm_version(bridge_hub_kusama_location(), newer_xcm_version);
	// set version for AssetHubKusama on BridgeHubKusama
	BridgeHubKusama::force_xcm_version(
		ParentThen(Parachain(AssetHubKusama::para_id().into()).into()).into(),
		newer_xcm_version,
	);

	// send XCM from AssetHubPolkadot - ok
	assert_ok!(send_assets_from_asset_hub_polkadot(
		destination.clone(),
		(native_token.clone(), amount).into(),
		0
	));
	assert_bridge_hub_polkadot_message_accepted(true);
	assert_bridge_hub_kusama_message_received();
	// message delivered and processed at destination
	AssetHubKusama::execute_with(|| {
		type RuntimeEvent = <AssetHubKusama as Chain>::RuntimeEvent;
		assert_expected_events!(
			AssetHubKusama,
			vec![
				// message processed with failure, but for this scenario it is ok, important is that was delivered
				RuntimeEvent::MessageQueue(
					pallet_message_queue::Event::Processed { success: false, .. }
				) => {},
			]
		);
	});
}
