#![allow(non_snake_case)]
use dioxus::prelude::*;
use std::ops::Range;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ButtonScheme {
    #[default]
    Default,
    Success,
    Outline,
    Warn,
    Danger,
    Custom,
}
impl ButtonScheme {
    pub fn to_string(&self) -> &'static str {
        match self {
            ButtonScheme::Default => "text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm me-2 mb-2 dark:bg-blue-600 dark:hover:bg-blue-700 focus:outline-none dark:focus:ring-blue-800 cursor-pointer",
            ButtonScheme::Success => "focus:outline-none text-white bg-green-700 hover:bg-green-800 focus:ring-4 focus:ring-green-300 font-medium rounded-lg text-sm me-2 mb-2 dark:bg-green-600 dark:hover:bg-green-700 dark:focus:ring-green-800 cursor-pointer",
            ButtonScheme::Outline =>"text-blue-700 hover:text-white border border-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm text-center me-2 mb-2 dark:border-blue-500 dark:text-blue-500 dark:hover:text-white dark:hover:bg-blue-500 dark:focus:ring-blue-800 cursor-pointer",
            ButtonScheme::Warn => "focus:outline-none text-white bg-yellow-400 hover:bg-yellow-500 focus:ring-4 focus:ring-yellow-300 font-medium rounded-lg text-sm me-2 mb-2 dark:focus:ring-yellow-900 cursor-pointer",
            ButtonScheme::Danger => "focus:outline-none text-white bg-red-700 hover:bg-red-800 focus:ring-4 focus:ring-red-300 font-medium rounded-lg text-sm me-2 mb-2 dark:bg-red-600 dark:hover:bg-red-700 dark:focus:ring-red-900 cursor-pointer",
            ButtonScheme::Custom => "focus:outline-none text-white bg-customGreen-500 hover:bg-customGreen-600 focus:ring-4 focus:ring-red-300 font-medium rounded-lg text-sm me-2 mb-2 dark:bg-red-600 dark:hover:bg-red-700 dark:focus:ring-red-900 cursor-pointer",
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ButtonType {
    Submit,
    Reset,
    #[default]
    Button,
}

impl ButtonType {
    pub fn to_string(&self) -> &'static str {
        match self {
            ButtonType::Submit => "submit",
            ButtonType::Reset => "reset",
            ButtonType::Button => "button",
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ButtonSize {
    #[default]
    Default,
    Small,
    ExtraSmall,
    Large,
}

impl ButtonSize {
    pub fn to_string(&self) -> &'static str {
        match self {
            ButtonSize::Default => "px-5 py-2.5 ",
            ButtonSize::ExtraSmall => "px-2.5 py-2",
            ButtonSize::Small => "px-3 py-2",
            ButtonSize::Large => "px-5 py-3",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    children: Element,
    id: Option<String>,
    disabled: Option<bool>,
    class: Option<String>,
    prefix_image_src: Option<String>,
    suffix_image_src: Option<String>,
    button_type: Option<ButtonType>,
    button_size: Option<ButtonSize>,
    button_scheme: Option<ButtonScheme>,
    drawer_trigger: Option<String>,
    modal_trigger: Option<String>,
    disabled_text: Option<String>,
    text: Option<String>,
    on_click: EventHandler<MouseEvent>
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let button_scheme = props.button_scheme.unwrap_or_default();
    let button_type = props.button_type.unwrap_or_default();
    let button_size = props.button_size.unwrap_or_default();
    let button_type_str = button_type.to_string();

    let custom_class = props.class.clone().unwrap_or_else(|| "".to_string());
    
    let disabled = props.disabled.unwrap_or(false);
    
    let class = format!(
        "btn {} {} {}",
        custom_class,
        button_scheme.to_string(),
        button_size.to_string()
    );

    let button_text = props.text.clone().unwrap_or_else(|| "".to_string());

    rsx! {
        button {
            class: "{class}",
            id: props.id.clone(),
            disabled: if disabled { Some(true) } else { None },
            "data-drawer-target": props.drawer_trigger.clone(),
            "data-modal-target": props.modal_trigger.clone(),
            r#type: "{button_type_str}",
            "data-disabled-text": props.disabled_text.clone(),
            onclick: move |evt| props.on_click.call(evt),
            if let Some(ref img_src) = props.prefix_image_src {
                img { src: "{img_src}", class: "mr-2", width: "12" }
            }
            {props.children}
            if let Some(ref img_src) = props.suffix_image_src {
                img { src: "{img_src}", class: "ml-2", width: "12" }
            }
            if !button_text.is_empty() {
                "{button_text}"
            }
        }
    }
}