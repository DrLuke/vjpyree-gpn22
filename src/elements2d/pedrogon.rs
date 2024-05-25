use std::f32::consts::PI;
use bevy::asset::Assets;
use bevy::prelude::{ColorMaterial, Commands, Event, Mesh, ResMut};
use bevy::sprite::{Material2d, MaterialMesh2dBundle, Mesh2dHandle};
use crate::hexagon::HexagonDefinition;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use vleue_kinetoscope::{AnimatedGif, AnimatedGifController};
use crate::elements2d::tunnelgon::{TunnelgonMaterial, TunnelgonParams};
use crate::propagating_render_layers::PropagatingRenderLayers;
use crate::swirl::render_target::SwirlRenderTarget;

#[derive(Component)]
pub struct Pedrogon {
    hexagon_definition: HexagonDefinition,
}

#[derive(Event)]
pub struct SetPedrogonEvent {
    pub affected_hexagons: Vec<HexagonDefinition>,
}

pub fn spawn_pedrogon(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>
) {
    let gif_asset: Handle<AnimatedGif> = asset_server.load("pedro.gif");


    for hex in vec![HexagonDefinition::A1, HexagonDefinition::A2, HexagonDefinition::A3,
                    HexagonDefinition::B1, HexagonDefinition::B2, HexagonDefinition::B3] {
        let mesh = Mesh2dHandle(meshes.add(
            RegularPolygon::new(HexagonDefinition::size(&hex).x / 2., 6)
        ));
        commands.spawn((
            MaterialMesh2dBundle {
                mesh,
                material: materials.add(ColorMaterial {
                    texture: None, //TODO
                    ..default()
                }),
                transform: Transform::from_xyz(
                    // Distribute shapes from -X_EXTENT to +X_EXTENT.
                    HexagonDefinition::center(&hex).x - 1920. / 2.,
                    HexagonDefinition::center(&hex).y - 1080. / 2.,
                    0.0,
                ).with_rotation(Quat::from_rotation_z(PI / 6.)),
                //visibility: Visibility::Hidden,
                ..default()
            },
            PropagatingRenderLayers { render_layers: RenderLayers::layer(3) },
            Pedrogon { hexagon_definition: hex },
            gif_asset.clone(),
            AnimatedGifController::default(),
            Handle::<Image>::default(),
        ));
    }
}

pub fn update_pedrogon(
    mut query: Query<(&Handle<ColorMaterial>, &Handle<Image>), (With<Pedrogon>, Changed<Handle<Image>>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (m_h, i_h) in query.iter() {
        let mat = materials.get_mut(&(*m_h)).unwrap();
        mat.texture = Some(i_h.clone());
    }
}

pub fn show_pedrogon(
    mut query: Query<(&Pedrogon, &mut Visibility)>,
    mut ev_reader: EventReader<SetPedrogonEvent>,
) {
    for ev in ev_reader.read() {
        for (sw, mut vis) in query.iter_mut() {
            if ev.affected_hexagons.contains(&sw.hexagon_definition) {
                *vis = Visibility::Visible;
            } else {
                *vis = Visibility::Hidden;
            }
        }
    }
}