use iced::{
    widget::{container, text},
    Element, Length,
};

/// Creates a standard card with a title and content
pub fn standard<'a, Message: 'static>(
    title: &str,
    content: Element<'a, Message>,
) -> Element<'a, Message> 
{
    let title_text = text(title)
        .size(24);
    
    container(
        iced::widget::column![
            title_text,
            iced::widget::vertical_space(10),
            content,
        ]
        .spacing(5)
    )
    .padding(10)
    .width(Length::Fill)
    .into()
}

/// Creates a section card for inputs or configuration
pub fn section<'a, Message: 'static>(
    title: &str,
    content: Element<'a, Message>,
) -> Element<'a, Message> 
{
    let title_text = text(title)
        .size(18);

    container(
        iced::widget::column![
            title_text,
            iced::widget::vertical_space(5),
            content,
        ]
        .spacing(5)
    )
    .padding(10)
    .width(Length::Fill)
    .into()
}

/// Creates a panel with a title and content, usually for controls
pub fn panel<'a, Message: 'static>(
    title: &str, 
    content: Element<'a, Message>,
) -> Element<'a, Message> 
{
    let title_text = text(title)
        .size(16);

    container(
        iced::widget::column![
            title_text,
            iced::widget::vertical_space(5),
            content,
        ]
        .spacing(5)
    )
    .padding(10)
    .width(Length::Fill)
    .into()
} 