use chrono::Local;
use iced::{
    widget::{
        button, column, container, row, scrollable, text, horizontal_space, vertical_space,
        text_input, Rule,
    },
    alignment, Element, Length, Padding,
};

use crate::gui::{
    app::{AstraApp, ResultsView},
    message::Message,
    style,
};

use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn view(app: &AstraApp) -> Element<'_, Message> {
    let results_view = app.get_results_view();
    let servers = app.get_servers();
    let channels_count = app.get_channels_found();
    
    // Header con título y botones de acción
    let header = container(
        row![
            text("Resultados del Escaneo").size(28).style(iced::theme::Text::Default),
            horizontal_space(Length::Fill),
            button(
                text("Exportar Resultados").size(16)
            )
            .padding([10, 20])
            .style(iced::theme::Button::Secondary)
            .on_press(Message::ExportResults),
        ]
        .spacing(20)
        .align_items(alignment::Alignment::Center)
    )
    .padding(Padding::new(25.0))
    .width(Length::Fill)
    .style(iced::theme::Container::Custom(Box::new(style::HeaderContainer)));
    
    // Panel de estadísticas en la parte superior
    let stats_panel = container(
        row![
            column![
                text("RESUMEN").size(18).style(iced::theme::Text::Color(style::ACCENT_BLUE)),
                vertical_space(Length::Fixed(15.0)),
                row![
                    column![
                        text("Servidores").size(14),
                        text(format!("{}", servers.len())).size(22)
                            .style(iced::theme::Text::Color(style::ACCENT_GREEN)),
                    ].spacing(5).align_items(alignment::Alignment::Center),
                    
                    horizontal_space(Length::Fixed(40.0)),
                    
                    column![
                        text("Canales").size(14),
                        text(format!("{}", channels_count)).size(22)
                            .style(iced::theme::Text::Color(style::ACCENT_GREEN)),
                    ].spacing(5).align_items(alignment::Alignment::Center),
                    
                    horizontal_space(Length::Fixed(40.0)),
                    
                    column![
                        text("Combinaciones Escaneadas").size(14),
                        text(format!("{}", app.get_checked_combinations())).size(22)
                            .style(iced::theme::Text::Color(style::ACCENT_BLUE)),
                    ].spacing(5).align_items(alignment::Alignment::Center),
                ].spacing(20),
            ]
            .width(Length::Fill)
            .spacing(10)
            .align_items(alignment::Alignment::Center),
        ]
    )
    .padding(20)
    .width(Length::Fill)
    .style(iced::theme::Container::Custom(Box::new(style::CardContainer)));
    
    // Pestañas para alternar entre servidores y canales - más grandes y claras
    let tabs = container(
        row![
            button(
                container(
                    column![
                        text("SERVIDORES").size(16),
                        text(format!("({} encontrados)", servers.len())).size(12),
                    ]
                    .spacing(5)
                    .align_items(alignment::Alignment::Center)
                )
                .padding(10)
                .width(Length::Fill)
                .center_x()
            )
            .width(Length::FillPortion(1))
            .padding([15, 10])
            .style(if matches!(results_view, ResultsView::Servers) {
                iced::theme::Button::Primary
            } else {
                iced::theme::Button::Secondary
            })
            .on_press(Message::SwitchResultsView(ResultsView::Servers)),
            
            button(
                container(
                    column![
                        text("CANALES").size(16),
                        text(format!("({} encontrados)", channels_count)).size(12),
                    ]
                    .spacing(5)
                    .align_items(alignment::Alignment::Center)
                )
                .padding(10)
                .width(Length::Fill)
                .center_x()
            )
            .width(Length::FillPortion(1))
            .padding([15, 10])
            .style(if matches!(results_view, ResultsView::Channels) {
                iced::theme::Button::Primary
            } else {
                iced::theme::Button::Secondary
            })
            .on_press(Message::SwitchResultsView(ResultsView::Channels)),
        ]
        .spacing(0)
    )
    .width(Length::Fill);
    
    // Contenido según la pestaña seleccionada
    let content = match results_view {
        ResultsView::Servers => view_servers(app),
        ResultsView::Channels => view_channels(app),
    };
    
    // Contenedor principal
    container(
        column![
            header,
            vertical_space(Length::Fixed(15.0)),
            stats_panel,
            vertical_space(Length::Fixed(20.0)),
            tabs,
            content,
        ]
        .spacing(0)
        .padding(20)
        .width(Length::Fill)
        .height(Length::Fill)
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .style(iced::theme::Container::Custom(Box::new(style::MainContainer)))
    .into()
}

// Vista de servidores encontrados
fn view_servers(app: &AstraApp) -> Element<'_, Message> {
    let servers = app.get_servers();
    
    if servers.is_empty() {
        // No servers found
        container(
            column![
                vertical_space(Length::Fill),
                container(
                    column![
                        text("No se encontraron servidores").size(22).style(iced::theme::Text::Color(style::ACCENT_BLUE)),
                        vertical_space(Length::Fixed(15.0)),
                        text("Regresa al dashboard para iniciar un escaneo").size(16),
                    ]
                    .spacing(10)
                    .align_items(alignment::Alignment::Center)
                    .width(Length::Fill)
                )
                .padding(30)
                .style(iced::theme::Container::Custom(Box::new(style::InfoContainer))),
                vertical_space(Length::Fill),
            ]
            .spacing(20)
            .align_items(alignment::Alignment::Center)
            .width(Length::Fill)
        )
        .center_y()
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    } else {
        // Tabla de servidores con cabecera
        let header_row = container(
            row![
                text("Dirección IP").size(16).width(Length::FillPortion(3)),
                text("Puerto").size(16).width(Length::FillPortion(1)),
                text("Servicio").size(16).width(Length::FillPortion(2)),
                text("Descubierto").size(16).width(Length::FillPortion(3)),
                text("Acciones").size(16).width(Length::FillPortion(3)),
            ]
            .spacing(10)
            .padding(15)
        )
        .style(iced::theme::Container::Custom(Box::new(style::ListHeaderContainer)))
        .width(Length::Fill);
        
        // Construir filas de servidores
        let mut server_rows = column![header_row].spacing(5);
        
        for server in servers {
            let row = container(
                row![
                    text(server.ip.to_string()).size(14).width(Length::FillPortion(3)),
                    text(server.port.to_string()).size(14).width(Length::FillPortion(1)),
                    text(&server.service).size(14).width(Length::FillPortion(2)),
                    text(server.discovery_time.format("%Y-%m-%d %H:%M:%S").to_string()).size(14).width(Length::FillPortion(3)),
                    row![
                        // Botón de detalles
                        button(
                            row![
                                text("Detalles").size(14),
                                text("🌎").size(14).style(iced::theme::Text::Color(style::ACCENT_BLUE)),
                            ]
                            .spacing(5)
                            .align_items(alignment::Alignment::Center)
                        )
                        .padding([8, 10])
                        .style(iced::theme::Button::Text)
                        .on_press(Message::ViewServerDetails(server.ip, server.port)),
                        
                        // Botón de descarga de playlist
                        button(
                            row![
                                text("Playlist").size(14),
                                text("⬇").size(14).style(iced::theme::Text::Color(style::ACCENT_GREEN)),
                            ]
                            .spacing(5)
                            .align_items(alignment::Alignment::Center)
                        )
                        .padding([8, 10])
                        .style(iced::theme::Button::Text)
                        .on_press(Message::DownloadServerPlaylist(server.ip, server.port))
                    ]
                    .spacing(10)
                    .width(Length::FillPortion(3)),
                ]
                .spacing(10)
                .padding(15)
                .align_items(alignment::Alignment::Center)
            )
            .style(iced::theme::Container::Custom(Box::new(style::CardContainer)))
            .width(Length::Fill);
            
            server_rows = server_rows.push(row);
        }
        
        // Contenedor con scroll para los resultados
        scrollable(
            container(server_rows)
                .width(Length::Fill)
                .padding(10)
        )
        .height(Length::Fill)
        .width(Length::Fill)
        .into()
    }
}

// Vista de canales encontrados
fn view_channels(app: &AstraApp) -> Element<'_, Message> {
    let channels_count = app.get_channels_found();
    let search_query = app.get_channels_search();
    
    // Intentar leer el archivo de canales
    let all_channels = read_channels_from_file();
    
    // Campo de búsqueda para filtrar canales
    let search_input = container(
        row![
            text("Buscar canales:").size(16),
            horizontal_space(Length::Fixed(15.0)),
            text_input("Ingresa el nombre del canal...", search_query)
                .padding(10)
                .on_input(Message::ChannelsSearchChanged)
                .width(Length::FillPortion(4)),
        ]
        .spacing(10)
        .align_items(alignment::Alignment::Center)
    )
    .padding(15)
    .width(Length::Fill)
    .style(iced::theme::Container::Custom(Box::new(style::InfoContainer)));
    
    if all_channels.is_empty() {
        // No se encontraron canales
        container(
            column![
                search_input,
                vertical_space(Length::Fixed(20.0)),
                vertical_space(Length::Fill),
                container(
                    column![
                        text("No se encontraron canales").size(22).style(iced::theme::Text::Color(style::ACCENT_BLUE)),
                        vertical_space(Length::Fixed(15.0)),
                        text(if channels_count > 0 {
                            format!("Se encontraron {} canales pero no se pudieron leer del archivo", channels_count)
                        } else {
                            "Regresa al dashboard para iniciar un escaneo".to_string()
                        }).size(16),
                    ]
                    .spacing(10)
                    .align_items(alignment::Alignment::Center)
                    .width(Length::Fill)
                )
                .padding(30)
                .style(iced::theme::Container::Custom(Box::new(style::InfoContainer))),
                vertical_space(Length::Fill),
            ]
            .spacing(20)
            .align_items(alignment::Alignment::Center)
            .width(Length::Fill)
        )
        .center_y()
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    } else {
        // Filtrar canales según la búsqueda
        let filtered_channels: Vec<_> = if search_query.is_empty() {
            all_channels
        } else {
            all_channels.into_iter()
                .filter(|(title, _)| {
                    extract_channel_title(title)
                        .to_lowercase()
                        .contains(&search_query.to_lowercase())
                })
                .collect()
        };
        
        // Mostrar resultados de búsqueda
        let search_results_text = if !search_query.is_empty() {
            format!("Mostrando {} canales que coinciden con '{}'", 
                filtered_channels.len(),
                search_query
            )
        } else {
            format!("Mostrando todos los canales ({})", filtered_channels.len())
        };
        
        let search_results_info = container(
            text(search_results_text).size(14)
        )
        .padding(10)
        .style(iced::theme::Container::Custom(Box::new(style::InfoContainer)));
        
        // Tabla de canales con cabecera
        let header_row = container(
            row![
                text("Nombre del Canal").size(16).width(Length::FillPortion(5)),
                text("URL").size(16).width(Length::FillPortion(6)),
                text("Acciones").size(16).width(Length::FillPortion(1)),
            ]
            .spacing(10)
            .padding(15)
        )
        .style(iced::theme::Container::Custom(Box::new(style::ListHeaderContainer)))
        .width(Length::Fill);
        
        // Construir filas de canales
        let mut channel_rows = column![header_row].spacing(5);
        
        for (title, url) in filtered_channels {
            let channel_title = extract_channel_title(&title);
            
            let row = container(
                row![
                    text(channel_title).size(14).width(Length::FillPortion(5)),
                    text(&url).size(14).width(Length::FillPortion(6)),
                    button(
                        row![
                            text("▶").size(16).style(iced::theme::Text::Color(style::ACCENT_GREEN))
                        ]
                    )
                    .padding([8, 12])
                    .style(iced::theme::Button::Primary)
                    .on_press(Message::PlayChannel(url.clone()))
                    .width(Length::FillPortion(1)),
                ]
                .spacing(10)
                .padding(15)
                .align_items(alignment::Alignment::Center)
            )
            .style(iced::theme::Container::Custom(Box::new(style::CardContainer)))
            .width(Length::Fill);
            
            channel_rows = channel_rows.push(row);
        }
        
        // Contenedor con scroll para los resultados
        column![
            search_input,
            vertical_space(Length::Fixed(15.0)),
            search_results_info,
            vertical_space(Length::Fixed(10.0)),
            scrollable(
                container(channel_rows)
                    .width(Length::Fill)
                    .padding(10)
            )
            .height(Length::Fill)
            .width(Length::Fill)
        ]
        .spacing(0)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}

// Función para leer los canales del archivo
fn read_channels_from_file() -> Vec<(String, String)> {
    let path = Path::new("channels/all_channels.m3u8");
    
    if !path.exists() {
        return Vec::new();
    }
    
    match File::open(path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            let mut channels = Vec::new();
            let mut current_title = String::new();
            
            for line in reader.lines() {
                if let Ok(line) = line {
                    if line.starts_with("#EXTINF:") {
                        current_title = line;
                    } else if (line.starts_with("http://") || line.starts_with("https://")) && !current_title.is_empty() {
                        channels.push((current_title.clone(), line));
                        current_title = String::new();
                    }
                }
            }
            
            channels
        },
        Err(_) => Vec::new(),
    }
}

// Función para extraer el título del canal de la línea #EXTINF
fn extract_channel_title(extinf_line: &str) -> String {
    if let Some(idx) = extinf_line.find(',') {
        if idx + 1 < extinf_line.len() {
            return extinf_line[idx + 1..].trim().to_string();
        }
    }
    "Canal sin nombre".to_string()
} 