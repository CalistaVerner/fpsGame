use abi_stable::std_types::RString;
use newengine_plugin_api::Blob;
use newengine_project_api::{GAME_MANIFEST_ENV, PROJECT_LAUNCH_PRESET_ENV};
use serde_json::Value;
use std::path::PathBuf;

pub(crate) fn execute_launch(payload: &Blob) -> Result<Blob, RString> {
    let (manifest_path, launch_id) = decode_launch_request(payload.as_slice())?;

    newengine_game_module_fps::activate()
        .map_err(|error| RString::from(format!("activate FPS game module: {error}")))?;

    match launch_id.as_deref() {
        Some(launch_id) => std::env::set_var(PROJECT_LAUNCH_PRESET_ENV, launch_id),
        None => std::env::remove_var(PROJECT_LAUNCH_PRESET_ENV),
    }
    std::env::set_var(GAME_MANIFEST_ENV, &manifest_path);
    std::env::remove_var("NEWENGINE_PROJECT");
    eprintln!(
        "FPS runtime composition: launching GameReady with explicit module factory {}",
        newengine_game_module_fps::FPS_GAME_MODULE_ID
    );
    let profile = newengine_game_ready_profile::GameReadyRuntimeProfile::standalone_game()
        .with_game_module_factory(newengine_game_module_fps::factory_registration());
    newengine_game_ready_profile::launch_game_ready_profile_with(&manifest_path, profile)
        .map_err(RString::from)?;
    Ok(Blob::new())
}

fn decode_launch_request(payload: &[u8]) -> Result<(PathBuf, Option<String>), RString> {
    let request: Value = serde_json::from_slice(payload)
        .map_err(|error| RString::from(format!("decode runtime launch request: {error}")))?;
    let manifest_path = request
        .get("game_manifest_path")
        .or_else(|| request.get("manifest_path"))
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| RString::from("runtime launch request missing game_manifest_path"))?;
    let launch_id = request
        .get("launch_id")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned);
    Ok((PathBuf::from(manifest_path), launch_id))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_game_launch_request_extracts_manifest_and_launch_id() {
        let (manifest, launch_id) = decode_launch_request(
            br#"{"game_manifest_path":"C:/Games/Fps/game.toml","launch_id":" server "}"#,
        )
        .expect("request should decode");
        assert_eq!(manifest, PathBuf::from("C:/Games/Fps/game.toml"));
        assert_eq!(launch_id.as_deref(), Some("server"));
    }

    #[test]
    fn decode_launch_request_rejects_runtime_toml_as_game_descriptor() {
        let error = decode_launch_request(br#"{"runtime_manifest_path":"C:/Engine/runtime.toml"}"#)
            .expect_err("runtime.toml must never replace game.toml");
        assert!(error.as_str().contains("game_manifest_path"));
    }
}
