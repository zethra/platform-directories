#[macro_use]
extern crate neon;

use neon::prelude::*;
use directories::{BaseDirs, ProjectDirs, UserDirs};
use std::path::PathBuf;

fn base_dirs(mut cx: FunctionContext) -> JsResult<JsObject> {
    let bds = BaseDirs::new().expect("Home directory could not be found");
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

fn project_dirs(mut cx: FunctionContext) -> JsResult<JsObject> {
    let pds = if cx.len() == 3 {
        let qualifier = cx.argument::<JsString>(0)?.value();
        let organization = cx.argument::<JsString>(1)?.value();
        let application = cx.argument::<JsString>(2)?.value();
        ProjectDirs::from(&qualifier, &organization, &application).expect("Home directory could not be found")
    } else {
        let path = cx.argument::<JsString>(0)?.value();
        ProjectDirs::from_path(PathBuf::from(&path)).expect("Home directory could not be found")
    };
    let js_object = JsObject::new(&mut cx);

    let dir = cx.string(pds.project_path().to_string_lossy());
    js_object.set(&mut cx, "projectPath", dir)?;

    let dir = cx.string(pds.cache_dir().to_string_lossy());
    js_object.set(&mut cx, "cacheDir", dir)?;

    let dir = cx.string(pds.config_dir().to_string_lossy());
    js_object.set(&mut cx, "configDir", dir)?;

    let dir = cx.string(pds.data_dir().to_string_lossy());
    js_object.set(&mut cx, "dataDir", dir)?;

    let dir = cx.string(pds.data_local_dir().to_string_lossy());
    js_object.set(&mut cx, "dataLocalDir", dir)?;

    let dir: Handle<'_, JsValue> = pds.runtime_dir()
        .map_or(cx.null().upcast(), |p| cx.string(p.to_string_lossy()).upcast());
    js_object.set(&mut cx, "runtimeDir", dir)?;

    Ok(js_object)
}

fn user_dirs(mut cx: FunctionContext) -> JsResult<JsObject> {
    let uds = UserDirs::new().expect("Home directory could not be found");
    let js_object = JsObject::new(&mut cx);

    let dir = cx.string(uds.home_dir().to_string_lossy());
    js_object.set(&mut cx, "homeDir", dir)?;

    let dir: Handle<'_, JsValue> = uds.audio_dir()
        .map_or(cx.null().upcast(), |p| cx.string(p.to_string_lossy()).upcast());
    js_object.set(&mut cx, "audioDir", dir)?;

    let dir: Handle<'_, JsValue> = uds.desktop_dir()
        .map_or(cx.null().upcast(), |p| cx.string(p.to_string_lossy()).upcast());
    js_object.set(&mut cx, "desktopDir", dir)?;

    let dir: Handle<'_, JsValue> = uds.document_dir()
        .map_or(cx.null().upcast(), |p| cx.string(p.to_string_lossy()).upcast());
    js_object.set(&mut cx, "documentDir", dir)?;

    let dir: Handle<'_, JsValue> = uds.download_dir()
        .map_or(cx.null().upcast(), |p| cx.string(p.to_string_lossy()).upcast());
    js_object.set(&mut cx, "downloadDir", dir)?;

    let dir: Handle<'_, JsValue> = uds.picture_dir()
        .map_or(cx.null().upcast(), |p| cx.string(p.to_string_lossy()).upcast());
    js_object.set(&mut cx, "pictureDir", dir)?;

    let dir: Handle<'_, JsValue> = uds.template_dir()
        .map_or(cx.null().upcast(), |p| cx.string(p.to_string_lossy()).upcast());
    js_object.set(&mut cx, "tempalateDir", dir)?;

    let dir: Handle<'_, JsValue> = uds.video_dir()
        .map_or(cx.null().upcast(), |p| cx.string(p.to_string_lossy()).upcast());
    js_object.set(&mut cx, "videoDir", dir)?;

    let dir: Handle<'_, JsValue> = uds.public_dir()
        .map_or(cx.null().upcast(), |p| cx.string(p.to_string_lossy()).upcast());
    js_object.set(&mut cx, "publicDir", dir)?;

    let dir: Handle<'_, JsValue> = uds.font_dir()
        .map_or(cx.null().upcast(), |p| cx.string(p.to_string_lossy()).upcast());
    js_object.set(&mut cx, "fontDir", dir)?;

    Ok(js_object)
}

register_module!(mut cx, {
    cx.export_function("baseDirs", base_dirs)?;
    cx.export_function("projectDirs", project_dirs)?;
    cx.export_function("userDirs", user_dirs)?;
    Ok(())
});
