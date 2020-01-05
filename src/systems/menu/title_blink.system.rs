use amethyst::{
    core::Time,
    ecs::{System, WriteStorage, ReadExpect, Read},
    ui::{UiText}
};

use crate::{
    states::Menu
};

use std::f32::consts::PI;

pub struct TitleBlinkSystem;

impl<'a> System<'a> for TitleBlinkSystem {
    type SystemData = (
        WriteStorage<'a, UiText>,
        ReadExpect<'a, Menu>,
        Read<'a, Time>
    );

    fn run(&mut self, (mut ui_text, menus, time): Self::SystemData) {
        if let Some(blinking_entity) = menus.text.as_ref().unwrap().get("continue_text") {
            let seconds = time.absolute_real_time().as_secs_f32();
            if let Some(blinking_text) = ui_text.get_mut(*blinking_entity) {
                blinking_text.color[3] = ((PI * 0.5 * seconds.cos()).sin() + 1.0) * 0.5;
            }
        }
    }
}
