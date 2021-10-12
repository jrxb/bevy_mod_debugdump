use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.set_runner(bevy_mod_debugdump::print_schedule_runner);
    app.run();
}
