use std::ops::Index;
use json::JsonValue;
use regex::{Regex};
use crate::nuklid::{Nuklid, ZerfallsArt};
use crate::nuklid::ZerfallsArt::{Alpha, BetaMinus, BetaPlus, Stable, Unknown};

pub fn deserialize(string: &str) -> Vec<Nuklid> {
    let parsed = json::parse(string).unwrap();

    let json_array: &JsonValue = parsed.entries().next().unwrap().1;

    let mut nuk_vec: Vec<Nuklid> = vec![];
    for i in 0..json_array.len() {
        let element: &JsonValue = json_array.index(i);
        let nuklid_struct: Nuklid = translate_to_struct(element);
        nuk_vec.push(nuklid_struct);
    }
    nuk_vec
}

fn translate_to_struct(element: &JsonValue) -> Nuklid {
    let mut vec: Vec<(&str, &JsonValue)> = vec![];
    for entry in element.entries() {
        vec.push(entry);
    }

    Nuklid {
        name: Box::from(vec[2].1.to_string()),
        neutronen: vec[1].1.as_i32().unwrap(),
        protonen: vec[0].1.as_i32().unwrap(),
        life: Box::from(vec[3].1.to_string()),
        zerfalls_art: translate_zerfalls_art(vec[4].1),
    }
}

fn translate_zerfalls_art(input: &JsonValue) -> ZerfallsArt {
    let pattern: Regex = Regex::new(r"(b-)|a|d|(b\+)").unwrap();
    let first_match = pattern.find(&input.as_str().unwrap());
    if first_match.is_none() {
        return Unknown;
    }
    let first_str: &str = first_match.unwrap().as_str();

    return match first_str {
        "a" => Alpha,
        "b-" => BetaMinus,
        "b+" => BetaPlus,
        "d" => Stable,
        &_ => Unknown,
    }

    //DeadCode
    // if first_str == "a" {
    //     return Alpha;
    // } else if first_str == "b-" {
    //     return BetaMinus;
    // } else if first_str == "b+" {
    //     return BetaPlus;
    // } else if first_str == "d" {
    //     return Stable;
    // }
    // Unknown
}