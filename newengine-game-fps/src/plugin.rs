use crate::config::{apply_noop_config, default_config};
use crate::constants::{PLUGIN_ID, PLUGIN_NAME, SERVICE_VERSION};
use crate::metadata::{service_id, service_registration_metadata};
use crate::service::FpsGameModuleDescriptorService;
use abi_stable::std_types::{RResult, RString, RVec};
use newengine_plugin_api::{
    ConfigApplyResultV1, ConfigBlobV1, ConfigDiagV1, ConfigPatchV1, HostApiV1, PluginDescriptor,
    PluginKind, PluginModule, ServiceV1Dyn,
};

pub(crate) struct FpsGamePlugin;

pub(crate) fn descriptor_v2() -> newengine_plugin_api::PluginDescriptorV2 {
    newengine_plugin_api::PluginDescriptorV2::builder(
        PLUGIN_ID,
        PLUGIN_NAME,
        env!("CARGO_PKG_VERSION"),
        PluginKind::Runtime,
    )
    .provides_service(
        service_id(),
        SERVICE_VERSION,
        service_registration_metadata(),
    )
    .build()
}

impl PluginModule for FpsGamePlugin {
    fn descriptor(&self) -> PluginDescriptor {
        PluginDescriptor::builder(
            PLUGIN_ID,
            PLUGIN_NAME,
            env!("CARGO_PKG_VERSION"),
            PluginKind::Runtime,
        )
        .provides_service(
            service_id(),
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
        let service = ServiceV1Dyn::from_value(
            FpsGameModuleDescriptorService,
            abi_stable::sabi_trait::TD_Opaque,
        );
        (host.register_service_v1)(service)
    }

    fn start(&mut self) -> RResult<(), RString> {
        RResult::ROk(())
    }
    fn fixed_update(&mut self, _dt: f32) -> RResult<(), RString> {
        RResult::ROk(())
    }
    fn update(&mut self, _dt: f32) -> RResult<(), RString> {
        RResult::ROk(())
    }
    fn render(&mut self, _dt: f32) -> RResult<(), RString> {
        RResult::ROk(())
    }
    fn shutdown(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use newengine_game_module_api::GAME_MODULE_SERVICE_ID;
    use newengine_plugin_api::CapabilityRole;

    #[test]
    fn descriptor_declares_only_game_module_descriptor_service() {
        let descriptor = FpsGamePlugin.descriptor();
        assert!(descriptor.capabilities.iter().any(|capability| {
            capability.id.as_str() == GAME_MODULE_SERVICE_ID
                && capability.role == CapabilityRole::Provides
        }));
        assert_eq!(
            descriptor
                .capabilities
                .iter()
                .filter(|capability| capability.role == CapabilityRole::Provides)
                .count(),
            1
        );
        assert!(!descriptor
            .capabilities
            .iter()
            .any(|capability| capability.role == CapabilityRole::Requires));
    }
}
