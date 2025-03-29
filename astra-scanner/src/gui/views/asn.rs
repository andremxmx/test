use iced::{
    widget::{
        button, column, container, row, text_input, text,
        horizontal_space, vertical_space, scrollable, Rule,
    },
    alignment, Color, Element, Length, Background, Padding,
};

use crate::gui::{app::AstraApp, message::Message, style};

/// ASN Scanner view - for looking up ASN data by country ISO code
pub fn view(app: &AstraApp) -> Element<'_, Message> {
    // Header with title and description
    let header = container(
        column![
            text("ASN Scanner by Country").size(28),
            text("Find network ranges by country ISO code").size(16),
        ]
        .spacing(8)
        .align_items(alignment::Alignment::Center)
    )
    .padding(Padding::new(20.0))
    .width(Length::Fill)
    .style(iced::theme::Container::Custom(Box::new(style::HeaderContainer)));
    
    // Country ISO code input with improved styling
    let country_input = container(
        column![
            text("Enter Country ISO Code").size(18),
            row![
                text_input("Enter country code (e.g., US, ES, UK)", "")
                    .padding(12)
                    .size(16)
                    .width(Length::FillPortion(3))
                    .on_input(Message::TargetChanged),
                horizontal_space(Length::Fixed(10.0)),
                button(
                    row![
                        text("Lookup ASNs").size(16),
                    ]
                    .spacing(8)
                    .align_items(alignment::Alignment::Center)
                )
                .padding(12)
                .style(iced::theme::Button::Primary)
                .on_press(Message::StartScan),
            ]
            .spacing(12)
            .align_items(alignment::Alignment::Center),
        ]
        .spacing(12)
    )
    .padding(20)
    .style(iced::theme::Container::Custom(Box::new(style::CardContainer)));
    
    // Mock results display with improved styling
    let results_header = row![
        text("ASN").size(16).width(Length::FillPortion(1)),
        text("Organization").size(16).width(Length::FillPortion(3)),
        text("IP Ranges").size(16).width(Length::FillPortion(2)),
        text("Actions").size(16).width(Length::FillPortion(1)),
    ]
    .spacing(10)
    .padding(15)
    .align_items(alignment::Alignment::Center);
    
    let results_placeholder = if app.is_scanning() {
        container(
            column![
                text("Loading ASN data...").size(18),
                text("Please wait while we retrieve information for the specified country.").size(14),
            ]
            .spacing(10)
            .align_items(alignment::Alignment::Center)
        )
    } else {
        container(
            column![
                text("No ASN data loaded yet").size(18),
                text("Enter a country code and click 'Lookup ASNs' to start.").size(14),
            ]
            .spacing(10)
            .align_items(alignment::Alignment::Center)
        )
    };
    
    let results_area = container(
        column![
            container(results_header)
                .style(iced::theme::Container::Custom(Box::new(style::ListHeaderContainer))),
            Rule::horizontal(1),
            scrollable(
                container(results_placeholder)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_y()
                    .padding(30)
            )
            .height(Length::Fill)
        ]
    )
    .padding(1)
    .height(Length::Fill)
    .style(iced::theme::Container::Custom(Box::new(style::CardContainer)));
    
    // Instructions panel with improved styling
    let instructions = container(
        column![
            text("Quick Guide").size(20),
            vertical_space(Length::Fixed(10.0)),
            text("1. Enter a country ISO 2 code (e.g., US, ES, UK, JP)").size(16),
            text("2. Click 'Lookup ASNs' to retrieve all ASNs for that country").size(16),
            text("3. View all networks and IP ranges registered in that country").size(16),
            text("4. Select ASNs to include in your network scan").size(16),
            vertical_space(Length::Fixed(10.0)),
            text("Common ISO Codes:").size(16),
            text("US - United States, UK - United Kingdom, ES - Spain").size(14),
            text("JP - Japan, DE - Germany, FR - France, IT - Italy").size(14),
        ]
        .spacing(8)
    )
    .padding(20)
    .style(iced::theme::Container::Custom(Box::new(style::InfoContainer)));
    
    // Main container
    container(
        column![
            header,
            vertical_space(Length::Fixed(20.0)),
            country_input,
            vertical_space(Length::Fixed(20.0)),
            results_area,
            vertical_space(Length::Fixed(20.0)),
            instructions,
        ]
        .spacing(5)
        .padding(20)
        .width(Length::Fill)
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .style(iced::theme::Container::Custom(Box::new(style::MainContainer)))
    .into()
} 