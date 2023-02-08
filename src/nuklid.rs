#![deny(unsafe_code)]

use std::fmt::{Display, Formatter};

use nannou::color::{DIMGRAY, WHITE};
use nannou::prelude::Srgb;

use crate::nuklid::ZerfallsArt::*;
use crate::subsup::super_ignore_unable;

#[derive(Debug)]
pub struct Nuklid {
    pub zerfalls_art: ZerfallsArt,
    pub name: Box<str>,
    pub life: Box<str>,
    pub neutronen: u8,
    pub protonen: u8,
}

impl Display for Nuklid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let number = self.neutronen as u16 + self.protonen as u16;
        write!(f, "{}", super_ignore_unable(number.to_string()) + &self.name)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum ZerfallsArt {
    Alpha,
    BetaPlus,
    BetaPlus2,
    P,
    P2,
    BetaPlusP,
    BetaMinus,
    BetaMinus2,
    N,
    N2,
    BetaMinusN,
    BetaMinusN2,
    SF,
    Stable,
    Unknown,
}

#[allow(dead_code)]
impl ZerfallsArt {
    pub const fn delta_prot(&self) -> i8 {
        return match *self {
            Alpha => -2,
            BetaPlus => -1,
            BetaPlus2 => -2,
            P => -1,
            P2 => -2,
            BetaPlusP => -2,
            BetaMinus => 1,
            BetaMinus2 => 2,
            N => 0,
            N2 => 0,
            BetaMinusN => 1,
            BetaMinusN2 => 2,
            SF => 0,
            Stable => 0,
            Unknown => 0
        };
    }

    pub const fn delta_neut(&self) -> i8 {
        return match *self {
            Alpha => -2,
            BetaPlus => 1,
            BetaPlus2 => 2,
            P => 0,
            P2 => 0,
            BetaPlusP => 1,
            BetaMinus => -1,
            BetaMinus2 => -2,
            N => -1,
            N2 => -2,
            BetaMinusN => -2,
            BetaMinusN2 => -3,
            SF => 0,
            Stable => 0,
            Unknown => 0
        };
    }

    pub fn color(&self) -> Srgb<u8> {
        return match self {
            Alpha => Srgb { red: 254, green: 255, blue: 69, standard: Default::default() },
            BetaPlus => Srgb { red: 238, green: 58, blue: 250, standard: Default::default() },
            BetaPlus2 => BetaPlus.color(),
            P => Srgb { red: 237, green: 47, blue: 32, standard: Default::default() },
            P2 => P.color(),
            BetaPlusP => BetaPlus.color(),
            BetaMinus => Srgb { red: 65, green: 106, blue: 249, standard: Default::default() },
            BetaMinus2 => BetaMinus.color(),
            N => Srgb { red: 118, green: 200, blue: 72, standard: Default::default() },
            N2 => N.color(),
            BetaMinusN => BetaMinus.color(),
            BetaMinusN2 => BetaMinus.color(),
            SF => Srgb { red: 220, green: 138, blue: 162, standard: Default::default() },
            Stable => WHITE,
            Unknown => DIMGRAY,
        };
    }

    pub fn parse_from_string(str: &str) -> ZerfallsArt {
        return match str {
            "B-" => BetaMinus,
            "N" => N,
            "2N" => N2,
            "P" => P,
            "A" => Alpha,
            "EC" => BetaPlus,
            "EC+B+" => BetaPlus,
            "2P" => P2,
            "B+P" => BetaPlusP,
            "B-N" => BetaMinusN,
            "B-2N" => BetaMinusN2,
            "B+" => BetaPlus,
            "2B-" => BetaMinus2,
            "2EC" => BetaPlus2,
            "2B+" => BetaPlus2,
            "SF" => SF,
            "S" => Stable,
            &_ => { /*eprintln!("Rest Triggert: {}", str);*/ Unknown }
        };
    }
}

impl Display for ZerfallsArt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}