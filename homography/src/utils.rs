use bevy::{
    ecs::{archetype::Archetypes, component::Components, entity::Entities},
    prelude::*,
};
use nalgebra::Point3;

pub fn inspect(
    keyboard: Res<Input<KeyCode>>,
    all_entities: Query<Entity>,
    entities: &Entities,
    archetypes: &Archetypes,
    components: &Components,
) {
    if keyboard.just_pressed(KeyCode::F1) {
        for entity in all_entities.iter() {
            println!("Entity: {:?}", entity);
            if let Some(entity_location) = entities.get(entity) {
                if let Some(archetype) = archetypes.get(entity_location.archetype_id) {
                    for component in archetype.components() {
                        if let Some(info) = components.get_info(component) {
                            println!("\tComponent: {}", info.name());
                        }
                    }
                }
            }
        }
    }
}

pub fn cross_lines(p: &Point3<f32>, size: f32) -> Vec<[f32; 3]> {
    let p = p.coords;
    return vec![
        [p.x + size, p.y, p.z],
        [p.x - size, p.y, p.z],
        [p.x, p.y + size, p.z],
        [p.x, p.y - size, p.z],
        [p.x, p.y, p.z + size],
        [p.x, p.y, p.z - size],
    ];
}
