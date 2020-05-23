#![cfg_attr(feature = "sgx_env_ver", no_std)]
#![cfg_attr(feature = "sgx_env_ver", feature(rustc_private))]

#[macro_use]

#[cfg(feature = "sgx_env_ver")]
extern crate sgx_tstd as std;

pub mod ast;
pub mod type_checker;
pub mod interpreter;
