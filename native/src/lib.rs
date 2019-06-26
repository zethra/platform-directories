#[macro_use]
extern crate neon;

use neon::prelude::*;
use directories::{BaseDirs};

fn base_dirs(mut cx: FunctionContext) -> JsResult<JsObject> {
    let bds = BaseDirs::new().unwrap();
    let js_object = JsObject::new(&mut cx);

    let dir = cx.string(bds.home_dir().to_string_lossy());
    js_object.set(&mut cx, "homeDir", dir)?;

    let dir = cx.string(bds.data_dir().to_string_lossy());
    js_object.set(&mut cx, "dataDir", dir)?;

    let dir = cx.string(bds.cache_dir().to_string_lossy());
    js_object.set(&mut cx, "cacheDir", dir)?;

    let dir = cx.string(bds.config_dir().to_string_lossy());
    js_object.set(&mut cx, "configDir", dir)?;

    let dir = cx.string(bds.data_local_dir().to_string_lossy());
    js_object.set(&mut cx, "dataLocalDir", dir)?;

    let dir: Handle<'_, JsValue> = bds.executable_dir()
        .map_or(cx.null().upcast(), |p| cx.string(p.to_string_lossy()).upcast());
    js_object.set(&mut cx, "executableDir", dir)?;

    let dir: Handle<'_, JsValue> = bds.runtime_dir()
        .map_or(cx.null().upcast(), |p| cx.string(p.to_string_lossy()).upcast());
    js_object.set(&mut cx, "runtimeDir", dir)?;

    Ok(js_object)
}

register_module!(mut cx, {
    cx.export_function("baseDirs", base_dirs)?;
    Ok(())
});
