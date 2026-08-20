use crate::constants::FPS_RUNTIME_SERVICE_ID;
use crate::launch::execute_launch;
use crate::metadata::service_description;
use abi_stable::std_types::{RResult, RString};
use newengine_plugin_api::{Blob, CapabilityId, MethodName, ServiceV1};
use newengine_project_api::RUNTIME_PROFILE_LAUNCH_METHOD_V1;

pub(crate) struct FpsRuntimeProfileService;

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

        match execute_launch(&payload) {
            Ok(result) => RResult::ROk(result),
            Err(error) => RResult::RErr(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_unknown_method_before_launch() {
        let result = FpsRuntimeProfileService.call(RString::from("invalid.method"), Blob::new());

        match result {
            RResult::RErr(error) => {
                assert!(error.as_str().contains("unknown runtime-profile method"));
            }
            RResult::ROk(_) => panic!("unknown method should be rejected"),
        }
    }
}
