use iced::{
    widget::{button, Button, text},
    Element, Length,
    theme,
};

use crate::gui::style::{ACCENT_GREEN, ACCENT_RED, LIGHT_TEXT};

/// Crea un botón de acción primaria (verde)
pub fn primary<'a, Message: 'static + Clone>(
    content: impl Into<Element<'a, Message>>,
    on_press: Option<Message>,
) -> Button<'a, Message> {
    let mut btn = button(content)
        .padding([8, 16])
        .width(Length::Shrink);
    
    if let Some(msg) = on_press {
        btn = btn.on_press(msg);
    }
    
    btn.style(theme::Button::Primary)
}

/// Crea un botón de acción secundaria (azul)
pub fn secondary<'a, Message: 'static + Clone>(
    content: impl Into<Element<'a, Message>>,
    on_press: Option<Message>,
) -> Button<'a, Message> {
    let mut btn = button(content)
        .padding([8, 16])
        .width(Length::Shrink);
    
    if let Some(msg) = on_press {
        btn = btn.on_press(msg);
    }
    
    btn.style(theme::Button::Secondary)
}

/// Crea un botón de acción destructiva (rojo)
pub fn destructive<'a, Message: 'static + Clone>(
    content: impl Into<Element<'a, Message>>,
    on_press: Option<Message>,
) -> Button<'a, Message> {
    let mut btn = button(content)
        .padding([8, 16])
        .width(Length::Shrink);
    
    if let Some(msg) = on_press {
        btn = btn.on_press(msg);
    }
    
    btn.style(theme::Button::Destructive)
}

/// Text button that can be active or inactive
pub fn tab_button<'a, Message: 'static + Clone>(
    label: &str,
    size: u16,
    is_active: bool,
    on_press: Option<Message>
) -> Button<'a, Message> {
    let mut btn = button(text(label).size(size))
        .padding([8, 16])
        .width(Length::Fill);
    
    if let Some(msg) = on_press {
        btn = btn.on_press(msg);
    }
    
    if is_active {
        btn.style(theme::Button::Secondary)
    } else {
        btn.style(theme::Button::Text)
    }
}

/// Crea un botón de texto simple
pub fn text_button<'a, Message: 'static + Clone>(
    content: impl Into<Element<'a, Message>>,
    on_press: Option<Message>,
) -> Button<'a, Message> {
    let mut btn = button(content)
        .padding([8, 16])
        .width(Length::Shrink);
    
    if let Some(msg) = on_press {
        btn = btn.on_press(msg);
    }
    
    btn.style(theme::Button::Text)
}

/// Utility for simple primary button with text
pub fn primary_text<'a, Message: 'static + Clone>(
    label: &str, 
    size: u16,
    on_press: Option<Message>
) -> Button<'a, Message> {
    primary(
        text(label).size(size),
        on_press
    )
}

/// Utility for simple secondary button with text
pub fn secondary_text<'a, Message: 'static + Clone>(
    label: &str, 
    size: u16,
    on_press: Option<Message>
) -> Button<'a, Message> {
    secondary(
        text(label).size(size),
        on_press
    )
}

/// Utility for simple destructive button with text
pub fn destructive_text<'a, Message: 'static + Clone>(
    label: &str, 
    size: u16,
    on_press: Option<Message>
) -> Button<'a, Message> {
    destructive(
        text(label).size(size),
        on_press
    )
} 