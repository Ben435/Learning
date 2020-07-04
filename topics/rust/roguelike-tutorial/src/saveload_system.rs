use specs::prelude::*;
use specs::saveload::{SimpleMarker,SerializeComponents,MarkedBuilder};
use specs::error::NoError;
use specs::World;
use std::fs::File;
use std::path::Path;
use crate::map::Map;
use crate::components::*;

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

pub fn save_game(ecs: &mut World) {
    let mapcopy = ecs.get_mut::<Map>().unwrap().clone();
    let savehelper = ecs
        .create_entity()
        .with(SerializationHelper{ map: mapcopy })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();

    {
        let data = ( ecs.entities(), ecs.read_storage::<SimpleMarker<SerializeMe>>() );

        let writer = File::create(SAVEGAME_FILE_NAME).unwrap();
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
