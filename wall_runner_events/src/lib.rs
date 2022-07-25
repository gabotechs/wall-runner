use bevy::ecs::schedule::ShouldRun;
use bevy::prelude::*;

pub struct EventsPlugin;

pub struct PauseEvent;

pub struct ResumeEvent;

pub struct QuitEvent;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PauseEvent>()
            .add_event::<QuitEvent>()
            .add_event::<ResumeEvent>();
    }
}

pub fn pause_run_criteria(
    mut is_paused: Local<bool>,
    pause_ev_reader: EventReader<PauseEvent>,
    resume_ev_reader: EventReader<ResumeEvent>,
) -> ShouldRun {
    if !pause_ev_reader.is_empty() {
        *is_paused = true;
    } else if !resume_ev_reader.is_empty() {
        *is_paused = false;
    }
    if *is_paused {
        ShouldRun::No
    } else {
        ShouldRun::Yes
    }
}
