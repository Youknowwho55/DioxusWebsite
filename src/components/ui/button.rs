#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ButtonScheme {
    #[default]
    Default,
    Success,
    Outline,
    Warn,
    Danger,
}

impl ButtonScheme {
    pub fn to_string(&self) -> &'static str {
        match self {
            ButtonScheme::Default => "text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm me-2 mb-2 dark:bg-blue-600 dark:hover:bg-blue-700 focus:outline-none dark:focus:ring-blue-800",
            ButtonScheme::Success => "focus:outline-none text-white bg-green-700 hover:bg-green-800 focus:ring-4 focus:ring-green-300 font-medium rounded-lg text-sm me-2 mb-2 dark:bg-green-600 dark:hover:bg-green-700 dark:focus:ring-green-800",
            ButtonScheme::Outline =>"text-blue-700 hover:text-white border border-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm text-center me-2 mb-2 dark:border-blue-500 dark:text-blue-500 dark:hover:text-white dark:hover:bg-blue-500 dark:focus:ring-blue-800",
            ButtonScheme::Warn => "focus:outline-none text-white bg-yellow-400 hover:bg-yellow-500 focus:ring-4 focus:ring-yellow-300 font-medium rounded-lg text-sm me-2 mb-2 dark:focus:ring-yellow-900",
            ButtonScheme::Danger => "focus:outline-none text-white bg-red-700 hover:bg-red-800 focus:ring-4 focus:ring-red-300 font-medium rounded-lg text-sm me-2 mb-2 dark:bg-red-600 dark:hover:bg-red-700 dark:focus:ring-red-900",
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
            ButtonSize::ExtraSmall => "px-3 py-2",
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
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let button_scheme = if props.button_scheme.is_some() {
        props.button_scheme.unwrap()
    } else {
        Default::default()
    };

    let button_type = if props.button_type.is_some() {
        props.button_type.unwrap()
    } else {
        Default::default()
    };
    let button_type = button_type.to_string();

    let button_size = if props.button_size.is_some() {
        props.button_size.unwrap()
    } else {
        Default::default()
    };

    let class = if let Some(class) = props.class {
        class
    } else {
        "".to_string()
    };

    let disabled = if let Some(disabled) = props.disabled {
        if disabled {
            Some(true)
        } else {
            None
        }
    } else {
        None
    };

    let class = format!(
        "btn {} {} {}",
        class,
        button_scheme.to_string(),
        button_size.to_string()
    );

    rsx!(
        button {
            class: "{class}",
            id: props.id,
            disabled,
            "data-drawer-target": props.drawer_trigger,
            "data-modal-target": props.modal_trigger,
            "type": "{button_type}",
            "data-disabled-text": props.disabled_text,
            if let Some(img_src) = props.prefix_image_src {
                img { src: "{img_src}", class: "mr-2", width: "12" }
            }
            {props.children}
            if let Some(img_src) = props.suffix_image_src {
                img { src: "{img_src}", class: "ml-2", width: "12" }
            }
        }
    )
}