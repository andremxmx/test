use iced::{
    widget::{
        button, column, container, row, text,
        horizontal_space, vertical_space, Rule
    },
    alignment, Element, Length, Padding,
};

use std::path::Path;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};

use crate::gui::{app::AstraApp, message::Message, style};

// Función para contar las IPs en el archivo ip.txt
fn count_ips_in_file() -> (bool, usize) {
    let path = Path::new("pool/ip.txt");
    
    if !path.exists() {
        return (false, 0);
    }
    
    match File::open(path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            let count = reader.lines()
                .filter_map(Result::ok)
                .filter(|line| !line.trim().is_empty())
                .count();
            (true, count)
        },
        Err(_) => (true, 0),
    }
}

// Función para contar los puertos en el archivo ports.txt
fn count_ports_in_file() -> (bool, usize) {
    let path = Path::new("pool/ports.txt");
    
    if !path.exists() {
        return (false, 0);
    }
    
    match File::open(path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            let count = reader.lines()
                .filter_map(Result::ok)
                .filter(|line| !line.trim().is_empty())
                .filter(|line| line.trim().parse::<u16>().is_ok())
                .count();
            (true, count)
        },
        Err(_) => (true, 0),
    }
}

// Crea un archivo pool/ip.txt con IPs de ejemplo si no existe
pub fn create_ip_file() -> Result<(), std::io::Error> {
    // Asegurar que el directorio pool existe
    fs::create_dir_all("pool")?;
    
    // Crear el archivo con algunas IPs de ejemplo
    let mut file = File::create("pool/ip.txt")?;
    
    // Escribir algunas IPs de ejemplo (puedes poner IPs reales aquí)
    writeln!(file, "192.168.1.1")?;
    writeln!(file, "192.168.1.2")?;
    writeln!(file, "192.168.1.3")?;
    writeln!(file, "192.168.1.100")?;
    writeln!(file, "192.168.1.254")?;
    
    file.flush()?;
    
    Ok(())
}

// Crea un archivo pool/ports.txt con puertos comunes si no existe
pub fn create_ports_file() -> Result<(), std::io::Error> {
    // Asegurar que el directorio pool existe
    fs::create_dir_all("pool")?;
    
    // Crear el archivo con puertos comunes
    let mut file = File::create("pool/ports.txt")?;
    
    // Puertos comunes para servidores Astra
    writeln!(file, "80")?;
    writeln!(file, "8080")?;
    writeln!(file, "8081")?;
    writeln!(file, "8000")?;
    writeln!(file, "8888")?;
    writeln!(file, "9000")?;
    
    file.flush()?;
    
    Ok(())
}

// Función para calcular el total de combinaciones IP:puerto
pub fn calculate_total_combinations() -> usize {
    let (ip_exists, ip_count) = count_ips_in_file();
    let (port_exists, port_count) = count_ports_in_file();
    
    if ip_exists && port_exists && ip_count > 0 && port_count > 0 {
        ip_count * port_count
    } else {
        0
    }
}

pub fn view(app: &AstraApp) -> Element<'_, Message> {
    // Verificar estado de los archivos
    let (ip_file_exists, ip_count) = count_ips_in_file();
    let (ports_file_exists, port_count) = count_ports_in_file();
    
    // Status text y scanning state
    let status_text = format!("{}", app.get_status());
    let is_scanning = app.is_scanning();
    
    // Total de combinaciones y progreso
    let total_combinations = app.get_total_combinations();
    let checked_combinations = app.get_checked_combinations();
    
    // ¿Están listos los archivos para escanear?
    let files_ready = ip_file_exists && ports_file_exists && ip_count > 0 && port_count > 0;

    // Header con título y descripción
    let header = container(
        column![
            text("Astra Server Scanner").size(36).style(iced::theme::Text::Default),
            text("Escaneo específico de servidores Astra desde archivos IP y puertos").size(18),
        ]
        .spacing(15)
        .align_items(alignment::Alignment::Center)
    )
    .padding(Padding::new(35.0))
    .width(Length::Fill)
    .style(iced::theme::Container::Custom(Box::new(style::HeaderContainer)));

    // Panel informativo de configuración con íconos más visibles
    let config_files_info = container(
        column![
            row![
                text("Archivos de Configuración").size(20).style(iced::theme::Text::Color(style::ACCENT_BLUE)),
                horizontal_space(Length::Fill),
                if !is_scanning {
                    button(
                        row![
                            text("Configuración").size(14),
                            text("⚙").size(16).style(iced::theme::Text::Color(style::ACCENT_BLUE)),
                        ].spacing(8)
                    )
                    .padding([8, 14])
                    .style(iced::theme::Button::Text)
                    .on_press(Message::ViewSettings)
                } else {
                    button(text(" ")).width(0)
                }
            ],
            Rule::horizontal(1),
            vertical_space(Length::Fixed(15.0)),
            
            // Información sobre IP.txt con mejor visualización
            row![
                column![
                    text("Archivo de IPs").size(16).style(iced::theme::Text::Color(style::ACCENT_BLUE)),
                    text("pool/ip.txt").size(12),
                ],
                horizontal_space(Length::Fill),
                container(
                    text(if ip_file_exists {
                        format!("✓ {} direcciones", ip_count)
                    } else {
                        "✗ No existe".to_string()
                    }).size(14)
                )
                .padding([6, 12])
                .style(if ip_file_exists {
                    iced::theme::Container::Custom(Box::new(style::SuccessContainer))
                } else {
                    iced::theme::Container::Custom(Box::new(style::ErrorContainer))
                }),
            ],
            vertical_space(Length::Fixed(15.0)),
            
            // Información sobre ports.txt con mejor visualización
            row![
                column![
                    text("Archivo de puertos").size(16).style(iced::theme::Text::Color(style::ACCENT_BLUE)),
                    text("pool/ports.txt").size(12),
                ],
                horizontal_space(Length::Fill),
                container(
                    text(if ports_file_exists {
                        format!("✓ {} puertos", port_count)
                    } else {
                        "✗ No existe".to_string()
                    }).size(14)
                )
                .padding([6, 12])
                .style(if ports_file_exists {
                    iced::theme::Container::Custom(Box::new(style::SuccessContainer))
                } else {
                    iced::theme::Container::Custom(Box::new(style::ErrorContainer))
                }),
            ],
        ]
        .spacing(10)
        .padding(20)
    )
    .width(Length::Fill)
    .style(iced::theme::Container::Custom(Box::new(style::CardContainer)));

    // Botones para crear archivos con mejor diseño
    let action_buttons = container(
        row![
            if !ip_file_exists || ip_count == 0 {
                button(
                    row![
                        text(if !ip_file_exists { "CREAR ARCHIVO IP.txt" } else { "REGENERAR IP.txt" }).size(16),
                        text("+").size(20).style(iced::theme::Text::Color(style::ACCENT_GREEN)),
                    ].spacing(8)
                )
                .padding([12, 20])
                .style(iced::theme::Button::Secondary)
                .on_press(Message::CreateIPFile)
                .width(Length::FillPortion(1))
            } else {
                button(
                    row![
                        text("ARCHIVO IP.txt LISTO").size(16),
                        text("✓").size(18).style(iced::theme::Text::Color(style::ACCENT_GREEN)),
                    ].spacing(8)
                )
                .padding([12, 20])
                .style(iced::theme::Button::Text)
                .width(Length::FillPortion(1))
            },
            
            horizontal_space(Length::Fixed(20.0)),
            
            if !ports_file_exists || port_count == 0 {
                button(
                    row![
                        text(if !ports_file_exists { "CREAR ARCHIVO ports.txt" } else { "REGENERAR ports.txt" }).size(16),
                        text("+").size(20).style(iced::theme::Text::Color(style::ACCENT_GREEN)),
                    ].spacing(8)
                )
                .padding([12, 20])
                .style(iced::theme::Button::Secondary)
                .on_press(Message::CreatePortsFile)
                .width(Length::FillPortion(1))
            } else {
                button(
                    row![
                        text("ARCHIVO ports.txt LISTO").size(16),
                        text("✓").size(18).style(iced::theme::Text::Color(style::ACCENT_GREEN)),
                    ].spacing(8)
                )
                .padding([12, 20])
                .style(iced::theme::Button::Text)
                .width(Length::FillPortion(1))
            },
        ]
        .padding(10)
    )
    .width(Length::Fill)
    .style(iced::theme::Container::Custom(Box::new(style::CardContainer)));

    // Panel de estadísticas del escaneo
    let stats_panel = container(
        column![
            text("Estado del Escaneo").size(18).style(iced::theme::Text::Color(style::ACCENT_BLUE)),
            Rule::horizontal(1),
            vertical_space(Length::Fixed(15.0)),
            
            row![
                column![
                    text("Servidores").size(14).style(iced::theme::Text::Color(style::STATS_LABEL)),
                    text(format!("{}", app.get_servers().len())).size(24).style(iced::theme::Text::Color(style::ACCENT_GREEN)),
                ]
                .width(Length::FillPortion(1))
                .align_items(alignment::Alignment::Center),
                
                Rule::vertical(1),
                
                column![
                    text("Canales").size(14).style(iced::theme::Text::Color(style::STATS_LABEL)),
                    text(format!("{}", app.get_channels_found())).size(24).style(iced::theme::Text::Color(style::ACCENT_GREEN)),
                ]
                .width(Length::FillPortion(1))
                .align_items(alignment::Alignment::Center),
                
                Rule::vertical(1),
                
                column![
                    text("Progreso").size(14).style(iced::theme::Text::Color(style::STATS_LABEL)),
                    text(format!("{:.1}%", app.get_progress())).size(24).style(iced::theme::Text::Color(style::ACCENT_BLUE)),
                ]
                .width(Length::FillPortion(1))
                .align_items(alignment::Alignment::Center),
            ],
            
            vertical_space(Length::Fixed(20.0)),
            
            if total_combinations > 0 {
                column![
                    row![
                        text("Combinaciones totales:").size(14),
                        horizontal_space(Length::Fill),
                        text(format!("{}", total_combinations)).size(14).style(iced::theme::Text::Color(style::STATS_NUMBER)),
                    ],
                    vertical_space(Length::Fixed(8.0)),
                    row![
                        text("Combinaciones revisadas:").size(14),
                        horizontal_space(Length::Fill),
                        text(format!("{} ({:.1}%)", 
                            checked_combinations, 
                            (checked_combinations as f32 / total_combinations as f32) * 100.0)
                        ).size(14).style(iced::theme::Text::Color(style::STATS_NUMBER)),
                    ],
                ]
                .spacing(8)
            } else if !is_scanning && ip_file_exists && ports_file_exists && ip_count > 0 && port_count > 0 {
                column![
                    text(format!("Listo para escanear {} combinaciones", ip_count * port_count))
                    .size(16)
                    .horizontal_alignment(iced::alignment::Horizontal::Center)
                    .style(iced::theme::Text::Color(style::ACCENT_BLUE)),
                ]
            } else {
                column![]
            },
        ]
        .spacing(12)
        .padding(20)
    )
    .width(Length::Fill)
    .style(iced::theme::Container::Custom(Box::new(style::StatsContainer)));

    // Botón principal de escaneo con mejor diseño
    let scan_button = container(
        if is_scanning {
            // Botón para detener el escaneo
            button(
                row![
                    text("DETENER ESCANEO").size(18).style(iced::theme::Text::Default),
                    text("⏹").size(20),
                ].spacing(10)
            )
            .padding([18, 40])
            .style(iced::theme::Button::Destructive)
            .on_press(Message::StopScan)
            .width(Length::Fill)
        } else if !files_ready {
            // Botón para crear archivos de configuración primero
            button(
                row![
                    text("CREAR ARCHIVOS DE CONFIGURACIÓN").size(18).style(iced::theme::Text::Default),
                    text("⚙").size(20),
                ].spacing(10)
            )
            .padding([18, 40])
            .style(iced::theme::Button::Secondary)
            .on_press(Message::ViewSettings)
            .width(Length::Fill)
        } else {
            // Botón para iniciar el escaneo
            button(
                row![
                    text("INICIAR ESCANEO").size(18).style(iced::theme::Text::Default),
                    text("▶").size(20).style(iced::theme::Text::Color(style::ACCENT_GREEN)),
                ].spacing(10)
            )
            .padding([18, 40])
            .style(iced::theme::Button::Primary)
            .on_press(Message::StartAstraServerScan)
            .width(Length::Fill)
        }
    )
    .padding(20)
    .width(Length::Fill)
    .center_x()
    .style(iced::theme::Container::Custom(Box::new(style::CardContainer)));

    // Barra de estado con progreso
    let status_bar = container(
        column![
            row![
                text("Estado:").size(16).style(iced::theme::Text::Color(style::ACCENT_BLUE)),
                horizontal_space(Length::Fixed(8.0)),
                text(&status_text).size(16),
            ],
            vertical_space(Length::Fixed(12.0)),
            crate::gui::widgets::progress(app.get_progress() / 100.0).height(24.0),
        ]
        .spacing(8)
        .padding(20)
    )
    .width(Length::Fill)
    .style(iced::theme::Container::Custom(Box::new(style::InfoContainer)));

    // Estructura principal
    container(
        column![
            header,
            
            container(
                column![
                    vertical_space(Length::Fixed(20.0)),
                    config_files_info,
                    
                    vertical_space(Length::Fixed(20.0)),
                    action_buttons,
                    
                    vertical_space(Length::Fixed(20.0)),
                    stats_panel,
                    vertical_space(Length::Fixed(20.0)),
                    scan_button,
                    vertical_space(Length::Fixed(20.0)),
                    status_bar,
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