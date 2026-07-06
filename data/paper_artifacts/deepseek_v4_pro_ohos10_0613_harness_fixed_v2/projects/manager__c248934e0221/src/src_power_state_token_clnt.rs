//! Module: src_power_state_token_clnt
//!
//! Auto-generated skeleton - function bodies are unimplemented.

#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::types::*;
use crate::globals::*;
use crate::compat::*;

fn PowerStateTokenClntConstruct(clnt: *mut crate::types::PowerStateTokenClnt, tokenIf: *mut crate::types::IPowerStateToken) {
    unsafe {
        (*clnt).tokenIf = tokenIf;
        (*clnt).powerState = crate::types::PSM_STATE_INACTIVE;
    }
}

pub extern "C" fn PowerStateTokenClntNewInstance(tokenIf: *mut crate::types::IPowerStateToken)-> *mut crate::types::PowerStateTokenClnt {
    let token_clnt = unsafe {
        OsalMemCalloc(core::mem::size_of::<crate::types::PowerStateTokenClnt>().try_into().unwrap()) as *mut crate::types::PowerStateTokenClnt
    };
    if !token_clnt.is_null() {
        crate::src_power_state_token_clnt::PowerStateTokenClntConstruct(token_clnt, tokenIf);
    }
    token_clnt
}

pub extern "C" fn PowerStateTokenClntFreeInstance(tokenClnt: *mut crate::types::PowerStateTokenClnt) {
    if !tokenClnt.is_null() {
        unsafe {
            OsalMemFree(tokenClnt as *mut ::core::ffi::c_void);
        }
    }
}
