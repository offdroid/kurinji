use std::fs;
use bevy::prelude::*;
use bevy::app::AppExit;
use bevy::ecs::system::ResMut;
use kurinji::{OnActionActive, OnActionEnd, Kurinji, KurinjiPlugin};

fn main() {
    println!("Kurinji Action Events");
    App::build()
        .add_plugins(DefaultPlugins)
        // setup
        .add_plugin(KurinjiPlugin::default())
        .add_startup_system(setup.system())
        .add_system(action_active_events_system.system())
        .add_system(action_end_events_system.system())
        .run();
}
fn setup(mut kurinji: ResMut<Kurinji>) {
    let binding_json = fs::read_to_string("example/config/binding.json")
        .expect("Error! could not open config file");
    kurinji.set_bindings_with_json(&binding_json);
}
fn action_end_events_system(
    mut app_exit_events: EventWriter<AppExit>,
    mut action_end_event: EventReader<OnActionEnd>,
) {
    if let Some(value) = action_end_event.iter().next() {
        if value.action == "QUIT_APP" {
            println!("Quiting...");
            app_exit_events.send(AppExit);
        }
    }
}
fn action_active_events_system(
    mut action_active_event: EventReader<OnActionActive>,
) {
    if let Some(value) = action_active_event.iter().next() {
        if value.action == "JUMP" {
            println!("Jumping...");
        }
        if value.action == "SHOOT" {
            println!("Bang");
        }
        if value.action == "AIM_UP" {
            println!("AIM_UP... [ strength: {}] ", value.strength);
        }
        if value.action == "AIM_DOWN" {
            println!("AIM_DOWN... [ strength: {}] ", value.strength);
        }
        if value.action == "AIM_LEFT" {
            println!("AIM_LEFT... [ strength: {}] ", value.strength);
        }
        if value.action == "AIM_RIGHT" {
            println!("AIM_RIGHT... [ strength: {}] ", value.strength);
        }
    }
}
