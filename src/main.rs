use bevy::{prelude::*, window::WindowResolution, render::texture::ImageSampler};
use bevy_asset_loader::prelude::*;

mod plugins;
use plugins::{
    atlas::AtlasPlugin, game::GamePlugin
};
use bevy_egui::{egui, EguiContext, EguiContexts, EguiPlugin};

mod states;
use states::AppState;

mod assets;
use assets::GameAssets;

mod style;

fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(ImagePlugin {
                // 像素画放大后仍保证清晰
                default_sampler: ImageSampler::nearest_descriptor(),
            })
            .set(WindowPlugin {
                //设置窗口大小 1100*750
                primary_window: Some(Window {
                    #[cfg(target_os = "windows")]
                    position: WindowPosition::Centered(MonitorSelection::Primary), //窗口居中
                    resolution: WindowResolution::new(1000.0, 700.0),
                    ..default()
                }),
                ..default()
            }),
    )
    .add_state::<AppState>()
    .add_loading_state(
        LoadingState::new(AppState::LoadingAssets)
            .continue_to_state(AppState::BuildingAtlases)
    )
    .add_collection_to_loading_state::<_, GameAssets>(AppState::LoadingAssets)
    .add_plugins(AtlasPlugin)
    .add_plugins(GamePlugin)
    .insert_resource(ClearColor(Color::GRAY))
    .add_systems(Startup, initcreate);

    app.run();
}


fn initcreate(
    mut contexts: EguiContexts
){

    let ctx = contexts.ctx_mut();
    style::set_style(ctx, style::Theme::light());
}