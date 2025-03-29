use iced::{
    widget::{column, row, text, text_input, Column},
    Length,
    alignment::Horizontal,
    Element,
};

use crate::gui::{
    message::Message,
    style::{ACCENT_BLUE, DARK_BLUE, LIGHT_TEXT},
};

/// Un componente de entrada de texto con etiqueta
pub fn labeled_input<'a>(
    label: &str,
    placeholder: &str,
    value: &str,
    on_change: impl Fn(String) -> Message + 'static,
) -> Element<'a, Message> {
    let label = text(label)
        .size(16)
        .style(LIGHT_TEXT);
    
    let input = text_input(
        placeholder,
        value,
    )
    .padding(10)
    .size(16)
    .on_input(on_change);
    
    column![
        label,
        input,
    ]
    .spacing(5)
    .width(Length::Fill)
    .into()
}

/// Un componente de entrada de texto con etiqueta en la misma línea
pub fn inline_labeled_input<'a>(
    label: &str,
    placeholder: &str,
    value: &str,
    on_change: impl Fn(String) -> Message + 'static,
) -> Element<'a, Message> {
    let label = text(label)
        .size(16)
        .style(LIGHT_TEXT);
    
    let input = text_input(
        placeholder,
        value,
    )
    .padding(10)
    .size(16)
    .on_input(on_change);
    
    row![
        label.width(Length::FillPortion(1)),
        input.width(Length::FillPortion(3)),
    ]
    .spacing(10)
    .align_items(iced::Alignment::Center)
    .width(Length::Fill)
    .into()
}

/// Campo de texto para IP o rangos de red
pub fn ip_input<'a>(
    value: &str,
    on_change: impl Fn(String) -> Message + 'static,
) -> text_input::TextInput<'a, Message> {
    text_input(
        "IP or CIDR (e.g. 192.168.1.0/24)",
        value,
    )
    .padding(10)
    .size(16)
    .on_input(on_change)
}

/// Campo de texto para rangos de puertos
pub fn port_input<'a>(
    value: &str,
    on_change: impl Fn(String) -> Message + 'static,
) -> text_input::TextInput<'a, Message> {
    text_input(
        "Port Range (e.g. 27015-27020)",
        value,
    )
    .padding(10)
    .size(16)
    .on_input(on_change)
} 