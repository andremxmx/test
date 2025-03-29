# Astra Scanner

A network scanner application for discovering and analyzing Astra servers on your network. The application features both a command-line interface and a graphical user interface built with Iced.

## Features

- **Network Scanning**: Scan your network for Astra servers with configurable threads and timeout
- **Country-based ASN Lookup**: Find all network ranges for a specific country using ISO 2 codes (US, ES, UK, etc.)
- **Results Management**: View, filter, and export scan results
- **Graphical Interface**: User-friendly GUI built with Iced
- **Command-line Interface**: For scripting and automation

## Components

The application is structured in several modules:

- **GUI**: Built with Iced, provides a user-friendly interface
- **Scanner**: Core functionality for network scanning and server discovery
- **ASN**: Tools for looking up network ranges by country ISO code
- **Configuration**: Manages application settings
- **Language**: Supports internationalization

## Getting Started

### Prerequisites

- Rust 1.65 or later
- Cargo package manager

### Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/astra-scanner.git
cd astra-scanner
```

2. Build the application:
```bash
cargo build --release
```

3. Run the application:
```bash
# Run with GUI
./target/release/astra-scanner gui

### Desde binarios precompilados

*Próximamente*

## Uso

### Interfaz gráfica

```bash
astra-scanner gui
```

### Interfaz de terminal interactiva

```bash
astra-scanner tui
```

### Escaneo desde línea de comandos

```bash
# Escanear con configuración por defecto
astra-scanner scan

# Escanear con número específico de workers
astra-scanner --workers 16 scan

# Obtener rangos ASN para un país específico
astra-scanner asn US
```

## Estructura del Proyecto

- `src/gui/` - Interfaz gráfica con Iced
- `src/ui/` - Interfaz de terminal
- `src/scanner/` - Núcleo del escáner
- `src/asn/` - Herramientas para manejo de ASN
- `src/config/` - Gestión de configuración
- `src/lang/` - Internacionalización
- `src/utils/` - Utilidades generales

## Contribuir

Las contribuciones son bienvenidas. Por favor, sigue estos pasos:

1. Haz fork del repositorio
2. Crea una rama para tu característica (`git checkout -b feature/amazing-feature`)
3. Realiza tus cambios
4. Asegúrate de que los tests pasen
5. Haz commit de tus cambios (`git commit -m 'Add some amazing feature'`)
6. Envía tu rama (`git push origin feature/amazing-feature`)
7. Abre un Pull Request

## Licencia

Este proyecto está licenciado bajo la Licencia MIT - ver el archivo [LICENSE](LICENSE) para detalles. 