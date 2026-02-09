use crate::collision::{ObjectPlacement, ObjectType, spawn_objects_from_data};
use bevy::prelude::*;

/// Example function to spawn collision objects in the crash site room
pub fn spawn_crash_site_objects(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let ship_size = Vec2::new(1200.0, 800.0);
    let green_plant_size = Vec2::new(200.0, 200.0);
    let purple_plant_size = Vec2::new(100.0, 100.0);
    let coral_size = Vec2::new(100.0, 100.0);

    let placements = vec![
        // Main crashed spaceship (large collision)
        ObjectPlacement {
            object_type: ObjectType::CrashedShip,
            position: Vec2::new(300.0, 100.0),
            size: ship_size,
            sprite_path: "crashed_ship.png".to_string(),
            collision_offset: Some(Vec2::new(0.0, -120.0)),
            collision_size: Some(Vec2::new(1200.0, 500.0)),
        },
        // Alien foliage/plants
        ObjectPlacement {
            object_type: ObjectType::Foliage,
            position: Vec2::new(-500.0, 500.0),
            size: green_plant_size,
            sprite_path: "alien_plant_green.png".to_string(),
            collision_offset: Some(Vec2::new(0.0, -60.0)),
            collision_size: Some(Vec2::new(200.0, 70.0)),
        },
        ObjectPlacement {
            object_type: ObjectType::Foliage,
            position: Vec2::new(700.0, -300.0),
            size: purple_plant_size,
            sprite_path: "alien_plant_purple.png".to_string(),
            collision_offset: Some(Vec2::new(0.0, -45.0)),
            collision_size: Some(Vec2::new(75.0, 10.0)),
        },
        ObjectPlacement {
            object_type: ObjectType::Foliage,
            position: Vec2::new(900.0, -500.0),
            size: purple_plant_size,
            sprite_path: "alien_plant_purple.png".to_string(),
            collision_offset: Some(Vec2::new(0.0, -45.0)),
            collision_size: Some(Vec2::new(75.0, 10.0)),
        },
        ObjectPlacement {
            object_type: ObjectType::Foliage,
            position: Vec2::new(1200.0, -700.0),
            size: purple_plant_size,
            sprite_path: "alien_plant_purple.png".to_string(),
            collision_offset: Some(Vec2::new(0.0, -45.0)),
            collision_size: Some(Vec2::new(75.0, 10.0)),
        },
        ObjectPlacement {
            object_type: ObjectType::Foliage,
            position: Vec2::new(2400.0, -3000.0),
            size: purple_plant_size,
            sprite_path: "alien_plant_purple.png".to_string(),
            collision_offset: Some(Vec2::new(0.0, -45.0)),
            collision_size: Some(Vec2::new(75.0, 10.0)),
        },
        ObjectPlacement {
            object_type: ObjectType::Foliage,
            position: Vec2::new(650.0, -400.0),
            size: coral_size,
            sprite_path: "alien_coral.png".to_string(),
            collision_offset: Some(Vec2::new(0.0, 0.0)),
            collision_size: Some(coral_size),
        },
    ];

    spawn_objects_from_data(commands, asset_server, &placements);
}

pub fn spawn_deadbody_objects(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let green_plant_size = Vec2::new(200.0, 200.0);
    let purple_plant_size = Vec2::new(100.0, 100.0);
    let body_size = Vec2::new(64.0, 64.0);

    let placements = vec![
        // Main crashed spaceship (large collision)
        ObjectPlacement {
            object_type: ObjectType::Body,
            position: Vec2::new(-6064.0, 2500.0),
            size: body_size,
            sprite_path: "dead_person.png".to_string(),
            collision_offset: Some(Vec2::new(0.0, 0.0)),
            collision_size: Some(Vec2::new(64.0, 64.0)),
        },
        // Alien foliage/plants
        ObjectPlacement {
            object_type: ObjectType::Foliage,
            position: Vec2::new(-6240.0, 2500.0),
            size: green_plant_size,
            sprite_path: "alien_plant_green.png".to_string(),
            collision_offset: Some(Vec2::new(0.0, -60.0)),
            collision_size: Some(Vec2::new(200.0, 70.0)),
        },
        ObjectPlacement {
            object_type: ObjectType::Foliage,
            position: Vec2::new(-5000.0, 2400.0),
            size: green_plant_size,
            sprite_path: "alien_plant_green.png".to_string(),
            collision_offset: Some(Vec2::new(0.0, -45.0)),
            collision_size: Some(Vec2::new(75.0, 10.0)),
        },
        ObjectPlacement {
            object_type: ObjectType::Foliage,
            position: Vec2::new(-5500.0, 2300.0),
            size: purple_plant_size,
            sprite_path: "alien_plant_purple.png".to_string(),
            collision_offset: Some(Vec2::new(0.0, -45.0)),
            collision_size: Some(Vec2::new(75.0, 10.0)),
        },
        ObjectPlacement {
            object_type: ObjectType::Foliage,
            position: Vec2::new(-6400.0, 2400.0),
            size: purple_plant_size,
            sprite_path: "alien_plant_purple.png".to_string(),
            collision_offset: Some(Vec2::new(0.0, -45.0)),
            collision_size: Some(Vec2::new(75.0, 10.0)),
        },
    ];

    spawn_objects_from_data(commands, asset_server, &placements);
}

pub fn spawn_joe_objects(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let green_plant_size = Vec2::new(200.0, 200.0);
    let purple_plant_size = Vec2::new(100.0, 100.0);
    let body_size = Vec2::new(64.0, 64.0);

    let placements = vec![
        // Main crashed spaceship (large collision)
        ObjectPlacement {
            object_type: ObjectType::Body,
            position: Vec2::new(3550.0, -2800.0),
            size: body_size,
            sprite_path: "dead_joe.png".to_string(),
            collision_offset: Some(Vec2::new(0.0, 0.0)),
            collision_size: Some(Vec2::new(64.0, 64.0)),
        },
        // Alien foliage/plants
        ObjectPlacement {
            object_type: ObjectType::Foliage,
            position: Vec2::new(3550.0, -3000.0),
            size: green_plant_size,
            sprite_path: "alien_plant_green.png".to_string(),
            collision_offset: Some(Vec2::new(0.0, -60.0)),
            collision_size: Some(Vec2::new(200.0, 70.0)),
        },
        ObjectPlacement {
            object_type: ObjectType::Foliage,
            position: Vec2::new(3850.0, -2700.0),
            size: green_plant_size,
            sprite_path: "alien_plant_green.png".to_string(),
            collision_offset: Some(Vec2::new(0.0, -45.0)),
            collision_size: Some(Vec2::new(75.0, 10.0)),
        },
        ObjectPlacement {
            object_type: ObjectType::Foliage,
            position: Vec2::new(4050.0, -2800.0),
            size: purple_plant_size,
            sprite_path: "alien_plant_purple.png".to_string(),
            collision_offset: Some(Vec2::new(0.0, -45.0)),
            collision_size: Some(Vec2::new(75.0, 10.0)),
        },
        ObjectPlacement {
            object_type: ObjectType::Foliage,
            position: Vec2::new(3850.0, -2500.0),
            size: purple_plant_size,
            sprite_path: "alien_plant_purple.png".to_string(),
            collision_offset: Some(Vec2::new(0.0, -45.0)),
            collision_size: Some(Vec2::new(75.0, 10.0)),
        },
    ];

    spawn_objects_from_data(commands, asset_server, &placements);
}
