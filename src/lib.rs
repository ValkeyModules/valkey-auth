#[macro_use]
extern crate valkey_module;

use valkey_module::{alloc::ValkeyAlloc, Context, ValkeyResult, ValkeyString, ValkeyValue};

fn valkey_auth(_ctx: &Context, _args: Vec<ValkeyString>) -> ValkeyResult {
    Ok(ValkeyValue::SimpleStringStatic("valkey auth"))
}

valkey_module! {
    name: "valkey-auth",
    version: 1,
    allocator: (ValkeyAlloc, ValkeyAlloc),
    data_types: [],
    commands: [
        ["valkey.auth", valkey_auth, "", 0, 0, 0],
    ]
}
