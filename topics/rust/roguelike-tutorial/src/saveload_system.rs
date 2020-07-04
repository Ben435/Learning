use specs::prelude::*;
use specs::saveload::*;
use specs::error::NoError;
use specs::World;
use std::fs;
use std::path::Path;
use crate::map::Map;
use crate::components::*;
use crate::constants::MAPCOUNT;

const SAVEGAME_FILE_NAME: &str = "./savegame.json";

macro_rules! serialize_individually {
    ($ecs:expr, $ser:expr, $data:expr, $( $type:ty),*) => {
        $(
        SerializeComponents::<NoError, SimpleMarker<SerializeMe>>::serialize(
            &( $ecs.read_storage::<$type>(), ),
            &$data.0,
            &$data.1,
            &mut $ser,
        )
        .unwrap();
        )*
    };
}

macro_rules! deserialize_individually {
    ($ecs:expr, $de:expr, $data:expr, $( $type:ty),*) => {
        $(
        DeserializeComponents::<NoError, _>::deserialize(
            &mut ( &mut $ecs.write_storage::<$type>(), ),
            &mut $data.0, // entities
            &mut $data.1, // marker
            &mut $data.2, // allocater
            &mut $de,
        )
        .unwrap();
        )*
    };
}

pub fn save_game(ecs: &mut World) {
    let mapcopy = ecs.get_mut::<Map>().unwrap().clone();
    let savehelper = ecs
        .create_entity()
        .with(SerializationHelper{ map: mapcopy })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();

    {
        let data = ( ecs.entities(), ecs.read_storage::<SimpleMarker<SerializeMe>>() );

        let writer = fs::File::create(SAVEGAME_FILE_NAME).unwrap();
        let mut serializer = serde_json::Serializer::new(writer);
        serialize_individually!(ecs, serializer, data, Position, Renderable, Player, Viewshed, Monster, 
            Name, BlocksTile, CombatStats, SufferDamage, WantsToMelee, Item, Consumable, Ranged, InflictsDamage, 
            AreaOfEffect, Confusion, ProvidesHealing, InBackpack, WantsToPickupItem, WantsToUseItem,
            WantsToDropItem, SerializationHelper
        );
    }

    ecs.delete_entity(savehelper).expect("Crash on cleanup");
}

pub fn does_save_exist() -> bool {
    Path::new(&SAVEGAME_FILE_NAME).exists()
}

pub fn load_game(ecs: &mut World) {
    {
        // Clear the world, so we don't blur loaded data with current data.
        let mut to_delete = Vec::new();
        for e in ecs.entities().join() {
            to_delete.push(e);
        }
        for del in to_delete.iter() {
            ecs.delete_entity(*del).expect("Delete failed");
        }
    }

    let data = fs::read_to_string(SAVEGAME_FILE_NAME).unwrap();
    let mut de = serde_json::Deserializer::from_str(&data);

    {
        let mut d = (
            &mut ecs.entities(), 
            &mut ecs.write_storage::<SimpleMarker<SerializeMe>>(), 
            &mut ecs.write_resource::<SimpleMarkerAllocator<SerializeMe>>()
        );

        deserialize_individually!(ecs, de, d, Position, Renderable, Player, Viewshed, Monster, 
            Name, BlocksTile, CombatStats, SufferDamage, WantsToMelee, Item, Consumable, Ranged, InflictsDamage, 
            AreaOfEffect, Confusion, ProvidesHealing, InBackpack, WantsToPickupItem, WantsToUseItem,
            WantsToDropItem, SerializationHelper
        );
    }

    let mut deleteme: Option<Entity> = None;
    {
        let entities = ecs.entities();
        let helper = ecs.read_storage::<SerializationHelper>();
        let player = ecs.read_storage::<Player>();
        let position = ecs.read_storage::<Position>();
        for (e, h) in (&entities, &helper).join() {
            let mut worldmap = ecs.write_resource::<Map>();
            *worldmap = h.map.clone();
            worldmap.tile_content = vec![Vec::new(); MAPCOUNT];
            deleteme = Some(e);
        }

        for (e,_p,pos) in (&entities, &player, &position).join() {
            let mut ppos = ecs.write_resource::<rltk::Point>();
            *ppos = rltk::Point::new(pos.x, pos.y);
            let mut player_resource = ecs.write_resource::<Entity>();
            *player_resource = e;
        }
    }
    ecs.delete_entity(deleteme.unwrap()).expect("Unable to delete helper");
}

pub fn delete_save() {
    if does_save_exist() {
        fs::remove_file(SAVEGAME_FILE_NAME).expect("Unable to delete file");
    }
}
