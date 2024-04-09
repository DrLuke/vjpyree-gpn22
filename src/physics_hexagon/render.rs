use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};
use bevy::render::texture::{ImageAddressMode, ImageSampler, ImageSamplerDescriptor};


#[derive(Resource)]
pub struct PhysicsHexagonRenderTarget {
    pub render_target: Handle<Image>,
}

impl FromWorld for PhysicsHexagonRenderTarget {
    fn from_world(world: &mut World) -> Self {
        let mut images = world.get_resource_mut::<Assets<Image>>().unwrap();

        let size = Extent3d {
            width: 1920,
            height: 1080,
            ..default()
        };
        let mut image = Image {
            texture_descriptor: TextureDescriptor {
                label: None,
                size,
                dimension: TextureDimension::D2,
                format: TextureFormat::Rgba16Float,
                mip_level_count: 1,
                sample_count: 1,
                usage: TextureUsages::TEXTURE_BINDING
                    | TextureUsages::COPY_DST
                    | TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            },
            sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
                address_mode_u: ImageAddressMode::Repeat,
                address_mode_v: ImageAddressMode::Repeat,
                ..Default::default()
            }),
            ..default()
        };
        image.resize(size);

        Self {
            render_target: images.add(image),
        }
    }
}