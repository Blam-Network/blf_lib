mod sandbox;
mod avalanche;
mod last_resort;
mod construct;
mod foundry;
mod deadlock;
mod guardian;
mod isolation;
mod valhalla;
mod epitaph;
mod snowbound;
mod narrows;
mod the_pit;
mod sandtrap;
mod standoff;
mod longshore;
mod assembly;
mod orbital;
mod blackout;
mod rats_nest;
mod ghost_town;
mod cold_storage;
mod heretic;
mod citadel;

pub fn get_h3_index_for_mcc_object(map_id: u32, mcc_object_index: u32) -> Option<u32> {
    match map_id {
        30 => {
            last_resort::last_resort_objects_map.get_by_right(&mcc_object_index).cloned()
        }
        300 => {
            construct::construct_objects_map.get_by_right(&mcc_object_index).cloned()
        }
        310 => {
            deadlock::deadlock_objects_map.get_by_right(&mcc_object_index).cloned()
        }
        320 => {
            guardian::guardian_objects_map.get_by_right(&mcc_object_index).cloned()
        }
        330 => {
            isolation::isolation_objects_map.get_by_right(&mcc_object_index).cloned()
        }
        340 => {
            valhalla::valhalla_objects_map.get_by_right(&mcc_object_index).cloned()
        }
        350 => {
            epitaph::epitaph_objects_map.get_by_right(&mcc_object_index).cloned()
        }
        360 => {
            snowbound::snowbound_objects_map.get_by_right(&mcc_object_index).cloned()
        }
        380 => {
            narrows::narrows_objects_map.get_by_right(&mcc_object_index).cloned()
        }
        390 => {
            the_pit::the_pit_objects_map.get_by_right(&mcc_object_index).cloned()
        }
        400 => {
            sandtrap::sandtrap_objects_map.get_by_right(&mcc_object_index).cloned()
        }
        410 => {
            standoff::standoff_objects_map.get_by_right(&mcc_object_index).cloned()
        }
        440 => {
            longshore::longshore_objects_map.get_by_right(&mcc_object_index).cloned()
        }
        470 => {
            avalanche::avalanche_objects_map.get_by_right(&mcc_object_index).cloned()
        }
        480 => {
            foundry::foundry_objects_map.get_by_right(&mcc_object_index).cloned()
        }
        490 => {
            assembly::assembly_objects_map.get_by_right(&mcc_object_index).cloned()
        }
        500 => {
            orbital::orbital_objects_map.get_by_right(&mcc_object_index).cloned()
        }
        520 => {
            blackout::blackout_objects_map.get_by_right(&mcc_object_index).cloned()
        }
        580 => {
            rats_nest::rats_nest_objects_map.get_by_right(&mcc_object_index).cloned()
        }
        590 => {
            ghost_town::ghost_town_objects_map.get_by_right(&mcc_object_index).cloned()
        }
        600 => {
            cold_storage::cold_storage_objects_map.get_by_right(&mcc_object_index).cloned()
        }
        720 => {
            heretic::heretic_objects_map.get_by_right(&mcc_object_index).cloned()
        }
        730 => {
            sandbox::sandbox_objects_map.get_by_right(&mcc_object_index).cloned()
        },
        740 => {
            citadel::citadel_objects_map.get_by_right(&mcc_object_index).cloned()
        },
        _ => {
            panic!("No MCC object mapping for Map {}", map_id);
        }
    }
}