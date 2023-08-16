// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           13
// Async Callback:                       1
// Total number of exported functions:  15

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!(static64k);
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    erc1155
    (
        init => init
        safeTransferFrom => safe_transfer_from
        safeBatchTransferFrom => safe_batch_transfer_from
        setApprovalForAll => set_approved_for_all
        createToken => create_token
        mint => mint
        burn => burn
        balanceOf => balance_of
        balanceOfBatch => balance_of_batch
        getTokenOwner => token_owner
        getTokenTypeCreator => token_type_creator
        getTokenTypeUri => token_type_uri
        isFungible => is_fungible
        isApprovedForAll => is_approved
    )
}

multiversx_sc_wasm_adapter::async_callback! { erc1155 }
