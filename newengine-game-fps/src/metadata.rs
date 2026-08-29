use crate::constants::{PLUGIN_ID, SERVICE_VERSION};
use newengine_game_module_api::{GAME_MODULE_CONTRACT_V2, GAME_MODULE_SERVICE_ID};
use serde_json::json;
use std::sync::OnceLock;

static SERVICE_REGISTRATION_METADATA: OnceLock<String> = OnceLock::new();

pub(crate) fn service_registration_metadata() -> &'static str {
    SERVICE_REGISTRATION_METADATA
        .get_or_init(|| {
            json!({
                "contract": GAME_MODULE_CONTRACT_V2,
                "module_id": PLUGIN_ID,
                "role": "game-module-descriptor",
                "service_version": SERVICE_VERSION,
                "ownership": "descriptor-only; runtime profile and gameplay implementations are external"
            })
            .to_string()
        })
        .as_str()
}

pub(crate) fn service_id() -> &'static str {
    GAME_MODULE_SERVICE_ID
}
