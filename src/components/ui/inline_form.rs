#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum InputType {
    #[default]
    Email,
}

impl InputType {
    pub fn to_string(&self) -> &'static str {
        match self {
            InputType::Email => "email",
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
            class: "input m-2 bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block  p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500 {input_class}",
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

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ButtonScheme {
    #[default]
    Custom,
}

impl ButtonScheme {
    pub fn to_string(&self) -> &'static str {
        match self {
            ButtonScheme::Custom => "focus:outline-none text-white bg-slate-700 hover:bg-slate-950 focus:ring-4 focus:ring-red-300 font-medium rounded-lg text-sm me-2 mb-2 dark:bg-red-600 dark:hover:bg-red-700 dark:focus:ring-red-900 cursor-pointer",
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ButtonType {
    #[default]
    Submit,
}

impl ButtonType {
    pub fn to_string(&self) -> &'static str {
        match self {
            ButtonType::Submit => "submit",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct InlineFormProps {
    // Required props
    pub action: String,
    pub input_name: String,
    pub button_text: String,
    
    // Optional props with defaults
    #[props(default)]
    pub method: String,
    #[props(default = Some(InputType::Email))]
    pub input_type: Option<InputType>,
    #[props(default = Some(InputSize::Large))]
    pub input_size: Option<InputSize>,
    #[props(default)]
    pub input_id: Option<String>,
    #[props(default)]
    pub input_label_class: Option<String>,
    #[props(default)]
    pub input_value: Option<String>,
    #[props(default)]
    pub input_label: Option<String>,
    #[props(default)]
    pub input_help_text: Option<String>,
    #[props(default = Some("Enter your email".to_string()))]
    pub input_placeholder: Option<String>,
    #[props(default)]
    pub input_step: Option<String>,
    #[props(default = Some(true))]
    pub input_required: Option<bool>,
    #[props(default)]
    pub input_disabled: Option<bool>,
    #[props(default)]
    pub input_readonly: Option<bool>,
    #[props(default = Some(ButtonScheme::Custom))]
    pub button_scheme: Option<ButtonScheme>,
    #[props(default = Some(ButtonType::Submit))]
    pub button_type: Option<ButtonType>,
    #[props(default)]
    pub button_class: Option<String>,
    #[props(default)]
    pub button_disabled: Option<bool>,
    #[props(default)]
    pub on_submit: Option<EventHandler<FormEvent>>,
}

#[component]
pub fn InlineForm(props: InlineFormProps) -> Element {
    let button_type = props.button_type.unwrap_or_default();
    let button_scheme = props.button_scheme.unwrap_or_default();
    
    let button_type_str = button_type.to_string();
    let button_scheme_str = button_scheme.to_string();
    let button_class = props.button_class.clone().unwrap_or_else(|| button_scheme_str.to_string());
    
    rsx!(
        form {
            class: "flex flex-row items-center",
            action: "{props.action}",
            method: "{props.method}",
            onsubmit: move |evt| {
                if let Some(handler) = &props.on_submit {
                    handler.call(evt);
                }
            },
            Input {
                input_type: props.input_type,
                input_size: props.input_size,
                name: props.input_name.clone(),
                id: props.input_id.clone(),
                label_class: props.input_label_class.clone(),
                value: props.input_value.clone(),
                label: props.input_label.clone(),
                help_text: props.input_help_text.clone(),
                placeholder: props.input_placeholder.clone(),
                step: props.input_step.clone(),
                required: props.input_required,
                disabled: props.input_disabled,
                readonly: props.input_readonly,
            }
            button {
                class: "px-5 py-2.5  {button_class}",
                "type": "{button_type_str}",
                disabled: props.button_disabled,
                "{props.button_text}"
            }
        }
    )
}


