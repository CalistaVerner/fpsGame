use crate::constants::{
    COMPOSITION_ID, FPS_RUNTIME_SERVICE_ID, GAME_READY_RUNTIME_SERVICE_ID, PLUGIN_ID,
    RUNTIME_PROFILE_ID,
};
use newengine_project_api::{PROJECT_RUNTIME_PROFILE_ABI_V1, RUNTIME_PROFILE_LAUNCH_METHOD_V1};
use serde_json::json;
use std::sync::OnceLock;

static SERVICE_DESCRIPTION: OnceLock<String> = OnceLock::new();
static SERVICE_REGISTRATION_METADATA: OnceLock<String> = OnceLock::new();
static REQUIRED_RUNTIME_PROFILE_METADATA: OnceLock<String> = OnceLock::new();

pub(crate) fn service_description() -> &'static str {
    SERVICE_DESCRIPTION
        .get_or_init(|| {
            json!({
                "id": FPS_RUNTIME_SERVICE_ID,
                "contract": PROJECT_RUNTIME_PROFILE_ABI_V1,
                "runtime_profile": RUNTIME_PROFILE_ID,
                "game_module": PLUGIN_ID,
                "methods": [RUNTIME_PROFILE_LAUNCH_METHOD_V1],
                "composition": COMPOSITION_ID,
                "delegates_to": GAME_READY_RUNTIME_SERVICE_ID
            })
            .to_string()
        })
        .as_str()
}

pub(crate) fn service_registration_metadata() -> &'static str {
    SERVICE_REGISTRATION_METADATA
        .get_or_init(|| {
            json!({
                "contract": PROJECT_RUNTIME_PROFILE_ABI_V1,
                "runtime_profile": RUNTIME_PROFILE_ID,
                "game_module": PLUGIN_ID,
                "role": "game-runtime-composition",
                "delegates_to": GAME_READY_RUNTIME_SERVICE_ID
            })
            .to_string()
        })
        .as_str()
}

pub(crate) fn required_runtime_profile_metadata() -> &'static str {
    REQUIRED_RUNTIME_PROFILE_METADATA
        .get_or_init(|| {
            json!({
                "contract": PROJECT_RUNTIME_PROFILE_ABI_V1,
                "runtime_profile": RUNTIME_PROFILE_ID,
                "role": "runtime-profile-delegate"
            })
            .to_string()
        })
        .as_str()
}
