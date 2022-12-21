// Code generated by the mx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           27
// Async Callback (empty):               1
// Total number of exported functions:  29

#![no_std]
#![feature(alloc_error_handler, lang_items)]

mx_sc_wasm_adapter::allocator!();
mx_sc_wasm_adapter::panic_handler!();

mx_sc_wasm_adapter::endpoints! {
    abi_tester
    (
        echo_abi_test_type
        echo_enum
        take_managed_type
        multi_result_3
        multi_result_4
        var_args
        multi_result_vec
        optional_arg
        optional_result
        address_vs_h256
        managed_address_vs_byte_array
        esdt_local_role
        esdt_token_payment
        esdt_token_data
        sample_storage_mapper
        item_for_vec
        item_for_array_vec
        item_for_managed_vec
        item_for_array
        item_for_box
        item_for_boxed_slice
        item_for_ref
        item_for_slice
        item_for_option
        payable_egld
        payable_some_token
        payable_any_token
    )
}

mx_sc_wasm_adapter::empty_callback! {}
