use polkadot_runtime_test::perform_runtime_upgrade;
use frame_support::traits::{GetPalletVersion, PalletVersion};
use polkadot_runtime::Runtime;

macro_rules! assert_pallet_versions {
    ($($module:ty),*) => {{
        $(
            let storage_version = <$module>::storage_version();
            println!("{:?}", storage_version.is_some());
            assert_eq!(Some(PalletVersion::new(2, 0, 0)), storage_version);
        )*
    }};
}

#[test]
fn test_pallet_version() {
    let node = perform_runtime_upgrade(polkadot_runtime::WASM_BINARY.unwrap().to_vec());

    node.with_state(|| {
        assert_pallet_versions!(
            frame_system::Module::<Runtime>,
            pallet_randomness_collective_flip::Module::<Runtime>,
            pallet_scheduler::Module::<Runtime>,
            pallet_babe::Module::<Runtime>,
            pallet_timestamp::Module::<Runtime>,
            pallet_indices::Module::<Runtime>,
            pallet_balances::Module::<Runtime>,
            pallet_transaction_payment::Module::<Runtime>,
            pallet_authorship::Module::<Runtime>,
            pallet_staking::Module::<Runtime>,
            pallet_offences::Module::<Runtime>,
            pallet_session::Module::<Runtime>,
            pallet_grandpa::Module::<Runtime>,
            pallet_im_online::Module::<Runtime>,
            pallet_authority_discovery::Module::<Runtime>,
            pallet_democracy::Module::<Runtime>,
            pallet_elections_phragmen::Module::<Runtime>,
            pallet_treasury::Module::<Runtime>,
            pallet_vesting::Module::<Runtime>,
            pallet_utility::Module::<Runtime>,
            pallet_identity::Module::<Runtime>,
            pallet_multisig::Module::<Runtime>
        );
    });
}
