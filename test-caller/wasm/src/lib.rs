// Code generated by the dharitri-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                            4
// Async Callback (empty):               1
// Total number of exported functions:   7

#![no_std]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    test_caller
    (
        init => init
        upgrade => upgrade
        callPayable => call_payable
        callNonPayable => call_non_payable
        callPayableWithParams => call_payable_with_params
        getCalledDataParams => get_called_data_params
    )
}

dharitri_sc_wasm_adapter::async_callback_empty! {}
