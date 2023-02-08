#![deny(unsafe_code)]

use std::collections::HashMap;
use std::ops::Index;

use json::JsonValue;

use crate::datastring;
use crate::nuklid::{Nuklid, ZerfallsArt};

//Advanced Nuklid Struct
pub fn deserialize_ad_to_map() -> HashMap<u8, HashMap<u8, Nuklid>> {
    let mut y_achse_map: HashMap<u8, HashMap<u8, Nuklid>> = HashMap::new();

    // Get the Array Containing the Nuklids
    let json_array: JsonValue = json::parse(datastring::TEXT).unwrap();

    for i in 0..json_array.len() {
        //Get Nuklid Json out of the Nuklid Json Array
        let element: &JsonValue = json_array.index(i);
        //Parse JSON to Nuklid Struct
        let nuklid_struct: Nuklid = translate_to_ad_struct(element);
        //Insert Nuklid into the correct Hashmaps
        let protonen = nuklid_struct.protonen;

        //Insert Nuklid into the correct Hashmaps
        if !y_achse_map.contains_key(&protonen) {
            //If no Map for the current number of Protons exists then we create a new Hashmap for
            //the Isotopes of this Element
            let new_x_achse_map: HashMap<u8, Nuklid> = HashMap::new();
            y_achse_map.insert(*&protonen, new_x_achse_map);
        }

        let x_achse_map: &mut HashMap<u8, Nuklid> = y_achse_map.get_mut(&protonen).unwrap();
        x_achse_map.insert(nuklid_struct.neutronen, nuklid_struct);
    }
    y_achse_map
}

fn translate_to_ad_struct(element: &JsonValue) -> Nuklid {
    let mut vec: Vec<(&str, &JsonValue)> = vec![];
    for entry in element.entries() {
        vec.push(entry);
    }

    //TODO Bunch of unsafe unwraps
    Nuklid {
        name: vec[2].1.to_string(),
        neutronen: vec[1].1.as_u8().unwrap(), //n
        protonen: vec[0].1.as_u8().unwrap(), //z
        life: vec[3].1.to_string(),
        zerfalls_art: ZerfallsArt::parse_from_string(vec[4].1.as_str().unwrap()),
    }
}