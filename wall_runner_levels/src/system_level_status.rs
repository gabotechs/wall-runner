use crate::event_level_finish::EventLevelFinish;
use crate::resource_level_status::LevelStatus;
use crate::LevelPlayerPositionInput;
use bevy::prelude::*;

pub fn level_status(
    player_position: Res<LevelPlayerPositionInput>,
    level_status: Res<LevelStatus>,
    mut ev_writer: EventWriter<EventLevelFinish>,
) {
    if player_position.position.z.abs() > level_status.current_win_z {
        info!("level passed!");
        ev_writer.send(EventLevelFinish { win: true });
    } else if player_position.position.y < 0.0 {
        info!("game over :(");
        ev_writer.send(EventLevelFinish { win: false });
    }
}
