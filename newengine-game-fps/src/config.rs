use abi_stable::std_types::{RString, RVec};
use newengine_plugin_api::{ConfigApplyResultV1, ConfigBlobV1, ConfigDiagV1};

const CONFIG_CONTENT_TYPE: &str = "application/json";
const EMPTY_CONFIG: &[u8] = b"{}";
const CONFIG_FORMAT_VERSION: u32 = 1;

pub(crate) fn default_config() -> ConfigBlobV1 {
    ConfigBlobV1 {
        content_type: RString::from(CONFIG_CONTENT_TYPE),
        bytes: RVec::from(EMPTY_CONFIG.to_vec()),
        format_version: CONFIG_FORMAT_VERSION,
    }
}

pub(crate) fn apply_noop_config(base: &ConfigBlobV1) -> ConfigApplyResultV1 {
    ConfigApplyResultV1 {
        effective: base.clone(),
        diags: RVec::<ConfigDiagV1>::new(),
        changed: false,
    }
}
