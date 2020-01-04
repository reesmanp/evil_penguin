use amethyst::{
    prelude::{SimpleState, StateData, GameData, StateEvent, SimpleTrans, Trans},
    input::{VirtualKeyCode, is_key_down}
};

pub struct PausedState;

impl SimpleState for PausedState {
    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(e) = &event {
            if is_key_down(&e, VirtualKeyCode::Escape) {
                return Trans::Pop;
            }
        }

        Trans::None
    }
}
