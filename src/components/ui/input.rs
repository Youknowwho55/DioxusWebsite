#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum InputType {
    #[default]
    Text,
    Number,
    Email,
    Password,
}

impl InputType {
    pub fn to_string(&self) -> &'static str {
        match self {
            InputType::Text => "text",
            InputType::Number => "number",
            InputType::Email => "email",
            InputType::Password => "password",
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum InputSize {
    #[default]
    Default,
    Small,
    ExtraSmall,
    Large,
    Medium,
}

impl InputSize {
    pub fn to_string(&self) -> &'static str {
        match self {
            InputSize::Default => "input-sm",
            InputSize::ExtraSmall => "input-xs",
            InputSize::Small => "input-sm",
            InputSize::Large => "input-lg",
            InputSize::Medium => "input-md",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct InputProps {
    input_type: Option<InputType>,
    input_size: Option<InputSize>,
    pub name: String,
    pub id: Option<String>,
    pub label_class: Option<String>,
    pub value: Option<String>,
    pub label: Option<String>,
    pub help_text: Option<String>,
    pub placeholder: Option<String>,
    pub step: Option<String>,
    pub required: Option<bool>,
    pub disabled: Option<bool>,
    pub readonly: Option<bool>,
}

#[component]
pub fn Input(props: InputProps) -> Element {
    let input_type = if props.input_type.is_some() {
        props.input_type.unwrap()
    } else {
        Default::default()
    };

    let input_size = if props.input_size.is_some() {
        props.input_size.unwrap()
    } else {
        Default::default()
    };

    let input_type = input_type.to_string();
    let input_size = input_size.to_string();

    let input_class = format!("{} {}", input_type, input_size);

    rsx!(
        match (props.label, props.required) {
            (Some(l), Some(_)) => rsx! {
                label { class: props.label_class, "{l} *" }
            },
            (Some(l), None) => rsx! {
                label { class: props.label_class, "{l}" }
            },
            (None, _) => rsx! {},
        }
        input {
            id: props.id,
            class: "input m-2 input-bordered {input_class}",
            value: props.value,
            required: props.required,
            disabled: props.disabled,
            readonly: props.readonly,
            name: "{props.name}",
            placeholder: props.placeholder,
            step: props.step,
            "type": "{input_type}",
        }
        if let Some(l) = props.help_text {
            label {
                span { class: "label-text-alt", "{l}" }
            }
        }
    )
}




#[component]
pub fn TextInput(
    i_value: String,
    i_placeholder: Option<String>,
    on_input: EventHandler<FormEvent>,
    class: Option<String>,
) -> Element {
    let i_placeholder = i_placeholder.unwrap_or_else(|| "".to_string());
    rsx! {
        input {
            value: "{i_value}",
            class: "input-primary",
            placeholder: "{i_placeholder}",
            oninput: move |event| on_input.call(event),
        }
    }
}

#[component]
pub fn PasswordInput(
    i_value: String,
    i_placeholder: Option<String>,
    on_input: EventHandler<FormEvent>,
    class: Option<String>,
) -> Element {
    let i_placeholder = i_placeholder.unwrap_or_else(|| "".to_string());
    rsx! {
        input {
            r#type: "password",
            value: "{i_value}",
            class: "input-primary",
            placeholder: "{i_placeholder}",
            oninput: move |event| on_input.call(event),
        }
    }
}

pub fn NumberInput(
    i_value: String,
    i_placeholder: Option<String>,
    on_input: EventHandler<FormEvent>,
    class: Option<String>,
    i_min: Option<String>,
    i_max: Option<String>,
    i_step: Option<String>,
) -> Element {
    let i_placeholder = i_placeholder.unwrap_or_else(|| "".to_string());
    rsx! {
        input {
            r#type: "number",
            value: "{i_value}",
            class: "input-primary",
            placeholder: "{i_placeholder}",
            oninput: move |event| on_input.call(event),
            min: i_min.unwrap_or_else(|| "".to_string()),
            max: i_max.unwrap_or_else(|| "".to_string()),
            step: i_step.unwrap_or_else(|| "".to_string()),
        }
    }
}

#[component]
pub fn DateInput(
    i_value: String,
    on_input: EventHandler<FormEvent>,
    class: Option<String>,
) -> Element {
    rsx! {
        input {
            r#type: "date",
            value: "{i_value}",
            class: "input-primary",
            oninput: move |event| on_input.call(event),
        }
    }
}

#[component]
pub fn SelectInput(
    i_value: String,
    on_input: EventHandler<FormEvent>,
    class: Option<String>,
    options: Vec<(String, String)>,
) -> Element {
    rsx! {
        select {
            value: "{i_value}",
            class: "input-select",
            oninput: move |event| on_input.call(event),
            for (value , display) in options {
                option { value, "{display}" }
            }
        }
    }
}