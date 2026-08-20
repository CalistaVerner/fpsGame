use crate::config::{apply_noop_config, default_config};
use crate::constants::{FPS_RUNTIME_SERVICE_ID, PLUGIN_ID, PLUGIN_NAME, SERVICE_VERSION};
use crate::metadata::service_registration_metadata;
use crate::service::FpsRuntimeProfileService;
use abi_stable::std_types::{RResult, RString, RVec};
use newengine_plugin_api::{
    ConfigApplyResultV1, ConfigBlobV1, ConfigDiagV1, ConfigPatchV1, HostApiV1, PluginDescriptor,
    PluginKind, PluginModule, ServiceV1Dyn,
};

pub(crate) struct FpsGamePlugin;

impl PluginModule for FpsGamePlugin {
    fn descriptor(&self) -> PluginDescriptor {
        PluginDescriptor::builder(
            PLUGIN_ID,
            PLUGIN_NAME,
            env!("CARGO_PKG_VERSION"),
            PluginKind::Runtime,
        )
        .provides_service(
            FPS_RUNTIME_SERVICE_ID,
            SERVICE_VERSION,
            service_registration_metadata(),
        )
        .build()
    }

    fn config_defaults(&self) -> RResult<ConfigBlobV1, RString> {
        RResult::ROk(default_config())
    }

    fn config_apply_patches(
        &self,
        base: &ConfigBlobV1,
        _patches: RVec<ConfigPatchV1>,
    ) -> RResult<ConfigApplyResultV1, RString> {
        RResult::ROk(apply_noop_config(base))
    }

    fn config_supports_live_update(&self) -> bool {
        false
    }

    fn config_update_live(
        &mut self,
        _effective: &ConfigBlobV1,
    ) -> RResult<RVec<ConfigDiagV1>, RString> {
        RResult::ROk(RVec::new())
    }

    fn init(&mut self, host: HostApiV1, _effective: ConfigBlobV1) -> RResult<(), RString> {
        let service =
            ServiceV1Dyn::from_value(FpsRuntimeProfileService, abi_stable::sabi_trait::TD_Opaque);
        (host.register_service_v1)(service)
    }

    fn start(&mut self) -> RResult<(), RString> {
        ready()
    }

    fn fixed_update(&mut self, _dt: f32) -> RResult<(), RString> {
        ready()
    }

    fn update(&mut self, _dt: f32) -> RResult<(), RString> {
        ready()
    }

    fn render(&mut self, _dt: f32) -> RResult<(), RString> {
        ready()
    }

    fn shutdown(&mut self) {}
}

#[inline]
fn ready() -> RResult<(), RString> {
    RResult::ROk(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use newengine_plugin_api::CapabilityRole;

    #[test]
    fn descriptor_declares_fps_runtime_composition_service() {
        let descriptor = FpsGamePlugin.descriptor();

        assert!(descriptor.capabilities.iter().any(|capability| {
            capability.id.as_str() == FPS_RUNTIME_SERVICE_ID
                && capability.role == CapabilityRole::Provides
        }));
        assert!(!descriptor
            .capabilities
            .iter()
            .any(|capability| capability.role == CapabilityRole::Requires));
    }
}
