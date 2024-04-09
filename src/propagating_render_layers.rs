use bevy::prelude::*;
use bevy::render::view::RenderLayers;


#[derive(Component)]
pub struct PropagatingRenderLayers {
    pub render_layers: RenderLayers,
}

pub struct PropagatingRenderLayersPlugin;

impl Plugin for PropagatingRenderLayersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_render_layers);
    }
}

fn update_render_layers(
    mut commands: Commands,
    query: Query<Entity, Changed<PropagatingRenderLayers>>,
    mut render_layers_query: Query<(Entity, Option<&PropagatingRenderLayers>, Option<&mut RenderLayers>)>,
    child_query: Query<&Children>,
) {
    for entity in query.iter() {
        recursive_propagate_render_layers(
            &mut commands, &entity, None, &mut render_layers_query, &child_query,
        );
    }
}


fn recursive_propagate_render_layers(
    commands: &mut Commands,
    entity: &Entity,
    last_render_layers: Option<&RenderLayers>,
    render_layers_query: &mut Query<(Entity, Option<&PropagatingRenderLayers>, Option<&mut RenderLayers>)>,
    child_query: &Query<&Children>,
) -> Result<(), ()> {
    let (_, maybe_propagating_render_layers, maybe_render_layers) = render_layers_query.get_mut(*entity).map_err(drop)?;

    let should_render_layers: RenderLayers = match maybe_propagating_render_layers {
        None => { last_render_layers.unwrap_or(&RenderLayers::all()).clone() }
        Some(propagating_render_layers) => { propagating_render_layers.render_layers.clone() }
    };

    match maybe_render_layers {
        None => { commands.entity(*entity).insert(should_render_layers.clone()) }
        Some(_) => { commands.entity(*entity).remove::<RenderLayers>().insert(should_render_layers.clone()) }
    };

    for child in child_query.get(*entity).ok().into_iter().flatten() {
        recursive_propagate_render_layers(
            commands, child, Some(&should_render_layers), render_layers_query, child_query,
        )?;
    }

    Ok(())
}