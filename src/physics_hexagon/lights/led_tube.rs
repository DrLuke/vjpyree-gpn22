use std::f32::consts::PI;
use bevy::math::Quat;
use bevy::prelude::{Commands, Component, SpatialBundle, Transform, Vec2, Vec3, BuildChildren, Color};
use bevy::utils::default;
use strum_macros::EnumIter;

pub const TUBE_LENGTH: f32 = 173.;
pub const LEDS_COUNT: isize = 16;

#[derive(Copy, Clone, EnumIter, Eq, PartialEq, Debug)]
pub enum TubeIndex {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
    Sixteen,
    Seventeen,
    Eighteen,
    Nineteen,
    Twenty,
    Twentyone,
    Twentytwo,
}

// Offset vertical:
// * Main first diagonal: 83
// * Main second diagonal: 166
// * Main into top: ?
// * Main horizontal distance from edge: 17
// Horizontal:
// * Main first diagonal: 142.5
// * Second diagonal: ? (unnecessary, we go from the middle)
// * Third diagonal: 142.5
impl TubeIndex {
    pub fn get_position(&self) -> Vec2 {
        match self {
            // Left fork
            TubeIndex::One => { Vec2 { x: 194., y: 706.5 } } // ✅
            TubeIndex::Two => { Vec2 { x: 194., y: 373.5 } } // ✅
            TubeIndex::Three => { Vec2 { x: 336.5, y: 623. } } // ✅
            TubeIndex::Four => { Vec2 { x: 336.5, y: 457. } } // ✅
            TubeIndex::Five => { Vec2 { x: 479., y: 540. } } // ✅

            // Main Diagonals left
            TubeIndex::Six => { Vec2 { x: 621.5, y: 623. } } // ✅
            TubeIndex::Seven => { Vec2 { x: 621.5, y: 457. } } // ✅
            TubeIndex::Eight => { Vec2 { x: 720., y: 790. } } // ✅ X EYEBALLED
            TubeIndex::Nine => { Vec2 { x: 720., y: 290. } } // ✅ X EYEBALLED

            // Main Horizontals
            TubeIndex::Ten => { Vec2 { x: 868.75, y: 873. } } // ✅
            TubeIndex::Eleven => { Vec2 { x: 868.75, y: 207. } } // ✅
            TubeIndex::Twelve => { Vec2 { x: 1051.25, y: 873. } } // ✅
            TubeIndex::Thirteen => { Vec2 { x: 1051.25, y: 207. } } // ✅

            // Main Diagonals right
            TubeIndex::Fourteen => { Vec2 { x: 1200., y: 790. } } // ✅ X EYEBALLED
            TubeIndex::Fifteen => { Vec2 { x: 1200., y: 290. } } // ✅ X EYEBALLED
            TubeIndex::Sixteen => { Vec2 { x: 1299., y: 623. } } // ✅
            TubeIndex::Seventeen => { Vec2 { x: 1299., y: 457. } } // ✅

            // Right fork
            TubeIndex::Eighteen => { Vec2 { x: 1440., y: 540. } } // ✅
            TubeIndex::Nineteen => { Vec2 { x: 1582.5, y: 623. } } // ✅
            TubeIndex::Twenty => { Vec2 { x: 1582.5, y: 457. } } // ✅
            TubeIndex::Twentyone => { Vec2 { x: 1725., y: 706.5 } } // ✅
            TubeIndex::Twentytwo => { Vec2 { x: 1725., y: 373.5 } } // ✅

            _ => { Vec2::default() }
        }
    }

    pub fn get_rotation(&self) -> f32 {
        match self {
            TubeIndex::One => { 0. } // ✅
            TubeIndex::Two => { PI } // ✅
            TubeIndex::Three => { -PI / 3. } // ✅
            TubeIndex::Four => { PI / 3. + PI  } // ✅
            TubeIndex::Five => { 0. } // ✅
            TubeIndex::Six => { PI / 3. } // ✅
            TubeIndex::Seven => { -PI / 3. + PI } // ✅
            TubeIndex::Eight => { PI / 3. } // ✅
            TubeIndex::Nine => { -PI / 3. + PI } // ✅
            TubeIndex::Ten => { 0. } // ✅
            TubeIndex::Eleven => { PI } // ✅
            TubeIndex::Twelve => { 0. } // ✅
            TubeIndex::Thirteen => { PI } // ✅
            TubeIndex::Fourteen => { -PI / 3. } // ✅
            TubeIndex::Fifteen => { PI / 3. + PI } // ✅
            TubeIndex::Sixteen => { -PI / 3. } // ✅
            TubeIndex::Seventeen => { PI / 3. + PI } // ✅
            TubeIndex::Eighteen => { 0. } // ✅
            TubeIndex::Nineteen => { PI / 3. } // ✅
            TubeIndex::Twenty => { -PI / 3. + PI  } // ✅
            TubeIndex::Twentyone => { 0. } // ✅
            TubeIndex::Twentytwo => { PI } // ✅
        }
    }
}

/// One LED tube (size in screenspace: 171x9 )
#[derive(Component)]
pub struct LedTube {
    index: TubeIndex,
    //adjacent_hexagons: (HexagonDefinition, Option<HexagonDefinition>),
}

impl LedTube {
    pub fn get_tube_index(&self) -> TubeIndex { self.index }
}

/// Single LED as part of the LED tube
#[derive(Component, Default)]
pub struct LedTubeLed {
    index: isize,
    pub color: Color,
}

impl LedTubeLed {
    pub fn get_index(&self) -> isize { self.index }
}

pub fn spawn_tube(
    tube_index: TubeIndex,
    mut commands: &mut Commands,
) {
    let position = tube_index.get_position();
    let angle = tube_index.get_rotation();
    let tube_entity = commands.spawn((
        LedTube {
            index: tube_index,
        },
        SpatialBundle {
            transform: Transform::from_xyz(position.x, position.y, 0.).with_rotation(Quat::from_rotation_z(angle)),
            ..default()
        }
    )).id();

    let step = TUBE_LENGTH / LEDS_COUNT as f32;

    for i in 0..LEDS_COUNT {
        let offset = TUBE_LENGTH / 2. + step * i as f32 - TUBE_LENGTH;
        let led_tube_led_entity = commands.spawn((
            LedTubeLed { index: i, ..default() },
            SpatialBundle {
                transform: Transform::from_translation(Vec3::new(offset, 0., 0.)),
                ..default()
            }
        )).id();
        commands.entity(tube_entity).push_children(&[led_tube_led_entity]);
    }
}