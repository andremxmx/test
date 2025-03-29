use std::net::IpAddr;
use crate::gui::app::{View, ResultsView};
use crate::config::Config;

/// Mensajes para la aplicación Iced
#[derive(Debug, Clone)]
pub enum Message {
    // View navigation
    ViewChanged(View),
    ViewResults,
    ViewSettings,
    ViewASN,
    ViewAstraServerScanner,
    
    // Scanner controls
    StartScan,
    StopScan,
    StartAstraServerScan,
    UpdateProgress(f32),
    
    // File management
    CreateIPFile,
    CreatePortsFile,
    
    // Basic Configuration
    ThreadsChanged(usize),
    TimeoutChanged(f64),
    SaveSettings,
    
    // Advanced Scanner Configuration
    MaxWorkersChanged(usize),
    ChunkSizeChanged(usize),
    MaxRetriesChanged(usize),
    BatchSizeChanged(usize),
    ConnectionTimeoutChanged(f64),
    PlaylistTimeoutChanged(usize),
    ChannelTimeoutChanged(usize),
    PoolConnectionsChanged(usize),
    PoolMaxSizeChanged(usize),
    
    // ASN Configuration
    AsnMaxWorkersChanged(usize),
    AsnApiTimeoutChanged(usize),
    
    // Server interactions
    ViewServerDetails(IpAddr, u16),
    ExportResults,
    PlayChannel(String),
    DownloadServerPlaylist(IpAddr, u16),
    ShowServerCountry(IpAddr),
    
    // Input changes
    TargetChanged(String),
    PortsChanged(String),
    ChannelsSearchChanged(String),
    
    // Modal actions
    OpenSettings,
    OpenAbout,
    CloseAbout,
    
    // Extra actions
    ImportList,
    Exit,
    
    // Config actions
    LoadConfig,
    SaveConfig,
    
    // Mensajes de navegación
    ViewDashboard,
    ViewLogs,
    ViewAstraServer,
    
    // Mensajes de acción
    StartAsnLookup(String),
    UpdateConfig(Config),
    ToggleLog(String),
    UpdateStatus(String),
    
    // Nuevo mensaje para cambiar la vista de resultados
    SwitchResultsView(ResultsView),
} 