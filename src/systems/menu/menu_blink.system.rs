use amethyst::{
    core::Time,
    ecs::{System, Read, WriteStorage},
    ui::{UiText, UiFinder}
};

use std::f32::consts::PI;

pub struct MenuBlinkSystem;

impl<'a> System<'a> for MenuBlinkSystem {
    type SystemData = (
        WriteStorage<'a, UiText>,
        UiFinder<'a>,
        Read<'a, Time>
    );

    fn run(&mut self, (mut ui_text, ui_finder, time): Self::SystemData) {
        if let Some(blinking_entity) = ui_finder.find("blink") {
            if let Some(blinking_text) = ui_text.get_mut(blinking_entity) {
                let seconds = time.absolute_real_time().as_secs_f32();
                blinking_text.color[3] = ((PI * 0.5 * seconds.cos()).sin() + 1.0) * 0.5;
            }
        }
    }
}
