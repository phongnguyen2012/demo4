// use crate::{mock::*, Error, KittyId};
// use frame_benchmarking::{whitelisted_caller, impl_benchmark_test_suite};
// use frame_support::{assert_noop, assert_ok};
// use frame_system::RawOrigin;

// #[test]
// fn it_works_for_default_value() {
// 	new_test_ext().execute_with(|| {
//     let dna = b"phongnv".to_vec();		// Dispatch a signed extrinsic.
// 		let caller: T::AccountId = whitelisted_caller();
// 	}: create_kitty (RawOrigin::Signed(caller), dna))
//     verify {
//         assert_eq!(KittyId::<T>::get(), 1);
//     }
//     impl_benchmark_test_suite!(Kitties, crate::mock::new_test_ext(), crate::mock::Test);
// }

// #[test]
// fn correct_error_for_none_value() {
// 	new_test_ext().execute_with(|| {
// 		// Ensure the expected error is thrown when no value is present.
// 		assert_noop!(TemplateModule::cause_error(Origin::signed(1)), Error::<Test>::NoneValue);
// 	});
// }
