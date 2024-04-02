use bevy::prelude::Vec2;

#[derive(Copy, Clone)]
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
            HexagonDefinition::A1 => Vec2::new(479.0, 373.0),
            HexagonDefinition::A2 => Vec2::new(194.0, 540.0),
            HexagonDefinition::A3 => Vec2::new(479.0, 706.0),
            HexagonDefinition::B1 => Vec2::new(1440.0, 373.0),
            HexagonDefinition::B2 => Vec2::new(1725.0, 540.0),
            HexagonDefinition::B3 => Vec2::new(1440.0, 706.0),
        }
    }

    pub fn size(&self) -> Vec2 {
        match self {
            HexagonDefinition::Main => Vec2::new(730.0, 632.0),
            _ => Vec2::new(346.0, 299.0),
        }
    }
}

pub trait HexagonComponent {
    fn hexagon_definition(&self) -> HexagonDefinition;

    fn center(&self) -> Vec2 { HexagonDefinition::center(&self.hexagon_definition()) }
    fn size(&self) -> Vec2 { HexagonDefinition::size(&self.hexagon_definition()) }
}

pub struct HexagonMain;

impl HexagonComponent for HexagonMain {
    fn hexagon_definition(&self) -> HexagonDefinition { HexagonDefinition::Main}
}

pub struct HexagonA1;

impl HexagonComponent for HexagonA1 {
    fn hexagon_definition(&self) -> HexagonDefinition { HexagonDefinition::A1}
}

pub struct HexagonA2;

impl HexagonComponent for HexagonA2 {
    fn hexagon_definition(&self) -> HexagonDefinition { HexagonDefinition::A2}
}

pub struct HexagonA3;

impl HexagonComponent for HexagonA3 {
    fn hexagon_definition(&self) -> HexagonDefinition { HexagonDefinition::A3}
}

pub struct HexagonB1;

impl HexagonComponent for HexagonB1 {
    fn hexagon_definition(&self) -> HexagonDefinition { HexagonDefinition::B1}
}

pub struct HexagonB2;

impl HexagonComponent for HexagonB2 {
    fn hexagon_definition(&self) -> HexagonDefinition { HexagonDefinition::B2}
}

pub struct HexagonB3;

impl HexagonComponent for HexagonB3 {
    fn hexagon_definition(&self) -> HexagonDefinition { HexagonDefinition::B3}
}