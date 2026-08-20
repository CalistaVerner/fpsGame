use crate::constants::{FPS_RUNTIME_SERVICE_ID, GAME_READY_RUNTIME_SERVICE_ID};
use crate::metadata::service_description;
use abi_stable::std_types::{RResult, RString};
use newengine_plugin_api::{Blob, CapabilityId, MethodName, ServiceV1};
use newengine_project_api::RUNTIME_PROFILE_LAUNCH_METHOD_V1;

type CallServiceV1 = extern "C" fn(CapabilityId, MethodName, Blob) -> RResult<Blob, RString>;

pub(crate) struct FpsRuntimeProfileService {
    call_service_v1: CallServiceV1,
}

impl FpsRuntimeProfileService {
    pub(crate) fn new(call_service_v1: CallServiceV1) -> Self {
        Self { call_service_v1 }
    }
}

impl ServiceV1 for FpsRuntimeProfileService {
    fn id(&self) -> CapabilityId {
        RString::from(FPS_RUNTIME_SERVICE_ID)
    }

    fn describe(&self) -> RString {
        RString::from(service_description())
    }

    fn call(&self, method: MethodName, payload: Blob) -> RResult<Blob, RString> {
        if method.as_str() != RUNTIME_PROFILE_LAUNCH_METHOD_V1 {
            return RResult::RErr(RString::from(format!(
                "unknown runtime-profile method: {}",
                method.as_str()
            )));
        }

        if let Err(error) = newengine_game_module_fps::activate() {
            return RResult::RErr(RString::from(format!("activate FPS game module: {error}")));
        }

        (self.call_service_v1)(
            RString::from(GAME_READY_RUNTIME_SERVICE_ID),
            method,
            payload,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    extern "C" fn unreachable_delegate(
        _capability: CapabilityId,
        _method: MethodName,
        _payload: Blob,
    ) -> RResult<Blob, RString> {
        RResult::RErr(RString::from("delegate should not be called"))
    }

    #[test]
    fn rejects_unknown_method_before_delegation() {
        let service = FpsRuntimeProfileService::new(unreachable_delegate);
        let result = service.call(RString::from("invalid.method"), Blob::new());

        match result {
            RResult::RErr(error) => {
                assert!(error.as_str().contains("unknown runtime-profile method"));
            }
            RResult::ROk(_) => panic!("unknown method should be rejected"),
        }
    }
}
