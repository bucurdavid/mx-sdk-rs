// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           13
// Async Callback (empty):               1
// Total number of exported functions:  15

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    order_book_pair
    (
        init => init
        createBuyOrder => create_buy_order_endpoint
        createSellOrder => create_sell_order_endpoint
        matchOrders => match_orders_endpoint
        cancelOrders => cancel_orders_endpoint
        cancelAllOrders => cancel_all_orders_endpoint
        freeOrders => free_orders_endpoint
        startGlobalOperation => global_op_start
        stopGlobalOperation => global_op_stop
        getAddressOrderIds => get_address_order_ids
        getOrderIdCounter => order_id_counter
        getOrderById => orders
        getFirstTokenId => first_token_id
        getSecondTokenId => second_token_id
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
