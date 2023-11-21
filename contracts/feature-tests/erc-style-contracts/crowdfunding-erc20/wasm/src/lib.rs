// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            8
// Async Callback:                       1
// Total number of exported functions:  10

#![no_std]

// Configuration that works with rustc < 1.73.0.
// TODO: Recommended rustc version: 1.73.0 or newer.
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    crowdfunding_erc20
    (
        init => init
        fund => fund
        status => status
        claim => claim
        get_target => target
        get_deadline => deadline
        get_deposit => deposit
        get_erc20_contract_address => erc20_contract_address
        get_total_balance => total_balance
    )
}

multiversx_sc_wasm_adapter::async_callback! { crowdfunding_erc20 }
