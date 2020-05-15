use amethyst::{
    assets::{
        PrefabData,
        ProgressCounter
    },
    derive::PrefabData,
    ecs::{
        Component,
        Entity
    },
    Error,
    ui::{UiText, UiButton}
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

enum FontSize {
    SMALL = 10,
    MEDIUM = 20,
    LARGE = 35,
    TITLE = 50
}

//#[derive(Deserialize, Serialize, PrefabData, Component)]
//#[serde(default)]
//#[serde(deny_unknown_fields)]
pub enum Menu {
    Layout(Vec<Menu>),
    Label(UiText),
    Button(UiButton)
}

impl Default for Menu {
    fn default() -> Self {
        Self::Layout(vec![])
    }
}
