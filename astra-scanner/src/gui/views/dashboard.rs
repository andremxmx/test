use iced::{
    widget::{
        button, column, container, row, text,
        horizontal_space, vertical_space, Rule
    },
    alignment, Element, Length, Padding,
};

use crate::gui::{app::AstraApp, message::Message, style};

pub fn view(app: &AstraApp) -> Element<'_, Message> {
    // Obtener datos relevantes
    let servers_found = app.get_servers().len();
    let channels_found = app.get_channels_found();
    let progress = app.get_progress();
    let is_scanning = app.is_scanning();
    
    // Header con título y descripción
    let header = container(
        column![
            text("Astra Scanner").size(36).style(iced::theme::Text::Default),
            text("Herramienta de escaneo para servidores de IPTV").size(18),
        ]
        .spacing(15)
        .align_items(alignment::Alignment::Center)
    )
    .padding(Padding::new(35.0))
    .width(Length::Fill)
    .style(iced::theme::Container::Custom(Box::new(style::HeaderContainer)));
    
    // Tarjetas de estadísticas
    let stats_cards = container(
        row![
            // Tarjeta de servidores
            container(
                column![
                    row![
                        text("Servidores").size(18).style(iced::theme::Text::Color(style::ACCENT_BLUE)),
                        horizontal_space(Length::Fill),
                        text("📺").size(24),
                    ],
                    vertical_space(Length::Fixed(20.0)),
                    text(format!("{}", servers_found)).size(36).style(iced::theme::Text::Color(style::STATS_NUMBER)),
                    text("encontrados").size(14).style(iced::theme::Text::Color(style::STATS_LABEL)),
                ]
                .spacing(10)
                .padding(20)
                .align_items(alignment::Alignment::Center)
            )
            .width(Length::FillPortion(1))
            .style(iced::theme::Container::Custom(Box::new(style::CardContainer))),
            
            // Tarjeta de canales
            container(
                column![
                    row![
                        text("Canales").size(18).style(iced::theme::Text::Color(style::ACCENT_BLUE)),
                        horizontal_space(Length::Fill),
                        text("📡").size(24),
                    ],
                    vertical_space(Length::Fixed(20.0)),
                    text(format!("{}", channels_found)).size(36).style(iced::theme::Text::Color(style::STATS_NUMBER)),
                    text("descubiertos").size(14).style(iced::theme::Text::Color(style::STATS_LABEL)),
                ]
                .spacing(10)
                .padding(20)
                .align_items(alignment::Alignment::Center)
            )
            .width(Length::FillPortion(1))
            .style(iced::theme::Container::Custom(Box::new(style::CardContainer))),
            
            // Tarjeta de progreso
            container(
                column![
                    row![
                        text("Progreso").size(18).style(iced::theme::Text::Color(style::ACCENT_BLUE)),
                        horizontal_space(Length::Fill),
                        text("🔍").size(24),
                    ],
                    vertical_space(Length::Fixed(20.0)),
                    text(format!("{:.1}%", progress)).size(36).style(iced::theme::Text::Color(style::STATS_NUMBER)),
                    text(if is_scanning {
                        "escaneando..."
                    } else if progress > 0.0 {
                        "completado"
                    } else {
                        "sin iniciar"
                    }).size(14).style(iced::theme::Text::Color(style::STATS_LABEL)),
                ]
                .spacing(10)
                .padding(20)
                .align_items(alignment::Alignment::Center)
            )
            .width(Length::FillPortion(1))
            .style(iced::theme::Container::Custom(Box::new(style::CardContainer))),
        ]
        .spacing(20)
    )
    .width(Length::Fill)
    .padding(20);
    
    // Sección de modos de escaneo
    let scan_modes_title = container(
        row![
            text("Modos de Escaneo").size(24).style(iced::theme::Text::Color(style::ACCENT_BLUE)),
            horizontal_space(Length::Fill),
            if is_scanning {
                text("Escaneando...").size(16).style(iced::theme::Text::Color(style::ACCENT_RED))
            } else {
                text("")
            },
        ]
        .padding(10)
    )
    .width(Length::Fill)
    .padding([0, 20]);
    
    // Tarjetas de modos de escaneo
    let scan_modes = container(
        row![
            // Tarjeta de Astra Server Scanner
            container(
                column![
                    text("Astra Server Scanner").size(20).style(iced::theme::Text::Color(style::ACCENT_BLUE)),
                    vertical_space(Length::Fixed(10.0)),
                    Rule::horizontal(1),
                    vertical_space(Length::Fixed(15.0)),
                    text("Escanea servidores Astra en un rango de IPs y puertos definidos en archivos.")
                    .size(14)
                    .horizontal_alignment(iced::alignment::Horizontal::Left),
                    vertical_space(Length::Fixed(20.0)),
                    button(
                        row![
                            text("Iniciar Scanner").size(16),
                            horizontal_space(Length::Fixed(5.0)),
                            text("▶").size(16).style(iced::theme::Text::Color(style::ACCENT_GREEN)),
                        ].spacing(8)
                    )
                    .width(Length::Fill)
                    .padding([12, 20])
                    .style(iced::theme::Button::Primary)
                    .on_press(Message::ViewAstraServerScanner)
                ]
                .spacing(8)
                .padding(20)
            )
            .width(Length::FillPortion(1))
            .height(Length::Fixed(250.0))
            .style(iced::theme::Container::Custom(Box::new(style::CardContainer))),
            
            // Tarjeta de URL Scanner
            container(
                column![
                    text("URL Scanner").size(20).style(iced::theme::Text::Color(style::ACCENT_BLUE)),
                    vertical_space(Length::Fixed(10.0)),
                    Rule::horizontal(1),
                    vertical_space(Length::Fixed(15.0)),
                    text("Escanea una URL específica para encontrar canales y playlists disponibles.")
                    .size(14)
                    .horizontal_alignment(iced::alignment::Horizontal::Left),
                    vertical_space(Length::Fixed(20.0)),
                    button(
                        row![
                            text("Iniciar Scanner").size(16),
                            horizontal_space(Length::Fixed(5.0)),
                            text("▶").size(16).style(iced::theme::Text::Color(style::ACCENT_GREEN)),
                        ].spacing(8)
                    )
                    .width(Length::Fill)
                    .padding([12, 20])
                    .style(iced::theme::Button::Primary)
                    .on_press(Message::StartScan)
                ]
                .spacing(8)
                .padding(20)
            )
            .width(Length::FillPortion(1))
            .height(Length::Fixed(250.0))
            .style(iced::theme::Container::Custom(Box::new(style::CardContainer))),
            
            // Tarjeta de Resultados
            container(
                column![
                    text("Resultados").size(20).style(iced::theme::Text::Color(style::ACCENT_BLUE)),
                    vertical_space(Length::Fixed(10.0)),
                    Rule::horizontal(1),
                    vertical_space(Length::Fixed(15.0)),
                    text("Visualiza los servidores y canales encontrados durante el escaneo.")
                    .size(14)
                    .horizontal_alignment(iced::alignment::Horizontal::Left),
                    vertical_space(Length::Fixed(20.0)),
                    button(
                        row![
                            text("Ver Resultados").size(16),
                            horizontal_space(Length::Fixed(5.0)),
                            text("→").size(16).style(iced::theme::Text::Color(style::ACCENT_BLUE)),
                        ].spacing(8)
                    )
                    .width(Length::Fill)
                    .padding([12, 20])
                    .style(iced::theme::Button::Secondary)
                    .on_press(Message::ViewResults)
                ]
                .spacing(8)
                .padding(20)
            )
            .width(Length::FillPortion(1))
            .height(Length::Fixed(250.0))
            .style(iced::theme::Container::Custom(Box::new(style::CardContainer))),
        ]
        .spacing(20)
    )
    .width(Length::Fill)
    .padding(20);
    
    // Estructura principal
    container(
        column![
            header,
            
            container(
                column![
                    vertical_space(Length::Fixed(20.0)),
                    stats_cards,
                    vertical_space(Length::Fixed(20.0)),
                    scan_modes_title,
                    scan_modes,
                    vertical_space(Length::Fixed(20.0)),
                ]
                .spacing(0)
                .padding(10)
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .style(iced::theme::Container::Custom(Box::new(style::MainContainer))),
        ]
        .spacing(0)
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
} 