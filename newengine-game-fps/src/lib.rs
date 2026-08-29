#![forbid(unsafe_op_in_unsafe_fn)]

mod config;
mod constants;
mod metadata;
mod plugin;
mod service;

use constants::{PLUGIN_ID, PLUGIN_NAME};
use plugin::FpsGamePlugin;

newengine_plugin_api::export_newengine_plugin_signature!(
    id = PLUGIN_ID,
    name = PLUGIN_NAME,
    kind = newengine_plugin_api::PluginKind::Runtime,
    phase = newengine_plugin_api::PluginBootstrapPhase::Engine,
);

newengine_plugin_api::export_newengine_plugin_descriptor_v2!(plugin::descriptor_v2);
newengine_plugin_api::export_newengine_plugin!(module = FpsGamePlugin);
