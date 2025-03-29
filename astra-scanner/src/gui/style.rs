use iced::{Color, widget::container, Background, Theme};

// Paleta de colores moderna con mayor contraste
pub const DARK_BLUE: Color = Color::from_rgb(0.10, 0.15, 0.35);
pub const ACCENT_BLUE: Color = Color::from_rgb(0.20, 0.55, 0.95);
pub const ACCENT_GREEN: Color = Color::from_rgb(0.12, 0.80, 0.50);
pub const ACCENT_RED: Color = Color::from_rgb(0.95, 0.23, 0.30);
pub const LIGHT_TEXT: Color = Color::from_rgb(0.98, 0.98, 0.98);
pub const CARD_BG: Color = Color::from_rgb(1.0, 1.0, 1.0); 
pub const MAIN_BG: Color = Color::from_rgb(0.94, 0.96, 0.99);
pub const HEADER_BG: Color = Color::from_rgb(0.14, 0.20, 0.40);
pub const INFO_BG: Color = Color::from_rgb(0.95, 0.97, 1.0);

// Colores específicos para UI mejorada
pub const SECONDARY_BG: Color = Color::from_rgb(0.97, 0.98, 1.0);
pub const ACTIVE_TAB_BG: Color = ACCENT_BLUE;
pub const ACTIVE_TAB_TEXT: Color = LIGHT_TEXT;
pub const LIST_HEADER_BG: Color = Color::from_rgb(0.90, 0.93, 0.98);
pub const LIST_ITEM_HOVER: Color = Color::from_rgb(0.93, 0.95, 0.99);
pub const STATS_NUMBER: Color = Color::from_rgb(0.1, 0.1, 0.1);
pub const STATS_LABEL: Color = Color::from_rgb(0.4, 0.4, 0.45);

// Container styling
pub struct MainContainer;

impl container::StyleSheet for MainContainer {
    type Style = Theme;
    
    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            border_radius: 0.0.into(),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            background: Some(Background::Color(MAIN_BG)),
            text_color: None,
        }
    }
}

pub struct HeaderContainer;

impl container::StyleSheet for HeaderContainer {
    type Style = Theme;
    
    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            border_radius: 0.0.into(),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            background: Some(Background::Color(HEADER_BG)),
            text_color: Some(LIGHT_TEXT),
        }
    }
}

pub struct CardContainer;

impl container::StyleSheet for CardContainer {
    type Style = Theme;
    
    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            border_radius: 8.0.into(),
            border_width: 1.0,
            border_color: Color { a: 0.05, ..Color::BLACK },
            background: Some(Background::Color(CARD_BG)),
            text_color: None,
        }
    }
}

pub struct InfoContainer;

impl container::StyleSheet for InfoContainer {
    type Style = Theme;
    
    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            border_radius: 6.0.into(),
            border_width: 1.0,
            border_color: Color { a: 0.08, ..Color::BLACK },
            background: Some(Background::Color(INFO_BG)),
            text_color: None,
        }
    }
}

pub struct ListHeaderContainer;

impl container::StyleSheet for ListHeaderContainer {
    type Style = Theme;
    
    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            border_radius: 6.0.into(),
            border_width: 1.0,
            border_color: Color { a: 0.08, ..Color::BLACK },
            background: Some(Background::Color(LIST_HEADER_BG)),
            text_color: Some(Color::from_rgb(0.3, 0.3, 0.35)),
        }
    }
}

// Stats Container - Para el panel de estadísticas
pub struct StatsContainer;

impl container::StyleSheet for StatsContainer {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            border_radius: 8.0.into(),
            border_width: 1.0,
            border_color: Color { a: 0.08, ..Color::BLACK },
            background: Some(Background::Color(SECONDARY_BG)),
            text_color: None,
        }
    }
}

// Contenedores de estado
pub struct SuccessContainer;

impl container::StyleSheet for SuccessContainer {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            border_radius: 6.0.into(),
            border_width: 1.0,
            border_color: Color::from_rgb(0.7, 0.9, 0.7),
            background: Some(Background::Color(Color::from_rgb(0.9, 1.0, 0.9))),
            text_color: Some(Color::from_rgb(0.0, 0.5, 0.0)),
        }
    }
}

pub struct ErrorContainer;

impl container::StyleSheet for ErrorContainer {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            border_radius: 6.0.into(),
            border_width: 1.0, 
            border_color: Color::from_rgb(0.9, 0.7, 0.7),
            background: Some(Background::Color(Color::from_rgb(1.0, 0.9, 0.9))),
            text_color: Some(Color::from_rgb(0.7, 0.0, 0.0)),
        }
    }
} 