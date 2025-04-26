use bevy::prelude::*;

use super::assets::UnitAssets;

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct UnitAnimations {
    idle: AnimationNodeIndex,
    walk: AnimationNodeIndex,
    run: AnimationNodeIndex,
}

#[derive(Component, Reflect, Clone, Copy)]
#[reflect(Component)]
struct UnitAnimationLink(Entity);

#[derive(Debug, Clone, Copy, PartialEq)]
enum UnitAnimationState {
    Idle,
    Walk,
    Run,
}

fn setup_animations(
    trigger: Trigger<OnAdd, UnitAnimationLink>,
    unit_animation_link: Query<&UnitAnimationLink>,
    mut commands: Commands,
    assets: Res<UnitAssets>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let unit_animation = unit_animation_link.get(trigger.target()).unwrap().0;

    let (graph, indices) = AnimationGraph::from_clips([
        assets.idle_animation.clone(),
        assets.walk_animation.clone(),
        assets.run_animation.clone(),
    ]);
    let graph_handle = graphs.add(graph);

    let [idle_index, walk_index, run_index] = indices.as_slice() else {
        unreachable!()
    };

    let animations = UnitAnimations {
        idle: *idle_index,
        walk: *walk_index,
        run: *run_index,
    };

    let transitions = AnimationTransitions::new();
    commands.entity(unit_animation).insert((
        animations,
        AnimationGraphHandle(graph_handle),
        transitions,
    ));
}

fn play_animations(mut query: Query<&UnitAnimation>) {}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let wolf_gltf_path = "units/33-gltf-wolf/gltf/Wolf-Blender-2.82a.glb";

    let (graph, index) = AnimationGraph::from_clip(
        asset_server.load(GltfAssetLabel::Animation(0).from_asset(wolf_gltf_path)),
    );

    let graph_handle = graphs.add(graph);

    let animation_to_play = AnimationToPlay {
        graph_handle,
        index,
    };

    let mesh_scene =
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(wolf_gltf_path)));

    commands
        .spawn((animation_to_play, mesh_scene))
        .observe(play_animation_when_ready);
}

fn play_animation_when_ready(
    trigger: Trigger<SceneInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    animation_to_play: Query<&AnimationToPlay>,
    mut players: Query<&mut AnimationPlayer>,
) {
    if let Ok(animation_to_play) = animation_to_play.get(trigger.target()) {
        for child in children.iter_descendants(trigger.target()) {
            if let Ok(mut player) = players.get_mut(child) {
                player.play(animation_to_play.index).repeat();

                commands
                    .entity(child)
                    .insert(AnimationGraphHandle(animation_to_play.graph_handle.clone()));
            }
        }
    }
}
