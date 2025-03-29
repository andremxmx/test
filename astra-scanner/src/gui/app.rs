use std::sync::Arc;
use std::time::Duration;
use chrono::Local;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

use iced::{
    Application, Command, Element, Length, Subscription, Theme,
    widget::{column, row, button, text, horizontal_space},
};

use crate::config::{Config, SimpleScannerConfig};
use crate::scanner::{SimpleScanner, Server};

use crate::gui::{
    message::Message,
    views,
};

// Para operaciones de entrada/salida
use std::io::{BufRead, Write};

// Para abrir URLs externas
use open;

// View state tracking
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum View {
    Dashboard,
    Results,
    Settings,
    ASN,
    AstraServer,
}

// Enum para controlar qué tipo de resultados mostrar
#[derive(Debug, Clone, Copy)]
pub enum ResultsView {
    Servers,
    Channels,
}

// Variable global para contar canales encontrados
pub static TOTAL_CHANNELS_FOUND: AtomicUsize = AtomicUsize::new(0);

// The main application
pub struct AstraApp {
    view: View,
    scanner: Arc<Mutex<SimpleScanner>>,
    simple_config: SimpleScannerConfig,
    config: Config,
    is_scanning: bool,
    progress: f32,
    status: String,
    servers: Vec<Server>,
    total_combinations: usize,
    checked_combinations: usize,
    channels_found: usize,
    results_view: ResultsView, // Qué tipo de resultados mostrar: servidores o canales
    channels_search: String,   // Texto para buscar canales por nombre
}

impl AstraApp {
    pub fn new() -> Self {
        let simple_config = SimpleScannerConfig::default();
        let config = Config::load().unwrap_or_default();
        let scanner = Arc::new(Mutex::new(SimpleScanner::new(simple_config.clone())));
        
        Self {
            view: View::Dashboard,
            scanner,
            simple_config,
            config,
            is_scanning: false,
            progress: 0.0,
            status: "Ready".to_string(),
            servers: Vec::new(),
            total_combinations: 0,
            checked_combinations: 0,
            channels_found: 0,
            results_view: ResultsView::Servers,
            channels_search: String::new(),
        }
    }
    
    // Getters
    pub fn get_view(&self) -> View {
        self.view
    }
    
    pub fn get_status(&self) -> &str {
        &self.status
    }
    
    pub fn get_progress(&self) -> f32 {
        self.progress
    }
    
    pub fn get_simple_config(&self) -> &SimpleScannerConfig {
        &self.simple_config
    }
    
    pub fn get_config(&self) -> &Config {
        &self.config
    }
    
    pub fn get_servers(&self) -> &Vec<Server> {
        &self.servers
    }
    
    pub fn is_scanning(&self) -> bool {
        self.is_scanning
    }
    
    // Getter para total de combinaciones
    pub fn get_total_combinations(&self) -> usize {
        self.total_combinations
    }
    
    // Getter para combinaciones escaneadas
    pub fn get_checked_combinations(&self) -> usize {
        self.checked_combinations
    }
    
    // Getter para canales encontrados
    pub fn get_channels_found(&self) -> usize {
        self.channels_found
    }
    
    // Getter para obtener el tipo de vista de resultados
    pub fn get_results_view(&self) -> ResultsView {
        self.results_view
    }
    
    // Getter para el texto de búsqueda de canales
    pub fn get_channels_search(&self) -> &str {
        &self.channels_search
    }
    
    // Setter para el texto de búsqueda de canales
    pub fn set_channels_search(&mut self, search: String) {
        self.channels_search = search;
    }
    
    // Create navigation tabs
    pub fn create_tabs(&self) -> Element<'_, Message> {
        let dashboard_tab = button(
            row![text("Dashboard").size(14)].spacing(5)
        )
        .padding([8, 16])
        .width(Length::Fill)
        .style(if self.view == View::Dashboard {
            iced::theme::Button::Secondary
        } else {
            iced::theme::Button::Text
        })
        .on_press(Message::ViewChanged(View::Dashboard));
        
        let results_tab = button(
            row![text("Results").size(14)].spacing(5)
        )
        .padding([8, 16])
        .width(Length::Fill)
        .style(if self.view == View::Results {
            iced::theme::Button::Secondary
        } else {
            iced::theme::Button::Text
        })
        .on_press(Message::ViewChanged(View::Results));
        
        let asn_tab = button(
            row![text("ASN Scanner").size(14)].spacing(5)
        )
        .padding([8, 16])
        .width(Length::Fill)
        .style(if self.view == View::ASN {
            iced::theme::Button::Secondary
        } else {
            iced::theme::Button::Text
        })
        .on_press(Message::ViewChanged(View::ASN));
        
        let astra_server_tab = button(
            row![text("Astra Server").size(14)].spacing(5)
        )
        .padding([8, 16])
        .width(Length::Fill)
        .style(if self.view == View::AstraServer {
            iced::theme::Button::Secondary
        } else {
            iced::theme::Button::Text
        })
        .on_press(Message::ViewChanged(View::AstraServer));
        
        let settings_tab = button(
            row![text("Settings").size(14)].spacing(5)
        )
        .padding([8, 16])
        .width(Length::Fill)
        .style(if self.view == View::Settings {
            iced::theme::Button::Secondary
        } else {
            iced::theme::Button::Text
        })
        .on_press(Message::ViewChanged(View::Settings));
        
        // Botón adicional para iniciar/detener escaneo
        let scan_button = button(
            row![
                if self.is_scanning {
                    text("DETENER ESCANEO").size(14).style(iced::theme::Text::Color(iced::Color::WHITE))
                } else {
                    text("INICIAR ESCANEO").size(14).style(iced::theme::Text::Color(iced::Color::WHITE))
                },
                text(if self.is_scanning { "⏹" } else { "▶" }).size(16).style(iced::theme::Text::Color(iced::Color::WHITE))
            ].spacing(8)
        )
        .padding([8, 20])
        .style(if self.is_scanning {
            iced::theme::Button::Destructive
        } else {
            iced::theme::Button::Primary
        })
        .on_press(if self.is_scanning {
            Message::StopScan
        } else {
            Message::StartAstraServerScan
        });
        
        row![
            dashboard_tab,
            results_tab,
            asn_tab,
            astra_server_tab,
            settings_tab,
            horizontal_space(Length::Fill),
            scan_button
        ]
        .spacing(5)
        .padding(10)
        .width(Length::Fill)
        .into()
    }
    
    // Setter para cambiar el tipo de vista de resultados
    pub fn set_results_view(&mut self, view: ResultsView) {
        self.results_view = view;
    }
}

impl Application for AstraApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (Self::new(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Astra Server Scanner")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ViewChanged(view) => {
                self.view = view;
                Command::none()
            }
            Message::StartScan => {
                if !self.is_scanning {
                    self.is_scanning = true;
                    self.progress = 0.0;
                    self.status = "Scanning...".to_string();
                    
                    // In a real app, you would start the scan here
                    // and use subscription to update progress
                    
                    // For demo purposes, simulate finding some servers
                    self.servers.push(Server {
                        ip: "192.168.1.1".parse().unwrap(),
                        port: 80,
                        service: "http".to_string(),
                        discovery_time: Local::now(),
                    });
                    
                    self.servers.push(Server {
                        ip: "192.168.1.5".parse().unwrap(),
                        port: 22,
                        service: "ssh".to_string(),
                        discovery_time: Local::now(),
                    });
                }
                Command::none()
            }
            Message::StopScan => {
                if self.is_scanning {
                    self.is_scanning = false;
                    self.status = "Scan stopped".to_string();
                }
                Command::none()
            }
            Message::UpdateProgress(progress) => {
                self.progress = progress;
                
                // Calcular el número aproximado de combinaciones escaneadas
                if self.total_combinations > 0 {
                    self.checked_combinations = ((progress / 100.0) * self.total_combinations as f32) as usize;
                }
                
                // Actualizar contador de canales desde la variable atómica estática
                self.channels_found = TOTAL_CHANNELS_FOUND.load(Ordering::SeqCst);
                
                if progress >= 100.0 {
                    self.is_scanning = false;
                    self.status = "Escaneo completado".to_string();
                    
                    // Obtener los servidores encontrados del scanner
                    if let Ok(scanner) = self.scanner.lock() {
                        self.servers = scanner.get_servers().clone();
                        self.status = format!("Escaneo completado. Encontrados {} servidores de {} combinaciones. Canales: {}.", 
                            self.servers.len(), self.total_combinations, self.channels_found);
                    }
                } else {
                    // Actualizar status con el progreso actual y obtener servidores actuales
                    if let Ok(scanner) = self.scanner.lock() {
                        self.servers = scanner.get_servers().clone();
                        self.status = format!("Escaneando... {:.1}% completado. Revisados {}/{} IPs:puertos. Encontrados {} servidores. Canales: {}.", 
                            progress, self.checked_combinations, self.total_combinations, self.servers.len(), self.channels_found);
                    } else {
                        self.status = format!("Escaneando... {:.1}% completado", progress);
                    }
                }
                Command::none()
            }
            Message::ThreadsChanged(threads) => {
                // Update both configs
                self.simple_config.threads = threads as u32;
                self.config.scanner.workers = threads;
                Command::none()
            }
            Message::TimeoutChanged(timeout) => {
                // Update both configs
                self.simple_config.timeout = std::time::Duration::from_secs_f64(timeout);
                self.config.scanner.timeout = timeout;
                self.config.scanner.connection_timeout = timeout / 2.0;
                Command::none()
            }
            
            // Advanced Scanner Config
            Message::MaxWorkersChanged(value) => {
                self.config.scanner.max_workers = value;
                Command::none()
            }
            Message::ChunkSizeChanged(value) => {
                self.config.scanner.chunk_size = value;
                Command::none()
            }
            Message::MaxRetriesChanged(value) => {
                self.config.scanner.max_retries = value;
                Command::none()
            }
            Message::BatchSizeChanged(value) => {
                self.config.scanner.batch_size = value;
                Command::none()
            }
            Message::ConnectionTimeoutChanged(value) => {
                self.config.scanner.connection_timeout = value;
                Command::none()
            }
            Message::PlaylistTimeoutChanged(value) => {
                self.config.scanner.playlist_timeout = value;
                Command::none()
            }
            Message::ChannelTimeoutChanged(value) => {
                self.config.scanner.channel_timeout = value;
                Command::none()
            }
            Message::PoolConnectionsChanged(value) => {
                self.config.scanner.pool_connections = value;
                Command::none()
            }
            Message::PoolMaxSizeChanged(value) => {
                self.config.scanner.pool_maxsize = value;
                Command::none()
            }
            
            // ASN Configuration
            Message::AsnMaxWorkersChanged(value) => {
                self.config.asn.max_workers = value;
                Command::none()
            }
            Message::AsnApiTimeoutChanged(value) => {
                self.config.asn.api_timeout = value;
                Command::none()
            }
            
            Message::SaveSettings => {
                // Update scanner with new config
                if let Ok(mut scanner) = self.scanner.lock() {
                    scanner.update_config(self.simple_config.clone());
                }
                
                // Save config to file
                if let Err(e) = self.config.save() {
                    self.status = format!("Error al guardar configuración: {}", e);
                } else {
                    self.status = "Configuración guardada correctamente".to_string();
                }
                
                Command::none()
            }
            Message::LoadConfig => {
                // Load config from file
                match Config::load() {
                    Ok(config) => {
                        self.config = config;
                        
                        // Update simple config with relevant values
                        self.simple_config.threads = self.config.scanner.workers as u32;
                        self.simple_config.timeout = std::time::Duration::from_secs_f64(self.config.scanner.timeout);
                        
                        self.status = "Configuración cargada correctamente".to_string();
                    },
                    Err(e) => {
                        self.status = format!("Error al cargar configuración: {}", e);
                    }
                }
                Command::none()
            }
            Message::SaveConfig => {
                // Save config to file
                if let Err(e) = self.config.save() {
                    self.status = format!("Error al guardar configuración: {}", e);
                } else {
                    self.status = "Configuración guardada correctamente".to_string();
                }
                Command::none()
            }
            Message::ViewResults => {
                self.view = View::Results;
                Command::none()
            }
            Message::ViewSettings => {
                self.view = View::Settings;
                Command::none()
            }
            Message::ViewASN => {
                self.view = View::ASN;
                Command::none()
            }
            Message::ViewAstraServerScanner => {
                self.view = View::AstraServer;
                Command::none()
            }
            Message::ViewServerDetails(ip, port) => {
                // In a real app, you would show details for this server
                println!("View details for {}:{}", ip, port);
                
                // Mostrar el país del servidor
                return Command::perform(
                    get_ip_country(&ip),
                    move |result| {
                        let country_info = match result {
                            Ok(country) => country,
                            Err(e) => {
                                // Si hay un error, usar un valor por defecto
                                println!("Error al obtener el país: {}", e);
                                "Desconocido".to_string()
                            }
                        };
                        
                        // Actualizar el estado con la información del país
                        Message::UpdateStatus(format!("Servidor {}:{} - País: {}", ip, port, country_info))
                    }
                );
            }
            Message::DownloadServerPlaylist(ip, port) => {
                // Descargar la playlist del servidor
                self.status = format!("Descargando playlist de {}:{}...", ip, port);
                
                return Command::perform(
                    download_server_playlist(&ip, port),
                    move |result| {
                        match result {
                            Ok(_) => {
                                Message::UpdateStatus(format!("Playlist de {}:{} descargada correctamente", ip, port))
                            },
                            Err(e) => {
                                Message::UpdateStatus(format!("Error al descargar la playlist: {}", e))
                            }
                        }
                    }
                );
            }
            Message::ShowServerCountry(ip) => {
                // Aquí se manejaría la visualización del país en una interfaz más completa
                // Por ahora, solo cambiamos el estado
                self.status = format!("Servidor en: {} (mostrando detalles)", ip);
                Command::none()
            }
            Message::UpdateStatus(status) => {
                self.status = status;
                Command::none()
            }
            Message::StartAstraServerScan => {
                // Verificar que no esté escaneando ya
                if !self.is_scanning {
                    // Calcular combinaciones totales
                    let total_combinations = crate::gui::views::astra_server::calculate_total_combinations();
                    
                    if total_combinations == 0 {
                        self.status = "No hay combinaciones para escanear. Verifica los archivos de IPs y puertos.".to_string();
                        return Command::none();
                    }
                    
                    // Iniciar estado de escaneo
                    self.is_scanning = true;
                    self.progress = 0.0;
                    self.status = "Iniciando escaneo de Astra Server...".to_string();
                    self.total_combinations = total_combinations;
                    self.checked_combinations = 0;
                    
                    // Resetear contador estático de canales
                    TOTAL_CHANNELS_FOUND.store(0, Ordering::SeqCst);
                    
                    // Clonar dependencias necesarias para el escaneo
                    let scanner_clone = self.scanner.clone();
                    let config_clone = self.config.clone();
                    
                    // Eliminar cualquier servidor encontrado previamente
                    self.servers.clear();
                    
                    // Iniciar escaneo en un hilo separado
                    return Command::perform(
                        Self::perform_astra_scan(scanner_clone, config_clone),
                        |result| {
                            match result {
                                Ok(_) => {
                                    Message::UpdateProgress(100.0)
                                },
                                Err(e) => {
                                    // Manejar error
                                    println!("Error en el escaneo: {:?}", e);
                                    Message::StopScan
                                }
                            }
                        }
                    );
                }
                Command::none()
            },
            Message::ExportResults => {
                // In a real app, you would export the results
                println!("Exporting {} server results", self.servers.len());
                Command::none()
            }
            Message::CreateIPFile => {
                // Crear archivo ip.txt
                match crate::gui::views::astra_server::create_ip_file() {
                    Ok(_) => {
                        self.status = "Archivo pool/ip.txt creado correctamente".to_string();
                    },
                    Err(e) => {
                        self.status = format!("Error creando archivo: {}", e);
                    }
                }
                Command::none()
            },
            Message::CreatePortsFile => {
                // Crear archivo ports.txt
                match crate::gui::views::astra_server::create_ports_file() {
                    Ok(_) => {
                        self.status = "Archivo pool/ports.txt creado correctamente".to_string();
                    },
                    Err(e) => {
                        self.status = format!("Error creando archivo: {}", e);
                    }
                }
                Command::none()
            },
            Message::SwitchResultsView(view) => {
                self.set_results_view(view);
                Command::none()
            },
            Message::ChannelsSearchChanged(search) => {
                self.set_channels_search(search);
                Command::none()
            },
            Message::PlayChannel(url) => {
                // Intentar reproducir con un reproductor externo
                #[cfg(target_os = "windows")]
                let player_result = {
                    // Primero intentar ejecutar VLC directamente (si está en el PATH)
                    let direct_vlc = std::process::Command::new("vlc")
                        .arg(&url)
                        .spawn();
                    
                    if direct_vlc.is_ok() {
                        direct_vlc
                    } else {
                        // Si falla, intentar iniciarlo con cmd (más común)
                        std::process::Command::new("cmd")
                            .args(["/C", "start", "vlc", &url])
                            .spawn()
                    }
                };
                
                #[cfg(target_os = "linux")]
                let player_result = std::process::Command::new("vlc")
                    .arg(&url)
                    .spawn();
                
                #[cfg(target_os = "macos")]
                let player_result = std::process::Command::new("open")
                    .args(["-a", "VLC", &url])
                    .spawn();
                
                // Comprobar si se pudo iniciar el reproductor
                match player_result {
                    Ok(_) => {
                        self.status = "Reproduciendo canal en VLC...".to_string();
                    },
                    Err(_) => {
                        // Intento alternativo con mpv
                        #[cfg(target_os = "windows")]
                        let alt_player = {
                            // Intentar con PotPlayer primero (popular para IPTV en Windows)
                            let potplayer = std::process::Command::new("PotPlayerMini64")
                                .arg(&url)
                                .spawn();
                                
                            if potplayer.is_ok() {
                                potplayer
                            } else {
                                // Si falla, intentar con mpv
                                std::process::Command::new("mpv")
                                    .arg(&url)
                                    .spawn()
                                    .or_else(|_| {
                                        // Último intento con cmd
                                        std::process::Command::new("cmd")
                                            .args(["/C", "start", "mpv", &url])
                                            .spawn()
                                    })
                            }
                        };
                        
                        #[cfg(target_os = "linux")]
                        let alt_player = std::process::Command::new("mpv")
                            .arg(&url)
                            .spawn();
                            
                        #[cfg(target_os = "macos")]
                        let alt_player = std::process::Command::new("open")
                            .args(["-a", "mpv", &url])
                            .spawn();
                            
                        match alt_player {
                            Ok(_) => {
                                self.status = "Reproduciendo canal en mpv...".to_string();
                            },
                            Err(_e) => {
                                // Si todo falla, intentar con open como última opción
                                if let Err(e) = open::that(&url) {
                                    eprintln!("Error al reproducir el canal: {}", e);
                                    self.status = format!("Error al reproducir el canal: {}. Instale VLC o mpv.", e);
                                } else {
                                    self.status = "Abriendo canal en el reproductor predeterminado...".to_string();
                                }
                            }
                        }
                    }
                }
                
                Command::none()
            },
            // Handle the other message types we're not using
            _ => Command::none(),
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.is_scanning {
            // Usar un closure que no capture self, sino solo una copia del scanner
            let scanner_clone = self.scanner.clone();
            
            struct ProgressTracker(Arc<Mutex<SimpleScanner>>);
            
            iced::subscription::unfold(
                "scan_progress_tracker",
                ProgressTracker(scanner_clone),
                move |tracker| async move {
                    // Esperar un poco
                    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                    
                    let progress = if let Ok(scanner) = tracker.0.lock() {
                        scanner.get_progress().min(100.0)
                    } else {
                        0.0
                    };
                    
                    let message = Message::UpdateProgress(progress);
                    (message, tracker)
                }
            )
        } else {
            Subscription::none()
        }
    }

    fn view(&self) -> Element<'_, Message> {
        // Common navigation tabs for all views
        let tabs = self.create_tabs();
        
        // Render the appropriate view based on state
        let view = match self.view {
            View::Dashboard => views::dashboard::view(self),
            View::Results => views::results::view(self),
            View::Settings => views::settings::view(self),
            View::ASN => views::asn::view(self),
            View::AstraServer => views::astra_server::view(self),
        };
        
        column![
            tabs,
            view,
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}

// Método para realizar un escaneo Astra Server en segundo plano
impl AstraApp {
    async fn perform_astra_scan(scanner: Arc<Mutex<SimpleScanner>>, config: Config) -> Result<usize, String> {
        use std::path::Path;
        use std::fs::{self, File, OpenOptions};
        use std::io::{BufRead, BufReader, Write};
        use tokio::time::sleep;
        use std::time::Duration;
        use std::sync::atomic::{AtomicUsize, Ordering};
        
        // Crear directorio pool si no existe
        if !Path::new("pool").exists() {
            fs::create_dir_all("pool").map_err(|e| format!("Error creando directorio pool: {}", e))?;
        }
        
        // 1. Cargar las IPs del archivo
        let ip_path = Path::new("pool/ip.txt");
        if !ip_path.exists() {
            return Err("El archivo de IPs no existe. Crea pool/ip.txt con direcciones IP, una por línea.".to_string());
        }
        
        let ip_file = File::open(ip_path).map_err(|e| format!("Error al abrir ip.txt: {}", e))?;
        let ip_reader = BufReader::new(ip_file);
        let ips: Vec<String> = ip_reader.lines()
            .filter_map(Result::ok)
            .filter(|line| !line.trim().is_empty())
            .collect();
            
        if ips.is_empty() {
            return Err("No hay IPs en el archivo pool/ip.txt".to_string());
        }
        
        // 2. Cargar los puertos del archivo
        let port_path = Path::new("pool/ports.txt");
        if !port_path.exists() {
            return Err("El archivo de puertos no existe. Crea pool/ports.txt con números de puerto, uno por línea.".to_string());
        }
        
        let port_file = File::open(port_path).map_err(|e| format!("Error al abrir ports.txt: {}", e))?;
        let port_reader = BufReader::new(port_file);
        let ports: Vec<u16> = port_reader.lines()
            .filter_map(Result::ok)
            .filter(|line| !line.trim().is_empty())
            .filter_map(|line| line.trim().parse::<u16>().ok())
            .collect();
            
        if ports.is_empty() {
            return Err("No hay puertos en el archivo pool/ports.txt".to_string());
        }
        
        // Preparar para guardar los resultados
        let found_servers_path = Path::new("found_servers.txt");
        let found_servers_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(found_servers_path)
            .map_err(|e| format!("Error abriendo found_servers.txt: {}", e))?;
        let writer = std::io::BufWriter::new(found_servers_file);
        let found_servers_writer = Arc::new(std::sync::Mutex::new(writer));
        
        // 3. Comenzar el escaneo
        let total_combinations = ips.len() * ports.len();
        let scanned = Arc::new(AtomicUsize::new(0));
        let found_servers_count = Arc::new(AtomicUsize::new(0));
        
        // Crear cliente HTTP para verificación real usando la configuración
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs_f64(config.scanner.connection_timeout))
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .build()
            .map_err(|e| format!("Error creando cliente HTTP: {}", e))?;
        
        // Configurar el número máximo de workers
        let max_workers = config.scanner.workers;
        let batch_size = config.scanner.batch_size.min(total_combinations);
        
        // Crear un pool de tareas con límite de concurrencia
        let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(max_workers));
        
        // Procesar en batches
        for (_batch_index, ips_batch) in ips.chunks(batch_size).enumerate() {
            for ip in ips_batch {
                for &port in &ports {
                    // Incrementar contador
                    let current_scanned = scanned.fetch_add(1, Ordering::SeqCst) + 1;
                    let progress = (current_scanned as f32 / total_combinations as f32) * 100.0;
                    
                    if let Ok(mut scanner_lock) = scanner.lock() {
                        scanner_lock.set_progress(progress);
                    }
                    
                    // Verificar si es un servidor Astra (como en la versión CLI)
                    let _server = format!("{}:{}", ip, port);
                    let url = format!("http://{}:{}", ip, port);
                    
                    // Obtener permiso del semáforo (limitar concurrencia)
                    let permit = semaphore.clone().acquire_owned().await.unwrap();
                    
                    // Clonar lo necesario para el worker
                    let client_clone = client.clone();
                    let ip_clone = ip.clone();
                    let scanner_clone = scanner.clone();
                    let found_servers_writer_clone = found_servers_writer.clone();
                    let found_servers_count_clone = found_servers_count.clone();
                    
                    // Lanzar tarea
                    tokio::spawn(async move {
                        // Al salir del scope, el permiso se libera automáticamente
                        let _permit = permit;
                        
                        match check_server(&client_clone, &url, Duration::from_secs_f64(config.scanner.timeout)).await {
                            Ok(true) => {
                                // Servidor Astra encontrado - incrementar contador atómico
                                found_servers_count_clone.fetch_add(1, Ordering::SeqCst);
                                
                                // Guardar en archivo
                                let server_str = format!("{}:{}\n", ip_clone, port);
                                if let Ok(mut writer) = found_servers_writer_clone.lock() {
                                    let _ = writer.write_all(server_str.as_bytes());
                                    let _ = writer.flush();
                                }
                                
                                // Registrar en el scanner
                                let server_obj = crate::scanner::Server {
                                    ip: ip_clone.parse().unwrap_or("0.0.0.0".parse().unwrap()),
                                    port,
                                    service: "http".to_string(),
                                    discovery_time: chrono::Local::now(),
                                };
                                
                                if let Ok(mut scanner_lock) = scanner_clone.lock() {
                                    scanner_lock.add_server(server_obj);
                                }
                                
                                // Intentar obtener la playlist (opcional)
                                tokio::spawn(process_playlist(
                                    client_clone.clone(), 
                                    ip_clone.clone(), 
                                    port,
                                    Duration::from_secs(config.scanner.playlist_timeout as u64),
                                    Duration::from_secs(config.scanner.channel_timeout as u64)
                                ));
                            },
                            _ => {
                                // No es un servidor Astra o error, continuar
                            }
                        }
                    });
                    
                    // Pequeña pausa para no sobrecargar la red
                    sleep(Duration::from_millis(10)).await;
                }
            }
            
            // Esperar entre batches
            sleep(Duration::from_millis(100)).await;
        }
        
        // Finalizar escritura
        found_servers_writer.lock().unwrap().flush()
            .map_err(|e| format!("Error finalizando escritura: {}", e))?;
        
        // Obtener el número final de servidores encontrados
        let total_found = found_servers_count.load(Ordering::SeqCst);
        
        // Guardar resumen del escaneo
        save_scan_summary(scanned.load(Ordering::SeqCst), total_found).await
            .map_err(|e| format!("Error guardando resumen: {}", e))?;
        
        // Completado exitosamente
        Ok(total_found)
    }
}

/// Verifica si una URL corresponde a un servidor Astra
async fn check_server(client: &reqwest::Client, url: &str, timeout: Duration) -> Result<bool, reqwest::Error> {
    // Realizar petición HEAD
    let response = client.head(url).timeout(timeout).send().await?;
    
    // Verificar encabezado del servidor
    if let Some(server_header) = response.headers().get(reqwest::header::SERVER) {
        if let Ok(server_value) = server_header.to_str() {
            if server_value.contains("Astra") {
                return Ok(true);
            }
        }
    }
    
    Ok(false)
}

/// Procesa la playlist de un servidor Astra
async fn process_playlist(client: reqwest::Client, ip: String, port: u16, playlist_timeout: Duration, channel_timeout: Duration) {
    // Intentar obtener la playlist
    let url = format!("http://{}:{}/playlist.m3u", ip, port);
    
    let playlist_result = match client.get(&url)
        .timeout(playlist_timeout)
        .send()
        .await {
            Ok(response) => {
                if response.status().is_success() {
                    match response.text().await {
                        Ok(content) => {
                            if content.contains("#EXTM3U") {
                                Ok(content)
                            } else {
                                Err("No es una playlist válida".to_string())
                            }
                        },
                        Err(_) => Err("Error leyendo respuesta".to_string())
                    }
                } else {
                    Err("Respuesta no exitosa".to_string())
                }
            },
            Err(_) => Err("Error de conexión".to_string())
        };
    
    if let Ok(content) = playlist_result {
        // Procesar los canales de la playlist con el timeout adecuado
        process_channels_with_timeout(&content, &format!("{}:{}", ip, port), channel_timeout).await;
    }
}

/// Procesa los canales de una playlist con timeout específico
async fn process_channels_with_timeout(content: &str, server: &str, timeout: Duration) {
    // Usamos el timeout para cualquier operación que lo necesite
    // Este método reemplaza al anterior process_channels pero con soporte para timeout
    
    // Extraer canales de la playlist
    let mut channels = Vec::new();
    let mut current_title = String::new();
    
    for line in content.lines() {
        if line.starts_with("#EXTINF:") {
            current_title = line.to_string();
        } else if line.starts_with("http://") || line.starts_with("https://") {
            if !current_title.is_empty() {
                channels.push((current_title.clone(), line.to_string()));
                current_title = String::new();
            }
        }
    }
    
    if channels.is_empty() {
        return;
    }
    
    // Crear directorio para los canales
    if let Err(_) = std::fs::create_dir_all("channels") {
        return;
    }
    
    // Crear cliente HTTP para verificar canales
    let client = reqwest::Client::builder()
        .timeout(timeout)
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()
        .unwrap_or_else(|_| reqwest::Client::new());
    
    // Verificar canales en batches pequeños
    let batch_size = 10;
    let mut working_channels = Vec::new();
    
    println!("Verificando {} canales de {}", channels.len(), server);
    
    // Procesar canales en batches para evitar sobrecargar la memoria/red
    for (_batch_index, batch) in channels.chunks(batch_size).enumerate() {
        // Crear tareas para verificar cada canal en paralelo
        let mut handles = Vec::new();
        
        for (title, url) in batch {
            let client_clone = client.clone();
            let url_clone = url.clone();
            let title_clone = title.clone();
            
            let handle = tokio::spawn(async move {
                let working = check_channel_working(&client_clone, &url_clone, timeout).await;
                if working {
                    println!("✓ Canal funcionando: {}", extract_channel_title(&title_clone));
                    Some((title_clone, url_clone))
                } else {
                    println!("✗ Canal no funciona: {}", extract_channel_title(&title_clone));
                    None
                }
            });
            
            handles.push(handle);
        }
        
        // Esperar a que todas las tareas del batch se completen
        for handle in handles {
            if let Ok(Some((title, url))) = handle.await {
                working_channels.push((title, url));
            }
        }
        
        // Pequeña pausa entre batches para no sobrecargar
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    
    println!("De {} canales en {}, {} están funcionando", channels.len(), server, working_channels.len());
    
    // Guardar canales en el archivo solo si funcionan
    let path = std::path::Path::new("channels/all_channels.m3u8");
    
    // Leer canales existentes para evitar duplicados
    let mut existing_urls = std::collections::HashSet::new();
    
    if path.exists() {
        if let Ok(file) = std::fs::File::open(path) {
            let reader = std::io::BufReader::new(file);
            let mut is_url_line = false;
            
            for line in reader.lines() {
                if let Ok(line) = line {
                    if is_url_line {
                        existing_urls.insert(line);
                        is_url_line = false;
                    } else if line.starts_with("#EXTINF:") {
                        is_url_line = true;
                    }
                }
            }
        }
    }
    
    // Filtrar canales duplicados
    let new_channels: Vec<_> = working_channels.iter()
        .filter(|(_, url)| !existing_urls.contains(url))
        .collect();
    
    if new_channels.is_empty() {
        return;
    }
    
    // Guardar nuevos canales
    if let Ok(file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path) {
        
        let mut writer = std::io::BufWriter::new(file);
        
        // Añadir cabecera si el archivo es nuevo
        if !path.exists() || std::fs::metadata(path).map(|m| m.len()).unwrap_or(0) == 0 {
            if let Err(_) = writeln!(writer, "#EXTM3U") {
                return;
            }
        }
        
        // Escribir nuevos canales
        for (title, url) in &new_channels {
            if writeln!(writer, "{}", title).is_err() ||
                writeln!(writer, "{}", url).is_err() {
                return;
            }
        }
        
        // Finalizar escritura
        let _ = writer.flush();
        
        // Actualizar contador global atómicamente
        let new_channels_count = new_channels.len();
        
        // Incrementar el contador global
        TOTAL_CHANNELS_FOUND.fetch_add(new_channels_count, Ordering::SeqCst);
        
        println!("Server {}: Added {} new channels (Total: {})", 
            server, new_channels_count, TOTAL_CHANNELS_FOUND.load(Ordering::SeqCst));
    }
}

/// Guarda un resumen del escaneo
async fn save_scan_summary(total_checked: usize, servers_found: usize) -> Result<(), std::io::Error> {
    use chrono::Local;
    use serde_json::json;
    
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
    // Leer servidores encontrados del archivo
    let mut found_servers = Vec::new();
    if let Ok(file) = std::fs::File::open("found_servers.txt") {
        let reader = std::io::BufReader::new(file);
        for line in reader.lines() {
            if let Ok(server) = line {
                if !server.trim().is_empty() {
                    found_servers.push(server);
                }
            }
        }
    }
    
    // Obtener cantidad de canales encontrados
    let channels_found = TOTAL_CHANNELS_FOUND.load(Ordering::SeqCst);
    
    // Crear resumen
    let summary = json!({
        "scan_date": timestamp,
        "total_checked": total_checked,
        "servers_found": servers_found,
        "channels_found": channels_found,
        "found_servers": found_servers
    });
    
    // Guardar en archivo
    let mut file = std::fs::File::create("scan_summary.json")?;
    file.write_all(serde_json::to_string_pretty(&summary)?.as_bytes())?;
    
    Ok(())
}

/// Verifica si un canal está funcionando
async fn check_channel_working(client: &reqwest::Client, url: &str, timeout: Duration) -> bool {
    // Primero intentar con un HEAD request (más rápido)
    match client.head(url)
        .timeout(timeout)
        .header(reqwest::header::USER_AGENT, "Mozilla/5.0")
        .send()
        .await {
        Ok(response) if response.status().is_success() => {
            return true;
        },
        _ => {
            // Si el HEAD falla y no es un m3u8, intentar obtener un fragmento del stream
            if !url.to_lowercase().contains(".m3u8") {
                match client.get(url)
                    .timeout(timeout)
                    .header(reqwest::header::USER_AGENT, "Mozilla/5.0")
                    .send()
                    .await {
                    Ok(response) if response.status().is_success() => {
                        // Intentar leer un fragmento de datos para verificar que el stream funciona
                        match response.bytes().await {
                            Ok(bytes) if !bytes.is_empty() => {
                                return true;
                            },
                            _ => return false
                        }
                    },
                    _ => return false
                }
            }
            return false;
        }
    }
}

/// Extrae el título del canal de la línea #EXTINF
fn extract_channel_title(extinf_line: &str) -> String {
    if let Some(idx) = extinf_line.find(',') {
        if idx + 1 < extinf_line.len() {
            return extinf_line[idx + 1..].trim().to_string();
        }
    }
    "Canal sin nombre".to_string()
}

/// Obtiene el país asociado a una dirección IP
async fn get_ip_country(ip: &std::net::IpAddr) -> Result<String, String> {
    // Usar el servicio ipapi.co para obtener información geográfica
    let url = format!("https://ipapi.co/{}/json/", ip);
    
    match reqwest::get(&url).await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<serde_json::Value>().await {
                    Ok(json) => {
                        // Extraer el código y nombre del país
                        let country_code = json["country_code"].as_str().unwrap_or("--");
                        let country_name = json["country_name"].as_str().unwrap_or("Desconocido");
                        return Ok(format!("{} ({})", country_name, country_code));
                    },
                    Err(_) => return Err("Error al analizar la respuesta de geolocalización".to_string())
                }
            } else {
                return Err(format!("Error al obtener la geolocalización: {}", response.status()));
            }
        },
        Err(e) => return Err(format!("Error al conectar con servicio de geolocalización: {}", e))
    }
}

/// Descarga la playlist de un servidor Astra
async fn download_server_playlist(ip: &std::net::IpAddr, port: u16) -> Result<(), String> {
    // URL de la playlist
    let playlist_url = format!("http://{}:{}/playlist.m3u", ip, port);
    
    // Crear cliente HTTP
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()
        .map_err(|e| format!("Error creando cliente HTTP: {}", e))?;
    
    // Obtener la playlist
    let response = client.get(&playlist_url)
        .send()
        .await
        .map_err(|e| format!("Error al conectar con el servidor: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("Error al obtener la playlist: {}", response.status()));
    }
    
    // Obtener el contenido de la playlist
    let content = response.text().await
        .map_err(|e| format!("Error al leer el contenido de la playlist: {}", e))?;
    
    if !content.contains("#EXTM3U") {
        return Err("El contenido descargado no es una playlist M3U válida".to_string());
    }
    
    // Crear el directorio de playlists si no existe
    std::fs::create_dir_all("playlists")
        .map_err(|e| format!("Error creando directorio playlists: {}", e))?;
    
    // Guardar la playlist en el archivo
    let filename = format!("playlists/playlist_{}_{}.m3u", ip, port);
    std::fs::write(&filename, content)
        .map_err(|e| format!("Error guardando la playlist: {}", e))?;
    
    // Abrir el directorio donde se guardó el archivo
    #[cfg(target_os = "windows")]
    std::process::Command::new("explorer")
        .args(["/select,", &filename])
        .spawn()
        .ok();
    
    #[cfg(target_os = "linux")]
    std::process::Command::new("xdg-open")
        .arg("playlists")
        .spawn()
        .ok();
    
    #[cfg(target_os = "macos")]
    std::process::Command::new("open")
        .arg("playlists")
        .spawn()
        .ok();
    
    Ok(())
} 