use abi_stable::std_types::{RResult, RString};
use newengine_game_module_api::{
    GAME_MODULE_CONTRACT_V2, GAME_MODULE_DESCRIBE_METHOD_V1, GAME_MODULE_DESCRIBE_METHOD_V2,
    GAME_MODULE_SERVICE_ID,
};
use newengine_game_module_fps_contract::{descriptor_v1_compat, descriptor_v2};
use newengine_plugin_api::{Blob, CapabilityId, MethodName, ServiceV1};
use newengine_service_api::SERVICE_METHOD_SHUTDOWN_V1;

use crate::constants::PLUGIN_ID;

pub(crate) struct FpsGameModuleDescriptorService;

impl ServiceV1 for FpsGameModuleDescriptorService {
    fn id(&self) -> CapabilityId {
        RString::from(GAME_MODULE_SERVICE_ID)
    }

    fn describe(&self) -> RString {
        RString::from(
            serde_json::json!({
                "id": GAME_MODULE_SERVICE_ID,
                "contract": GAME_MODULE_CONTRACT_V2,
                "active_module": PLUGIN_ID,
                "methods": [GAME_MODULE_DESCRIBE_METHOD_V2, GAME_MODULE_DESCRIBE_METHOD_V1, SERVICE_METHOD_SHUTDOWN_V1],
                "ownership": "descriptor-only"
            })
            .to_string(),
        )
    }

    fn call(&self, method: MethodName, payload: Blob) -> RResult<Blob, RString> {
        if method.as_str() == SERVICE_METHOD_SHUTDOWN_V1 {
            return RResult::ROk(Blob::from(Vec::new()));
        }
        if method.as_str() != GAME_MODULE_DESCRIBE_METHOD_V2
            && method.as_str() != GAME_MODULE_DESCRIBE_METHOD_V1
        {
            return RResult::RErr(RString::from("unknown game-module method"));
        }
        if !payload.is_empty() {
            if let Ok(request) = serde_json::from_slice::<serde_json::Value>(payload.as_slice()) {
                if let Some(requested) = request
                    .get("requested_module_id")
                    .and_then(serde_json::Value::as_str)
                {
                    if requested != PLUGIN_ID {
                        return RResult::RErr(RString::from(format!(
                            "FPS module descriptor '{}' does not satisfy requested '{}'",
                            PLUGIN_ID, requested
                        )));
                    }
                }
            }
        }
        let encoded = if method.as_str() == GAME_MODULE_DESCRIBE_METHOD_V2 {
            serde_json::to_vec(&descriptor_v2())
        } else {
            serde_json::to_vec(&descriptor_v1_compat())
        };
        match encoded {
            Ok(bytes) => RResult::ROk(Blob::from(bytes)),
            Err(error) => RResult::RErr(RString::from(format!(
                "encode FPS game-module descriptor: {error}"
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn descriptor_is_contract_only_and_valid() {
        let descriptor = descriptor_v2();
        descriptor.validate().unwrap();
        assert_eq!(descriptor.module_id, PLUGIN_ID);
        assert_eq!(descriptor.providers.len(), 4);
        assert!(descriptor
            .providers
            .iter()
            .all(|provider| provider.provider_id.starts_with("newengine.gameplay.fps")));
    }

    #[test]
    fn descriptor_service_accepts_standard_shutdown_v1() {
        let service = FpsGameModuleDescriptorService;
        let result = service.call(
            RString::from(SERVICE_METHOD_SHUTDOWN_V1),
            Blob::from(Vec::new()),
        );
        match result {
            RResult::ROk(payload) => assert!(payload.is_empty()),
            RResult::RErr(error) => panic!("shutdown_v1 rejected: {error}"),
        }
    }
}
