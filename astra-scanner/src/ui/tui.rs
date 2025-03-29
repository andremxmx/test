use std::io::{self};
use std::time::Duration;
use std::sync::Arc;
use tokio::sync::Mutex;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Tabs},
    Frame, Terminal,
};

use crate::scanner::AstraScanner;
use crate::config::Config;
use crate::lang::LanguageManager;

/// Estructura principal para la TUI
pub struct Tui<'a> {
    lang: &'a LanguageManager,
    config: &'a Config,
    active_tab: usize,
    scan_running: bool,
    scan_results: Option<ScanResults>,
}

/// Resultados del escaneo
struct ScanResults {
    servers_found: usize,
    channels_found: usize,
    total_checked: usize,
    progress_percent: f64,
}

impl<'a> Tui<'a> {
    pub fn new(lang: &'a LanguageManager, config: &'a Config) -> Self {
        Self {
            lang,
            config,
            active_tab: 0,
            scan_running: false,
            scan_results: None,
        }
    }
    
    /// Inicia la interfaz TUI
    pub async fn run(&mut self) -> Result<(), io::Error> {
        // Configuración de terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        let _res = crossterm::execute!(stdout.by_ref(), crossterm::terminal::EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        
        let result = self.run_app(&mut terminal).await;
        
        // Restaurar terminal
        disable_raw_mode()?;
        let mut stdout = io::stdout();
        let _res = crossterm::execute!(
            stdout.by_ref(),
            crossterm::terminal::LeaveAlternateScreen
        )?;
        terminal.show_cursor()?;
        
        result
    }
    
    async fn run_app<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<(), io::Error> {
        loop {
            terminal.draw(|f| self.draw_ui(f))?;
            
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Tab => self.active_tab = (self.active_tab + 1) % 3,
                        KeyCode::BackTab => self.active_tab = (self.active_tab + 2) % 3,
                        KeyCode::Char('1') => self.active_tab = 0,
                        KeyCode::Char('2') => self.active_tab = 1,
                        KeyCode::Char('3') => self.active_tab = 2,
                        KeyCode::Char('s') => {
                            if !self.scan_running && self.active_tab == 1 {
                                self.start_scan().await;
                            }
                        }
                        _ => {}
                    }
                }
            }
            
            // Actualizar resultados del escaneo si está en curso
            if self.scan_running {
                // Aquí iríamos actualizando los resultados del escaneo
                // Por ahora, simularemos progreso para fines de demostración
                // En una implementación real, obtendrías esto del AstraScanner
                if let Some(results) = &mut self.scan_results {
                    if results.progress_percent < 100.0 {
                        results.progress_percent += 0.5;
                        if results.progress_percent > 100.0 {
                            results.progress_percent = 100.0;
                            self.scan_running = false;
                        }
                    }
                }
            }
        }
    }
    
    async fn start_scan(&mut self) {
        self.scan_running = true;
        self.scan_results = Some(ScanResults {
            servers_found: 0,
            channels_found: 0,
            total_checked: 0,
            progress_percent: 0.0,
        });
        
        // En una implementación real, aquí iniciarías el escaneo con AstraScanner
        // y actualizarías self.scan_results periódicamente
    }
    
    fn draw_ui<B: Backend>(&self, f: &mut Frame<B>) {
        // División principal de la pantalla
        let size = f.size();
        
        // Crear las pestañas
        let titles = vec![
            "Dashboard", 
            "Astra Scanner", 
            "Settings"
        ].iter().map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(first, Style::default().fg(Color::Yellow).add_modifier(Modifier::UNDERLINED)),
                Span::styled(rest, Style::default().fg(Color::White)),
            ])
        }).collect();
        
        let tabs = Tabs::new(titles)
            .block(Block::default().borders(Borders::ALL).title("Astra Scanner"))
            .select(self.active_tab)
            .style(Style::default().fg(Color::Cyan))
            .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
            
        f.render_widget(tabs, size);
        
        // Área para el contenido (debajo de las pestañas)
        let inner_area = Rect::new(
            size.x + 1,
            size.y + 2,
            size.width - 2,
            size.height - 3,
        );
        
        match self.active_tab {
            0 => self.draw_dashboard(f, inner_area),
            1 => self.draw_scanner(f, inner_area),
            2 => self.draw_settings(f, inner_area),
            _ => {}
        }
    }
    
    fn draw_dashboard<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints::<&[Constraint]>([
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(50),
            ].as_ref())
            .split(area);
            
        // Párrafo superior
        let text = vec![
            Spans::from(vec![
                Span::styled("Astra Scanner Dashboard", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            ]),
            Spans::from(vec![
                Span::raw("Presiona "),
                Span::styled("Tab", Style::default().fg(Color::Yellow)),
                Span::raw(" para cambiar de pantalla, "),
                Span::styled("q", Style::default().fg(Color::Yellow)),
                Span::raw(" para salir"),
            ]),
        ];
        
        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title("Información"));
            
        f.render_widget(paragraph, chunks[0]);
        
        // Estadísticas
        let stats_block = Block::default()
            .borders(Borders::ALL)
            .title("Estadísticas");
            
        f.render_widget(stats_block, chunks[1]);
        
        if let Some(results) = &self.scan_results {
            let stats_inner = Layout::default()
                .direction(Direction::Horizontal)
                .constraints::<&[Constraint]>([
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                    Constraint::Percentage(34),
                ].as_ref())
                .split(chunks[1]);
                
            let servers_text = vec![
                Spans::from(vec![
                    Span::styled("Servidores", Style::default().fg(Color::Cyan)),
                ]),
                Spans::from(vec![
                    Span::styled(format!("{}", results.servers_found), 
                                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                ]),
            ];
            
            let channels_text = vec![
                Spans::from(vec![
                    Span::styled("Canales", Style::default().fg(Color::Cyan)),
                ]),
                Spans::from(vec![
                    Span::styled(format!("{}", results.channels_found), 
                                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                ]),
            ];
            
            let checked_text = vec![
                Spans::from(vec![
                    Span::styled("Verificados", Style::default().fg(Color::Cyan)),
                ]),
                Spans::from(vec![
                    Span::styled(format!("{}", results.total_checked), 
                                Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)),
                ]),
            ];
            
            f.render_widget(Paragraph::new(servers_text), stats_inner[0]);
            f.render_widget(Paragraph::new(channels_text), stats_inner[1]);
            f.render_widget(Paragraph::new(checked_text), stats_inner[2]);
        }
        
        // Lista de servidores encontrados
        let servers_block = Block::default()
            .borders(Borders::ALL)
            .title("Últimos servidores encontrados");
            
        f.render_widget(servers_block, chunks[2]);
    }
    
    fn draw_scanner<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints::<&[Constraint]>([
                Constraint::Percentage(20),
                Constraint::Percentage(10),
                Constraint::Percentage(70),
            ].as_ref())
            .split(area);
            
        // Instrucciones
        let text = vec![
            Spans::from("Escaneo de servidores Astra"),
            Spans::from(""),
            Spans::from(vec![
                Span::raw("Presiona "),
                Span::styled("s", Style::default().fg(Color::Yellow)),
                Span::raw(" para iniciar un escaneo")
            ]),
        ];
        
        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title("Astra Scanner"));
            
        f.render_widget(paragraph, chunks[0]);
        
        // Progreso del escaneo
        if self.scan_running || (self.scan_results.is_some() && !self.scan_running) {
            let progress = if let Some(results) = &self.scan_results {
                results.progress_percent / 100.0
            } else {
                0.0
            };
            
            let status = if self.scan_running {
                "En progreso"
            } else if let Some(results) = &self.scan_results {
                if results.progress_percent >= 100.0 {
                    "Completado"
                } else {
                    "Detenido"
                }
            } else {
                "No iniciado"
            };
            
            let gauge = Gauge::default()
                .block(Block::default().borders(Borders::ALL).title("Progreso"))
                .gauge_style(Style::default().fg(Color::Blue).bg(Color::Black))
                .ratio(progress)
                .label(format!("{:.1}% - {}", progress * 100.0, status));
                
            f.render_widget(gauge, chunks[1]);
            
            // Resultados
            if let Some(results) = &self.scan_results {
                let stats_text = vec![
                    Spans::from(vec![
                        Span::styled("Servidores encontrados: ", Style::default().fg(Color::Cyan)),
                        Span::styled(format!("{}", results.servers_found), Style::default().fg(Color::Green)),
                    ]),
                    Spans::from(vec![
                        Span::styled("Canales funcionando: ", Style::default().fg(Color::Cyan)),
                        Span::styled(format!("{}", results.channels_found), Style::default().fg(Color::Yellow)),
                    ]),
                    Spans::from(vec![
                        Span::styled("Total verificado: ", Style::default().fg(Color::Cyan)),
                        Span::styled(format!("{}", results.total_checked), Style::default().fg(Color::Blue)),
                    ]),
                ];
                
                let scanner_info = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints::<&[Constraint]>([
                        Constraint::Percentage(33),
                        Constraint::Percentage(33),
                        Constraint::Percentage(34),
                    ].as_ref())
                    .split(chunks[2]);
                    
                let stats = Paragraph::new(stats_text)
                    .block(Block::default().borders(Borders::ALL).title("Resultados"));
                    
                f.render_widget(stats, scanner_info[2]);
            }
        }
    }
    
    fn draw_settings<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints::<&[Constraint]>([
                Constraint::Length(3),
                Constraint::Percentage(100),
            ].as_ref())
            .split(area);
            
        // Lista de configuraciones
        let settings = vec![
            ListItem::new("Idioma: Español"),
            ListItem::new(format!("Conexiones simultáneas: {}", self.config.scanner.workers)),
            ListItem::new(format!("Timeout conexión: {} ms", self.config.scanner.connection_timeout * 1000.0)),
            ListItem::new(format!("Timeout playlist: {} s", self.config.scanner.playlist_timeout)),
            ListItem::new(format!("Tamaño de lote: {}", self.config.scanner.batch_size)),
        ];
        
        let settings_list = List::new(settings)
            .block(Block::default().borders(Borders::ALL).title("Configuración"))
            .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .highlight_symbol("> ");
            
        f.render_widget(settings_list, chunks[0]);
    }
}

/// Función para iniciar la TUI
pub async fn run_tui(lang: &LanguageManager, config: &Config) -> Result<(), io::Error> {
    let mut tui = Tui::new(lang, config);
    tui.run().await
} 