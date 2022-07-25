use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::ecs::schedule::ShouldRun;
use bevy::prelude::*;
use bevy_kira_audio::*;
use bevy_rapier3d::prelude::*;

use wall_runner_camera::*;
use wall_runner_events::*;
use wall_runner_input::*;
use wall_runner_levels::*;
use wall_runner_player::*;

mod post_system_player_level_finish;
mod pre_system_camera_player_sync;
mod pre_system_level_player_sync;
mod system_cursor_grab;
mod window;

pub(crate) const LEVELS: [&str; 3] = ["jump", "wall_run", "genesis"];

pub(crate) const INITIAL_POS: (f32, f32, f32) = (2.5, 3.0, -2.0);

pub fn app() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(PlayerSettings {
            initial_position: Vec3::from(INITIAL_POS),
            ..default()
        })
        .insert_resource(LevelInput {
            name: String::from(LEVELS[0]),
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(EventsPlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(window::WindowPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(LevelPlugin)
        .add_plugin(InputPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default().with_default_system_setup(false))
        .add_stage_after(
            CoreStage::Update,
            PhysicsStages::SyncBackend,
            SystemStage::parallel().with_system_set(
                RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsStages::SyncBackend)
                    .with_run_criteria(pause_run_criteria),
            ),
        )
        .add_stage_after(
            PhysicsStages::SyncBackend,
            PhysicsStages::StepSimulation,
            SystemStage::parallel().with_system_set(
                RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsStages::StepSimulation)
                    .with_run_criteria(pause_run_criteria),
            ),
        )
        .add_stage_after(
            PhysicsStages::StepSimulation,
            PhysicsStages::Writeback,
            SystemStage::parallel().with_system_set(
                RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsStages::Writeback)
                    .with_run_criteria(pause_run_criteria),
            ),
        )
        .add_stage_before(
            CoreStage::Last,
            PhysicsStages::DetectDespawn,
            SystemStage::parallel().with_system_set(
                RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsStages::DetectDespawn)
                    .with_run_criteria(pause_run_criteria),
            ),
        )
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_system_to_stage(
            CoreStage::PreUpdate,
            pre_system_level_player_sync::attach_player_to_level,
        )
        .add_system_to_stage(
            CoreStage::PreUpdate,
            pre_system_camera_player_sync::attach_camera_to_player,
        )
        .add_system(system_cursor_grab::cursor_grab)
        .add_system_to_stage(
            CoreStage::PostUpdate,
            post_system_player_level_finish::player_level_finish,
        )
        .run();
}
