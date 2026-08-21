pub(crate) const PLUGIN_ID: &str = newengine_game_module_fps::FPS_GAME_MODULE_ID;
pub(crate) const PLUGIN_NAME: &str = "NewEngine FPS Game";
pub(crate) const RUNTIME_PROFILE_ID: &str =
    newengine_game_ready_profile::GAME_READY_RUNTIME_PROFILE_ID;
pub(crate) fn fps_runtime_service_id() -> String {
    newengine_project_api::runtime_profile_service_id_for_game(RUNTIME_PROFILE_ID, Some(PLUGIN_ID))
}
pub(crate) const SERVICE_VERSION: u32 = 1;
pub(crate) const COMPOSITION_ID: &str = "fps";
