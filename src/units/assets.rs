use bevy::prelude::*;

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct UnitAssets {
    #[dependency]
    pub idle_animation: Handle<AnimationClip>,
    #[dependency]
    pub walk_animation: Handle<AnimationClip>,
    #[dependency]
    pub run_animation: Handle<AnimationClip>,
}

impl FromWorld for UnitAssets {
    fn from_world(world: &mut World) -> Self {
        let model_path = "units/33-gltf-wolf/gltf/Wolf-Blender-2.82a.glb";

        let assets = world.resource::<AssetServer>();

        Self {
            idle_animation: assets.load(GltfAssetLabel::Animation(5).from_asset(model_path)),
            walk_animation: assets.load(GltfAssetLabel::Animation(2).from_asset(model_path)),
            run_animation: assets.load(GltfAssetLabel::Animation(0).from_asset(model_path)),
        }
    }
}
