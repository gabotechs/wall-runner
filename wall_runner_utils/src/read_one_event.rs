use bevy::ecs::system::Resource;
use bevy::prelude::*;

pub fn read_one_event<T: Resource + Default + Copy>(mut input_ev_reader: EventReader<T>) -> T {
    if let Some(ev) = input_ev_reader.iter().next() {
        return *ev;
    }
    T::default()
}
