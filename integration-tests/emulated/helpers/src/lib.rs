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

pub use paste;

// Substrate
pub use pallet_balances;
pub use pallet_message_queue;

// Polkadot
pub use pallet_xcm;
pub use xcm::prelude::{AccountId32, VersionedAssets, Weight, WeightLimit};

// Cumulus
pub use asset_test_utils;
pub use cumulus_pallet_xcmp_queue;
pub use xcm_emulator::Chain;

/// TODO: when bumping to polkadot-sdk v1.8.0,
/// remove this crate altogether and get the macros from `emulated-integration-tests-common`.
#[macro_export]
macro_rules! test_parachain_is_trusted_teleporter {
	( $sender_para:ty, $sender_xcm_config:ty, vec![$( $receiver_para:ty ),+], ($assets:expr, $amount:expr) ) => {
		$crate::paste::paste! {
			// init Origin variables
			let sender = [<$sender_para Sender>]::get();
			let mut para_sender_balance_before =
				<$sender_para as $crate::Chain>::account_data_of(sender.clone()).free;
			let origin = <$sender_para as $crate::Chain>::RuntimeOrigin::signed(sender.clone());
			let fee_asset_item = 0;
			let weight_limit = $crate::WeightLimit::Unlimited;

			$(
				{
					// init Destination variables
					let receiver = [<$receiver_para Receiver>]::get();
					let para_receiver_balance_before =
						<$receiver_para as $crate::Chain>::account_data_of(receiver.clone()).free;
					let para_destination =
						<$sender_para>::sibling_location_of(<$receiver_para>::para_id());
					let beneficiary: Location =
						$crate::AccountId32 { network: None, id: receiver.clone().into() }.into();

					// Send XCM message from Origin Parachain
					// We are only testing the limited teleport version, which should be ok since success will
					// depend only on a proper `XcmConfig` at destination.
					<$sender_para>::execute_with(|| {
						assert_ok!(<$sender_para as [<$sender_para Pallet>]>::PolkadotXcm::limited_teleport_assets(
							origin.clone(),
							bx!(para_destination.clone().into()),
							bx!(beneficiary.clone().into()),
							bx!($assets.clone().into()),
							fee_asset_item,
							weight_limit.clone(),
						));

						type RuntimeEvent = <$sender_para as $crate::Chain>::RuntimeEvent;

						assert_expected_events!(
							$sender_para,
							vec![
								RuntimeEvent::PolkadotXcm(
									$crate::pallet_xcm::Event::Attempted { outcome: Outcome::Complete { .. } }
								) => {},
								RuntimeEvent::XcmpQueue(
									$crate::cumulus_pallet_xcmp_queue::Event::XcmpMessageSent { .. }
								) => {},
								RuntimeEvent::Balances(
									$crate::pallet_balances::Event::Burned { who: sender, amount }
								) => {},
							]
						);
					});

					// Receive XCM message in Destination Parachain
					<$receiver_para>::execute_with(|| {
						type RuntimeEvent = <$receiver_para as $crate::Chain>::RuntimeEvent;

						assert_expected_events!(
							$receiver_para,
							vec![
								RuntimeEvent::Balances(
									$crate::pallet_balances::Event::Minted { who: receiver, .. }
								) => {},
								RuntimeEvent::MessageQueue(
									$crate::pallet_message_queue::Event::Processed { success: true, .. }
								) => {},
							]
						);
					});

					// Check if balances are updated accordingly in Origin and Destination Parachains
					let para_sender_balance_after =
						<$sender_para as $crate::Chain>::account_data_of(sender.clone()).free;
					let para_receiver_balance_after =
						<$receiver_para as $crate::Chain>::account_data_of(receiver.clone()).free;
					let delivery_fees = <$sender_para>::execute_with(|| {
						$crate::asset_test_utils::xcm_helpers::transfer_assets_delivery_fees::<
							<$sender_xcm_config as xcm_executor::Config>::XcmSender,
						>($assets.clone(), fee_asset_item, weight_limit.clone(), beneficiary, para_destination)
					});

					assert_eq!(para_sender_balance_before - $amount - delivery_fees, para_sender_balance_after);
					assert!(para_receiver_balance_after > para_receiver_balance_before);

					// Update sender balance
					para_sender_balance_before = <$sender_para as $crate::Chain>::account_data_of(sender.clone()).free;
				}
			)+
		}
	};
}

#[macro_export]
macro_rules! test_parachain_can_claim_assets {
	( $sender_para:ty, $runtime_call:ty, $network_id:expr, $assets:expr, $amount:expr ) => {
		$crate::paste::paste! {
			let sender = [<$sender_para Sender>]::get();
			let origin = <$sender_para as $crate::Chain>::RuntimeOrigin::signed(sender.clone());
			// Receiver is the same as sender
			let beneficiary: Location =
				$crate::AccountId32 { network: Some($network_id), id: sender.clone().into() }.into();
			let versioned_assets: $crate::VersionedAssets = $assets.clone().into();

			<$sender_para>::execute_with(|| {
				// Assets are trapped for whatever reason.
				// The possible reasons for this might differ from runtime to runtime, so here we just drop them directly.
				<$sender_para as [<$sender_para Pallet>]>::PolkadotXcm::drop_assets(
					&beneficiary,
					$assets.into(),
					&XcmContext { origin: None, message_id: [0u8; 32], topic: None },
				);

				type RuntimeEvent = <$sender_para as $crate::Chain>::RuntimeEvent;
				assert_expected_events!(
					$sender_para,
					vec![
						RuntimeEvent::PolkadotXcm(
							$crate::pallet_xcm::Event::AssetsTrapped { origin: beneficiary, assets: versioned_assets, .. }
						) => {},
					]
				);

				let balance_before = <$sender_para as [<$sender_para Pallet>]>::Balances::free_balance(&sender);

				assert_ok!(<$sender_para as [<$sender_para Pallet>]>::PolkadotXcm::claim_assets(
					origin.clone(),
					bx!(versioned_assets.clone().into()),
					bx!(beneficiary.clone().into()),
				));

				assert_expected_events!(
					$sender_para,
					vec![
						RuntimeEvent::PolkadotXcm(
							$crate::pallet_xcm::Event::AssetsClaimed { origin: beneficiary, assets: versioned_assets, .. }
						) => {},
					]
				);

				let balance_after = <$sender_para as [<$sender_para Pallet>]>::Balances::free_balance(&sender);
				assert_eq!(balance_after, balance_before + $amount);
			});
		}
	};
}
