pub mod events;
mod game;
mod main_menu;
mod systems;

use bevy::prelude::*;
use game::GamePlugin;
use main_menu::MainMenuPlugin;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        .add_startup_system(spawn_camera)
        .add_system(transition_to_game_state)
        .add_system(transition_to_main_menu_state)
        .add_system(handle_game_over)
        .add_system(exit_game)
        .run();
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Copy, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
