use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
        },
    },
    sprite::{Material2d, Material2dKey, MaterialMesh2dBundle},
};
use wgpu_types::BlendState;

use crate::image_gen::{image_gen, ImageGenOptions, PremultipliedAlpha};

const SPINEBOY_SCALE: Vec3 = Vec3::new(136. * 2., 149. * 2., 1.);
const SPINEBOY_POSITION: Vec3 = Vec3::new(-230., 100., 0.);

const IMAGE_GEN_POSITION_1: Vec3 = Vec3::new(-210., -150., 0.);
const IMAGE_GEN_POSITION_2: Vec3 = Vec3::new(-250., -150., 0.);
const IMAGE_GEN_SCALE: Vec3 = Vec3::new(100., 100., 0.);

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<Material>>,
    mut images: ResMut<Assets<Image>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::from_translation(SPINEBOY_POSITION).with_scale(SPINEBOY_SCALE),
        material: materials.add(Material::new(asset_server.load("spineboy-head-pma.png"))),
        ..default()
    });
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::from_translation(SPINEBOY_POSITION).with_scale(SPINEBOY_SCALE),
        material: materials.add(Material::new(asset_server.load("spineboy-eye-pma.png"))),
        ..default()
    });

    let image = images.add(image_gen(ImageGenOptions {
        premultiplied_alpha: PremultipliedAlpha::SRGB, // Works correctly if changed to "Linear"
    }));
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::from_translation(IMAGE_GEN_POSITION_1).with_scale(IMAGE_GEN_SCALE),
        material: materials.add(Material::new(image.clone())),
        ..default()
    });
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::from_translation(IMAGE_GEN_POSITION_2 + Vec3::new(0., 0., 0.1))
            .with_scale(IMAGE_GEN_SCALE),
        material: materials.add(Material::new(image.clone())),
        ..default()
    });
}

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "0fcc897a-f436-436f-a648-361255d6fe3d"]
pub struct Material {
    #[texture(0)]
    #[sampler(1)]
    pub image: Handle<Image>,
}

impl Material {
    pub fn new(image: Handle<Image>) -> Self {
        Self { image }
    }
}

impl Material2d for Material {
    fn fragment_shader() -> ShaderRef {
        "vanilla.wgsl".into()
    }

    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: Material2dKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        if let Some(fragment) = &mut descriptor.fragment {
            if let Some(target_state) = &mut fragment.targets[0] {
                target_state.blend = Some(BlendState::PREMULTIPLIED_ALPHA_BLENDING);
            }
        }
        Ok(())
    }
}
