use crate::components::{Echo, Hero, Button, Input};
use crate::components::ui::button::{ButtonSize, ButtonScheme, ButtonType};
use crate::components::ui::input::{InputType, InputSize};

use dioxus::prelude::*;



#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
        Button {
            button_scheme: ButtonScheme::Default,
            button_size: ButtonSize::Default,
            button_type: ButtonType::Submit,
            class: "custom-class".to_string(),
            id: "my-button".to_string(),
            disabled: Some(false),
            children: rsx! { "Click Me!" },
        }
        Input {
            name: "username".to_string(),
            input_type: Some(InputType::Text),
            input_size: Some(InputSize::Medium),
            label: Some("Username".to_string()),
            placeholder: Some("Enter your username".to_string()),
        }
        Input {
            name: "username".to_string(),
            input_type: Some(InputType::Text),
            input_size: Some(InputSize::Medium),
            label: Some("Username".to_string()),
            placeholder: Some("Enter your username".to_string()),
            required: Some(true),
        }
    }
}
