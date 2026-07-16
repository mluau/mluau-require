use crate::{Vfs, VfsBlock};

use super::asset_requirer::*;

fn create_luaurc_with_aliases(aliases: indexmap::IndexMap<String, String>) -> String {
    serde_json::to_string(&serde_json::json!({
        "aliases": aliases
    }))
    .expect("Failed to create luaurc")
}

#[test]
fn test_basic_nested_require() {
    // Create a logger that emits trace log level with env_logger
    env_logger::builder()
        .is_test(true)
        .filter_level(log::LevelFilter::Trace)
        .try_init()
        .expect("Failed to init logger");

    let mut tree = std::collections::HashMap::new();
    tree.insert("init.luau".to_string(), "".to_string());
    tree.insert(
        "test.luau".to_string(),
        "return require('./foo/test')".to_string(),
    );
    tree.insert(
        "foo/test.luau".to_string(),
        "return require('./test2')".to_string(),
    );
    tree.insert(
        "foo/test2.luau".to_string(),
        "return require('./doo/test2')".to_string(),
    );
    tree.insert(
        "foo/doo/test2.luau".to_string(),
        "return require('@dir-alias/bar')".to_string(),
    );

    tree.insert(
        "foo/dir-alias/bar.luau".to_string(),
        "return require('./baz')".to_string(),
    );
    tree.insert(
        "foo/dir-alias/baz.luau".to_string(),
        "return require('@dir-alias/bat')".to_string(),
    );
    tree.insert(
        "foo/dir-alias/bat.luau".to_string(),
        "return require('./baz')".to_string(),
    );
    tree.insert(
        "foo/dir-alias/baz.luau".to_string(),
        "return require('../commacomma')".to_string(),
    );
    tree.insert(
        "foo/commacomma.luau".to_string(),
        "return require('./commacomma2')".to_string(),
    );
    tree.insert(
        "foo/commacomma2.luau".to_string(),
        "return require('../roothelper')".to_string(),
    );
    tree.insert(
        "roothelper.luau".to_string(),
        "return require('./roothelper2')".to_string(),
    );
    tree.insert(
        "roothelper2.luau".to_string(),
        "return require('@dir-alias-2/baz')".to_string(),
    );
    tree.insert(
        "dogs/2/baz.luau".to_string(),
        "return require('../../nextluaurcarea/baz')".to_string(),
    );
    tree.insert(
        "nextluaurcarea/baz.luau".to_string(),
        "return require('@dir-alias-2/chainy')".to_string(),
    );
    tree.insert("dogs/3/chainy.luau".to_string(), "return 3".to_string());

    tree.insert(
        ".luaurc".to_string(),
        create_luaurc_with_aliases(indexmap::indexmap! {
            "dir-alias".to_string() => "./foo/dir-alias".to_string(),
            "dir-alias-2".to_string() => "./dogs/2".to_string()
        }),
    );
    tree.insert(
        "nextluaurcarea/.luaurc".to_string(),
        create_luaurc_with_aliases(indexmap::indexmap! {
            "dir-alias".to_string() => "../foo/dir-alias".to_string(),
            "dir-alias-2".to_string() => "../dogs/3".to_string()
        }),
    );

    let lua = mluau::Lua::new();

    let c = AssetRequirer::new(
        super::memoryvfs::create_memory_vfs_from_map(tree),
        "test".to_string(),
        lua.globals(),
    );

    lua.globals()
        .set("require", lua.create_require_function(c).unwrap()) // Mock require
        .unwrap();

    let l: i32 = match lua
        .load("return require('@self/test')")
        .set_name("/")
        .call(())
    {
        Ok(v) => v,
        Err(e) => {
            println!("Error: {e}");
            panic!("Failed to load test");
        }
    };
    assert_eq!(l, 3);
}

#[test]
fn test_sythivo_a() {
    let main_luau = r#"
local foo = require("./foo/module")

assert(type(foo) == "function")
local res = foo();
assert(type(res) == "table")
print(res.resolved);
return res.resolved
    "#;

    let foo_module_luau = r#"
return function()
return require("./test")
end
    "#;

    let foo_test_luau = r#"
return {
resolved = 246
}
    "#;

    let lua = mluau::Lua::new();

    let mut c = VfsBlock::new();
    c.push_dir("foo".to_string());
    c.push_file("foo/module.luau".to_string(), foo_module_luau.to_string());
    c.push_file("foo/test.luau".to_string(), foo_test_luau.to_string());
    c.push_file("main.luau".to_string(), main_luau.to_string());
    
    let c = AssetRequirer::new(Vfs::new().add(c), "styhivo_abc".to_string(), lua.globals());

    lua.globals()
        .set("require", lua.create_require_function(c).unwrap())
        .unwrap();

    let func = lua
        .load(main_luau)
        .set_name("/main")
        .into_function()
        .unwrap();
    let th = lua.create_thread(func).unwrap();

    let l: u32 = match th.resume(()) {
        Ok(v) => v,
        Err(e) => {
            println!("Error: {e}");
            panic!("Failed to load test");
        }
    };

    assert!(l == 246);
}

#[test]
#[cfg(feature = "rust-embed")]
fn test_reqtest() {
    use rust_embed::Embed;

    #[derive(Embed)]
    #[folder = "src/tests"]
    struct ReqTest;


    let lua = mluau::Lua::new();

    let c = super::create_memory_vfs_from_embedded::<ReqTest>();

    let c = AssetRequirer::new(c, "reqtest".to_string(), lua.globals());

    lua.globals()
        .set("require", lua.create_require_function(c).unwrap())
        .unwrap();

    let l: i32 = match lua
        .load("return require('@self/reqtest/a')")
        .set_name("/")
        .call(())
    {
        Ok(v) => v,
        Err(e) => {
            println!("Error: {e}");
            panic!("Failed to load test");
        }
    };

    assert_eq!(l, 1);
}
