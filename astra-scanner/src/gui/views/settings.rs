use iced::{
    widget::{button, column, container, row, slider, text, vertical_space, scrollable, horizontal_rule, Rule},
    alignment, Element, Length, Padding, theme,
};

use crate::gui::{app::AstraApp, message::Message, style};

pub fn view(app: &AstraApp) -> Element<'_, Message> {
    // Obtenemos las configuraciones
    let config = app.get_config();

    // Header
    let header = container(
        text("Scanner Settings").size(28)
    )
    .padding(Padding::new(20.0))
    .width(Length::Fill)
    .style(iced::theme::Container::Custom(Box::new(style::HeaderContainer)));

    // Basic Scanner Settings
    let basic_settings_title = text("Basic Scanner Settings").size(24);
    
    // Thread count setting
    let thread_count = config.scanner.workers;
    let thread_slider = slider(1..=50, thread_count as i32, |val| Message::ThreadsChanged(val as usize));
    let threads_row = row![
        text("Threads:").size(18).width(Length::FillPortion(2)),
        thread_slider.width(Length::FillPortion(4)),
        text(format!("{}", thread_count)).size(18).width(Length::FillPortion(1)),
    ]
    .spacing(10)
    .padding(10)
    .align_items(alignment::Alignment::Center);
    
    // Timeout setting (in seconds)
    let timeout = config.scanner.timeout;
    let timeout_slider = slider(
        0.1..=10.0, 
        timeout as f32, 
        |t| Message::TimeoutChanged(t as f64)
    ).step(0.1);
    
    let timeout_row = row![
        text("Timeout (seconds):").size(18).width(Length::FillPortion(2)),
        timeout_slider.width(Length::FillPortion(4)),
        text(format!("{:.1}", timeout)).size(18).width(Length::FillPortion(1)),
    ]
    .spacing(10)
    .padding(10)
    .align_items(alignment::Alignment::Center);

    // Create the basic settings section
    let basic_settings = container(
        column![
            basic_settings_title,
            Rule::horizontal(1),
            vertical_space(Length::Fixed(10.0)),
            threads_row,
            timeout_row,
        ]
        .spacing(5)
        .padding(20)
    )
    .width(Length::Fill)
    .style(iced::theme::Container::Custom(Box::new(style::CardContainer)));

    // Advanced Scanner Settings
    let advanced_settings_title = text("Advanced Scanner Settings").size(24);
    
    // Max Workers
    let max_workers = config.scanner.max_workers;
    let max_workers_slider = slider(
        100..=5000, 
        max_workers as i32, 
        |val| Message::MaxWorkersChanged(val as usize)
    ).step(100);
    
    let max_workers_row = row![
        text("Max Workers:").size(18).width(Length::FillPortion(2)),
        max_workers_slider.width(Length::FillPortion(4)),
        text(format!("{}", max_workers)).size(18).width(Length::FillPortion(1)),
    ]
    .spacing(10)
    .padding(5)
    .align_items(alignment::Alignment::Center);
    
    // Chunk Size
    let chunk_size = config.scanner.chunk_size;
    let chunk_size_slider = slider(
        1..=50, 
        chunk_size as i32, 
        |val| Message::ChunkSizeChanged(val as usize)
    );
    
    let chunk_size_row = row![
        text("Chunk Size:").size(18).width(Length::FillPortion(2)),
        chunk_size_slider.width(Length::FillPortion(4)),
        text(format!("{}", chunk_size)).size(18).width(Length::FillPortion(1)),
    ]
    .spacing(10)
    .padding(5)
    .align_items(alignment::Alignment::Center);
    
    // Max Retries
    let max_retries = config.scanner.max_retries;
    let max_retries_slider = slider(
        0..=5, 
        max_retries as i32, 
        |val| Message::MaxRetriesChanged(val as usize)
    );
    
    let max_retries_row = row![
        text("Max Retries:").size(18).width(Length::FillPortion(2)),
        max_retries_slider.width(Length::FillPortion(4)),
        text(format!("{}", max_retries)).size(18).width(Length::FillPortion(1)),
    ]
    .spacing(10)
    .padding(5)
    .align_items(alignment::Alignment::Center);
    
    // Batch Size
    let batch_size = config.scanner.batch_size;
    let batch_size_slider = slider(
        100..=5000, 
        batch_size as i32, 
        |val| Message::BatchSizeChanged(val as usize)
    ).step(100);
    
    let batch_size_row = row![
        text("Batch Size:").size(18).width(Length::FillPortion(2)),
        batch_size_slider.width(Length::FillPortion(4)),
        text(format!("{}", batch_size)).size(18).width(Length::FillPortion(1)),
    ]
    .spacing(10)
    .padding(5)
    .align_items(alignment::Alignment::Center);
    
    // Connection Timeout
    let connection_timeout = config.scanner.connection_timeout;
    let connection_timeout_slider = slider(
        0.1..=5.0, 
        connection_timeout as f32, 
        |val| Message::ConnectionTimeoutChanged(val as f64)
    ).step(0.1);
    
    let connection_timeout_row = row![
        text("Connection Timeout:").size(18).width(Length::FillPortion(2)),
        connection_timeout_slider.width(Length::FillPortion(4)),
        text(format!("{:.1}s", connection_timeout)).size(18).width(Length::FillPortion(1)),
    ]
    .spacing(10)
    .padding(5)
    .align_items(alignment::Alignment::Center);
    
    // Playlist Timeout
    let playlist_timeout = config.scanner.playlist_timeout;
    let playlist_timeout_slider = slider(
        1..=30, 
        playlist_timeout as i32, 
        |val| Message::PlaylistTimeoutChanged(val as usize)
    );
    
    let playlist_timeout_row = row![
        text("Playlist Timeout:").size(18).width(Length::FillPortion(2)),
        playlist_timeout_slider.width(Length::FillPortion(4)),
        text(format!("{}s", playlist_timeout)).size(18).width(Length::FillPortion(1)),
    ]
    .spacing(10)
    .padding(5)
    .align_items(alignment::Alignment::Center);
    
    // Channel Timeout
    let channel_timeout = config.scanner.channel_timeout;
    let channel_timeout_slider = slider(
        1..=10, 
        channel_timeout as i32, 
        |val| Message::ChannelTimeoutChanged(val as usize)
    );
    
    let channel_timeout_row = row![
        text("Channel Timeout:").size(18).width(Length::FillPortion(2)),
        channel_timeout_slider.width(Length::FillPortion(4)),
        text(format!("{}s", channel_timeout)).size(18).width(Length::FillPortion(1)),
    ]
    .spacing(10)
    .padding(5)
    .align_items(alignment::Alignment::Center);
    
    // Pool Connections
    let pool_connections = config.scanner.pool_connections;
    let pool_connections_slider = slider(
        10..=200, 
        pool_connections as i32, 
        |val| Message::PoolConnectionsChanged(val as usize)
    ).step(10);
    
    let pool_connections_row = row![
        text("Pool Connections:").size(18).width(Length::FillPortion(2)),
        pool_connections_slider.width(Length::FillPortion(4)),
        text(format!("{}", pool_connections)).size(18).width(Length::FillPortion(1)),
    ]
    .spacing(10)
    .padding(5)
    .align_items(alignment::Alignment::Center);
    
    // Pool Max Size
    let pool_maxsize = config.scanner.pool_maxsize;
    let pool_maxsize_slider = slider(
        10..=200, 
        pool_maxsize as i32, 
        |val| Message::PoolMaxSizeChanged(val as usize)
    ).step(10);
    
    let pool_maxsize_row = row![
        text("Pool Max Size:").size(18).width(Length::FillPortion(2)),
        pool_maxsize_slider.width(Length::FillPortion(4)),
        text(format!("{}", pool_maxsize)).size(18).width(Length::FillPortion(1)),
    ]
    .spacing(10)
    .padding(5)
    .align_items(alignment::Alignment::Center);

    // Create the advanced settings section
    let advanced_settings = container(
        column![
            advanced_settings_title,
            Rule::horizontal(1),
            vertical_space(Length::Fixed(10.0)),
            max_workers_row,
            chunk_size_row,
            max_retries_row,
            batch_size_row,
            connection_timeout_row,
            playlist_timeout_row,
            channel_timeout_row,
            pool_connections_row,
            pool_maxsize_row,
        ]
        .spacing(5)
        .padding(20)
    )
    .width(Length::Fill)
    .style(iced::theme::Container::Custom(Box::new(style::CardContainer)));

    // ASN Settings
    let asn_settings_title = text("ASN Scanner Settings").size(24);
    
    // ASN Max Workers
    let asn_max_workers = config.asn.max_workers;
    let asn_max_workers_slider = slider(
        5..=50, 
        asn_max_workers as i32, 
        |val| Message::AsnMaxWorkersChanged(val as usize)
    );
    
    let asn_max_workers_row = row![
        text("ASN Max Workers:").size(18).width(Length::FillPortion(2)),
        asn_max_workers_slider.width(Length::FillPortion(4)),
        text(format!("{}", asn_max_workers)).size(18).width(Length::FillPortion(1)),
    ]
    .spacing(10)
    .padding(5)
    .align_items(alignment::Alignment::Center);
    
    // ASN API Timeout
    let asn_api_timeout = config.asn.api_timeout;
    let asn_api_timeout_slider = slider(
        1..=30, 
        asn_api_timeout as i32, 
        |val| Message::AsnApiTimeoutChanged(val as usize)
    );
    
    let asn_api_timeout_row = row![
        text("ASN API Timeout:").size(18).width(Length::FillPortion(2)),
        asn_api_timeout_slider.width(Length::FillPortion(4)),
        text(format!("{}s", asn_api_timeout)).size(18).width(Length::FillPortion(1)),
    ]
    .spacing(10)
    .padding(5)
    .align_items(alignment::Alignment::Center);

    // Create the ASN settings section
    let asn_settings = container(
        column![
            asn_settings_title,
            Rule::horizontal(1),
            vertical_space(Length::Fixed(10.0)),
            asn_max_workers_row,
            asn_api_timeout_row,
        ]
        .spacing(5)
        .padding(20)
    )
    .width(Length::Fill)
    .style(iced::theme::Container::Custom(Box::new(style::CardContainer)));

    // Save and load buttons
    let save_button = button(text("Save Settings").size(16))
        .padding([15, 35])
        .style(theme::Button::Primary)
        .on_press(Message::SaveSettings);
    
    let load_button = button(text("Load Config").size(16))
        .padding([15, 35])
        .style(theme::Button::Secondary)
        .on_press(Message::LoadConfig);
    
    let buttons_row = row![
        save_button,
        load_button,
    ]
    .spacing(20)
    .padding(20)
    .align_items(alignment::Alignment::Center);

    // Instructions
    let instructions = container(
        column![
            text("Settings Info").size(20),
            Rule::horizontal(1),
            vertical_space(Length::Fixed(10.0)),
            text("• These settings will be saved to pool/config.json").size(14),
            text("• Thread count: Number of parallel scan operations").size(14),
            text("• Timeout: Maximum time to wait for a response").size(14),
            text("• Connection Timeout: Time to establish connection").size(14),
            text("• Playlist Timeout: Time to retrieve M3U playlist").size(14),
            text("• Batch Size: Number of IPs to scan in parallel").size(14),
            text("• Changes are applied when you click Save Settings").size(14),
        ]
        .spacing(8)
        .padding(15)
    )
    .width(Length::Fill)
    .style(iced::theme::Container::Custom(Box::new(style::InfoContainer)));

    // Main content - a scrollable container with all settings
    let content = scrollable(
        column![
            basic_settings,
            vertical_space(Length::Fixed(20.0)),
            advanced_settings,
            vertical_space(Length::Fixed(20.0)),
            asn_settings,
            vertical_space(Length::Fixed(20.0)),
            buttons_row,
            vertical_space(Length::Fixed(20.0)),
            instructions,
        ]
        .spacing(10)
        .padding(20)
    )
    .height(Length::Fill)
    .width(Length::Fill);

    // Main container
    container(
        column![
            header,
            content,
        ]
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .style(iced::theme::Container::Custom(Box::new(style::MainContainer)))
    .into()
} 