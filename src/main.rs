use bevy_ecs::prelude::SystemStage;
use console_engine::KeyCode;

use components::{position::Position, velocity::Velocity};
use game::Game;
use resources::axis_inputs::AxisInputs;
use resources::bindings::Bindings;
use systems::reset_axis_input::reset_axial_inputs;
use crate::components::rendering_character::RenderingCharacter;
use crate::resources::mouse_inputs::MouseInputs;

use crate::resources::render_targets::RenderTargets;
use crate::resources::timer::Timer;
use crate::systems::character_renderer::{character_renderer, character_renderer_reset};
use crate::systems::debugger::debugger;
use crate::systems::movement::movement_system;
use crate::systems::reset_mouse_input::reset_mouse_inputs;
use crate::systems::timing::timing_system;

mod game;
mod components;
mod systems;
mod resources;
mod inputmanager;

fn main() {
    let mut game: Game = Game::new(3, 3, 60);
    let mut player_entity =
        game.get_world_mut().spawn();
    player_entity
        .insert(Position { x: 0., y: 0. })
        .insert(Velocity { x: 0.3, y: 0.1 })
        .insert(RenderingCharacter { character:'@' });

    add_resources(&mut game);

    stage_systems(&mut game);


    game.start();
}

fn add_resources(game: &mut Game) {
    game.get_world_mut().insert_resource(Timer::new());
    game.get_world_mut().insert_resource(RenderTargets::new());
    game.get_world_mut().insert_resource(AxisInputs::new());
    game.get_world_mut().insert_resource(Bindings::new());
    game.get_world_mut().insert_resource(MouseInputs::new());
}

fn stage_systems(game: &mut Game) {
    game.add_stage_to_schedule("timing", SystemStage::parallel()
        .with_system(timing_system),
    );
    game.add_stage_to_schedule("update", SystemStage::parallel()
        .with_system(movement_system)
        .with_system(character_renderer_reset)
        .with_system(debugger)
    );
    game.add_stage_to_schedule("pre-render", SystemStage::single_threaded()
        .with_system(character_renderer),
    );
    game.add_stage_to_schedule("post-render", SystemStage::single_threaded()
        .with_system(reset_axial_inputs)
        .with_system(reset_mouse_inputs),
    );
}

#[cfg(test)]
mod tests;