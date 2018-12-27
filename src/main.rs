extern crate amethyst;

use amethyst::input::{is_close_requested, is_key_down};
use amethyst::prelude::*;
use amethyst::renderer::{
    DisplayConfig, DrawFlat, Event, Pipeline, PosNormTex, RenderBundle, Stage, VirtualKeyCode,
};
use amethyst::ui::UiCreator;
use amethyst::ui::{DrawUi, UiBundle};

use amethyst::core::transform::TransformBundle;

fn main() -> Result<(), amethyst::Error> {
    amethyst::start_logger(Default::default());

    let path = format!(
        "{}/resources/display_config.ron",
        env!("CARGO_MANIFEST_DIR")
    );
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.00196, 0.23726, 0.21765, 1.0], 1.0)
            .with_pass(DrawFlat::<PosNormTex>::new())
            .with_pass(DrawUi::new()),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)))?
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<String, String>::new())?;
    let mut game = Application::new("./", MainState, game_data)?;
    game.run();
    Ok(())
}

pub struct LevelState;

impl<'a, 'b> SimpleState for LevelState {
    fn on_stop(&mut self, data: StateData<GameData>) {
        data.world.delete_all();
    }
    fn on_start(&mut self, data: StateData<GameData>) {
        data.world
            .exec(|mut creator: UiCreator| creator.create("oxygen_bar/prefab.ron", ()));
    }

    fn handle_event(&mut self, data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        println!("LEVEL STATE HANDLE");
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                println!("Quitting");
                return Trans::Quit;
            } else if is_key_down(&event, VirtualKeyCode::Tab) {
                println!("Leaving Level State");
                return Trans::Pop;
            }
        }
        Trans::None
    }
}

pub struct MainState;

impl SimpleState for MainState {
    fn on_start(&mut self, data: StateData<GameData>) {
        println!("started main");
    }

    fn handle_event(&mut self, _: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        println!("handle event");
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            } else if is_key_down(&event, VirtualKeyCode::Tab) {
                return Trans::Push(Box::new(LevelState));
            }
        }
        Trans::None
    }
}
