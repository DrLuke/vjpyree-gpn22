use bevy::prelude::{Component, Vec2};
use bevy::render::view::RenderLayers;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum HexagonDefinition {
    Main,
    A1,
    A2,
    A3,
    B1,
    B2,
    B3,
}

impl HexagonDefinition {
    pub fn center(&self) -> Vec2 {
        match self {
            HexagonDefinition::Main => Vec2::new(960.0, 540.0),
            HexagonDefinition::A3 => Vec2::new(479.0, 373.0),
            HexagonDefinition::A2 => Vec2::new(194.0, 540.0),
            HexagonDefinition::A1 => Vec2::new(479.0, 706.0),
            HexagonDefinition::B3 => Vec2::new(1440.0, 373.0),
            HexagonDefinition::B2 => Vec2::new(1725.0, 540.0),
            HexagonDefinition::B1 => Vec2::new(1440.0, 706.0),
        }
    }

    pub fn size(&self) -> Vec2 {
        match self {
            HexagonDefinition::Main => Vec2::new(730.0, 632.0),
            _ => Vec2::new(346.0, 299.0),
        }
    }

    pub fn get_render_layers(&self) -> RenderLayers {
        match self {
            HexagonDefinition::Main => { RenderLayers::layer(10) }
            HexagonDefinition::A1 => { RenderLayers::layer(11) }
            HexagonDefinition::A2 => { RenderLayers::layer(12) }
            HexagonDefinition::A3 => { RenderLayers::layer(13) }
            HexagonDefinition::B1 => { RenderLayers::layer(14) }
            HexagonDefinition::B2 => { RenderLayers::layer(15) }
            HexagonDefinition::B3 => { RenderLayers::layer(16) }
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Component)]
pub struct HexagonComponent {
    hexagon_definition: HexagonDefinition,
}

impl HexagonComponent {
    pub fn get_hexagon_definition(&self) -> HexagonDefinition { self.hexagon_definition }
}

impl From<HexagonDefinition> for HexagonComponent {
    fn from(value: HexagonDefinition) -> Self {
        Self { hexagon_definition: value }
    }
}

impl PartialEq<HexagonDefinition> for HexagonComponent {
    fn eq(&self, other: &HexagonDefinition) -> bool {
        self.hexagon_definition == *other
    }
}