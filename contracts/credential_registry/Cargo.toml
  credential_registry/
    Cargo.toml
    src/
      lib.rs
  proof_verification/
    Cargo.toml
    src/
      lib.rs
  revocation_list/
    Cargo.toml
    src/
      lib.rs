[package]
name = "credential_registry"
version = "0.1.0"
edition = "2021"
license = "MIT"

[lib]
crate-type = ["cdylib"]

[dependencies]
casper-contract = "2"
casper-types = "2"

[profile.release]
lto = true
opt-level = "z"
codegen-units = 1