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

use crate::*;
use emulated_integration_tests_common::accounts::{ALICE, BOB};
use frame_support::traits::fungibles::Mutate;
use polkadot_runtime_common::impls::VersionedLocatableAsset;
use polkadot_system_emulated_network::polkadot_emulated_chain::polkadot_runtime::Dmp;
use xcm_executor::traits::ConvertLocation;

#[test]
fn create_and_claim_treasury_spend_in_usdt() {
	const USDT_ID: u32 = 1984;
	const SPEND_AMOUNT: u128 = 1_000_000_000;
	// treasury location from a sibling parachain.
	let treasury_location: Location = Location::new(1, PalletInstance(19));
	// treasury account on a sibling parachain.
	let treasury_account =
		asset_hub_polkadot_runtime::xcm_config::LocationToAccountId::convert_location(
			&treasury_location,
		)
		.unwrap();
	let asset_hub_location =
		v5::Location::new(0, v5::Junction::Parachain(AssetHubPolkadot::para_id().into()));
	let root = <Polkadot as Chain>::RuntimeOrigin::root();
	// asset kind to be spend from the treasury.
	let asset_kind = VersionedLocatableAsset::V5 {
		location: asset_hub_location,
		asset_id: v5::AssetId(
			(v5::Junction::PalletInstance(50), v5::Junction::GeneralIndex(USDT_ID.into())).into(),
		),
	};
	// treasury spend beneficiary.
	let alice: AccountId = Polkadot::account_id_of(ALICE);
	let bob: AccountId = Polkadot::account_id_of(BOB);
	let bob_signed = <Polkadot as Chain>::RuntimeOrigin::signed(bob.clone());

	AssetHubPolkadot::execute_with(|| {
		type Assets = <AssetHubPolkadot as AssetHubPolkadotPallet>::Assets;

		// USDT created at genesis, mint some assets to the treasury account.
		assert_ok!(<Assets as Mutate<_>>::mint_into(USDT_ID, &treasury_account, SPEND_AMOUNT * 4));
		// beneficiary has zero balance.
		assert_eq!(<Assets as Inspect<_>>::balance(USDT_ID, &alice,), 0u128,);
	});

	Polkadot::execute_with(|| {
		type RuntimeEvent = <Polkadot as Chain>::RuntimeEvent;
		type Treasury = <Polkadot as PolkadotPallet>::Treasury;
		type AssetRate = <Polkadot as PolkadotPallet>::AssetRate;

		// create a conversion rate from `asset_kind` to the native currency.
		assert_ok!(AssetRate::create(root.clone(), Box::new(asset_kind.clone()), 2.into()));

		Dmp::make_parachain_reachable(1000);
		// create and approve a treasury spend.
		assert_ok!(Treasury::spend(
			root,
			Box::new(asset_kind),
			SPEND_AMOUNT,
			Box::new(Location::new(0, Into::<[u8; 32]>::into(alice.clone())).into()),
			None,
		));
		// claim the spend.
		assert_ok!(Treasury::payout(bob_signed.clone(), 0));

		assert_expected_events!(
			Polkadot,
			vec![
				RuntimeEvent::Treasury(pallet_treasury::Event::Paid { .. }) => {},
			]
		);
	});

	AssetHubPolkadot::execute_with(|| {
		type RuntimeEvent = <AssetHubPolkadot as Chain>::RuntimeEvent;
		type Assets = <AssetHubPolkadot as AssetHubPolkadotPallet>::Assets;

		// assert events triggered by xcm pay program
		// 1. treasury asset transferred to spend beneficiary
		// 2. response to Relay Chain treasury pallet instance sent back
		// 3. XCM program completed
		assert_expected_events!(
			AssetHubPolkadot,
			vec![
				RuntimeEvent::Assets(pallet_assets::Event::Transferred { asset_id: id, from, to, amount }) => {
					id: id == &USDT_ID,
					from: from == &treasury_account,
					to: to == &alice,
					amount: amount == &SPEND_AMOUNT,
				},
				RuntimeEvent::ParachainSystem(cumulus_pallet_parachain_system::Event::UpwardMessageSent { .. }) => {},
				RuntimeEvent::MessageQueue(pallet_message_queue::Event::Processed { success: true ,.. }) => {},
			]
		);
		// beneficiary received the assets from the treasury.
		assert_eq!(<Assets as Inspect<_>>::balance(USDT_ID, &alice,), SPEND_AMOUNT,);
	});

	Polkadot::execute_with(|| {
		type RuntimeEvent = <Polkadot as Chain>::RuntimeEvent;
		type Treasury = <Polkadot as PolkadotPallet>::Treasury;

		// check the payment status to ensure the response from the AssetHub was received.
		assert_ok!(Treasury::check_status(bob_signed, 0));
		assert_expected_events!(
			Polkadot,
			vec![
				RuntimeEvent::Treasury(pallet_treasury::Event::SpendProcessed { .. }) => {},
			]
		);
	});
}
