use amethyst::{
    core::Time,
    ecs::{System, WriteExpect, ReadExpect, Read, WriteStorage},
    ui::UiText
};

use crate::states::MenuEntities;

use std::f32::consts::PI;

pub struct MenuBlinkSystem;

impl<'a> System<'a> for MenuBlinkSystem {
    type SystemData = (
        ReadExpect<'a, MenuEntities>,
        WriteStorage<'a, UiText>,
        Read<'a, Time>
    );

    fn run(&mut self, (menu_entities, mut ui_text, time): Self::SystemData) {
        if let Some(blinking_text_entity) = menu_entities.essential_entities.get("text") {
            if let Some(blinking_text) = ui_text.get_mut(*blinking_text_entity) {
                let seconds = time.absolute_real_time().as_secs_f32();
                blinking_text.color[3] = ((PI * 0.5 * seconds.cos()).sin() + 1.0) * 0.5;
            }
        }
    }
}
