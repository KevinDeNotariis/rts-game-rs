use bevy::{prelude::*, scene::SceneInstanceReady};

pub struct RomeUnitsAnimationPlugin;

impl Plugin for RomeUnitsAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<RomeUnitsAnimationAssets>()
            .init_asset::<RomeUnitsAnimationAssets>()
            .init_resource::<RomeUnitsAnimationAssets>();
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct RomeUnitAnimationPlay {
    pub graph_handle: Handle<AnimationGraph>,
    pub index: AnimationNodeIndex,
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct RomeUnitsAnimationAssets {
    #[dependency]
    pub scene: Handle<Scene>,
    #[dependency]
    pub idle_animation: Handle<AnimationClip>,
    #[dependency]
    pub walk_animation: Handle<AnimationClip>,
    #[dependency]
    pub run_animation: Handle<AnimationClip>,
    #[dependency]
    pub fight_animation: Handle<AnimationClip>,
    #[dependency]
    pub die_animation: Handle<AnimationClip>,
}

impl FromWorld for RomeUnitsAnimationAssets {
    fn from_world(world: &mut World) -> Self {
        let model_path = "units/33-gltf-wolf/gltf/Wolf-Blender-2.82a.glb";

        let asset_server = world.resource::<AssetServer>();

        Self {
            scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset(model_path)),
            idle_animation: asset_server.load(GltfAssetLabel::Animation(4).from_asset(model_path)),
            walk_animation: asset_server.load(GltfAssetLabel::Animation(2).from_asset(model_path)),
            run_animation: asset_server.load(GltfAssetLabel::Animation(0).from_asset(model_path)),
            fight_animation: asset_server.load(GltfAssetLabel::Animation(1).from_asset(model_path)),
            die_animation: asset_server.load(GltfAssetLabel::Animation(3).from_asset(model_path)),
        }
    }
}

pub fn play_animation_when_ready(
    trigger: Trigger<SceneInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    animations_to_play: Query<&RomeUnitAnimationPlay>,
    mut players: Query<&mut AnimationPlayer>,
) {
    // The entity we spawned in `setup_mesh_and_animation` is the trigger's target.
    // Start by finding the AnimationToPlay component we added to that entity.
    if let Ok(animation_to_play) = animations_to_play.get(trigger.target()) {
        // The SceneRoot component will have spawned the scene as a hierarchy
        // of entities parented to our entity. Since the asset contained a skinned
        // mesh and animations, it will also have spawned an animation player
        // component. Search our entity's descendants to find the animation player.
        for child in children.iter_descendants(trigger.target()) {
            if let Ok(mut player) = players.get_mut(child) {
                // Tell the animation player to start the animation and keep
                // repeating it.
                //
                // If you want to try stopping and switching animations, see the
                // `animated_mesh_control.rs` example.
                player.play(animation_to_play.index).repeat();

                // Add the animation graph. This only needs to be done once to
                // connect the animation player to the mesh.
                commands
                    .entity(child)
                    .insert(AnimationGraphHandle(animation_to_play.graph_handle.clone()));
            }
        }
    }
}
