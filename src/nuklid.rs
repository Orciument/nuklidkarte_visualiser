#![deny(unsafe_code)]

use std::fmt::{Display, Formatter};

use nannou::color::{BLACK, DIMGRAY, WHITE};
use nannou::prelude::{Point2, Srgb};
use nannou::text::FontSize;
use nannou::Draw;

use crate::nuklid::ZerfallsArt::*;
use crate::subsup::subsup::super_ignore_unable;

pub const BACKGROUND: Srgb<u8> = BLACK;
pub const OUTER_SCALE: f32 = 0.95;
pub const INNER_SCALE: f32 = 0.82;

#[derive(Debug, Clone)]
pub struct Nuklid {
    pub zerfalls_art: ZerfallsArt,
    pub name: String,
    pub life: String,
    pub neutronen: u8,
    pub protonen: u8,
}

impl Display for Nuklid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let number = self.neutronen as u16 + self.protonen as u16;
        write!(f, "{}", super_ignore_unable(number.to_string()) + &self.name)
    }
}

impl Nuklid {
    pub fn draw(&self, draw: &Draw, square_size: &f32, overwrite_x_y: Option<(f32, f32)>) {
        let xy = match overwrite_x_y {
            None => (
                self.neutronen as f32 * square_size + (square_size * 0.5),
                self.protonen as f32 * square_size + (square_size * 0.5)
            ),
            Some(x) => x
        };
        // draw.quad()
        //     .w_h(square_size, square_size)
        //     .x_y(x,y)
        //     .color(DIMGRAY);

        let outer_square_w_h = square_size * OUTER_SCALE;
        draw.quad()
            .w_h(outer_square_w_h, outer_square_w_h)
            .xy(Point2::from(xy))
            .z(10.)
            .color(self.zerfalls_art.color());

        let inner_square_w_h = square_size * INNER_SCALE;
        draw.quad()
            .w_h(inner_square_w_h, inner_square_w_h)
            .xy(Point2::from(xy))
            .z(20.)
            .color(BACKGROUND);

        let number = self.neutronen as u16 + self.protonen as u16;
        draw.text(&*(super_ignore_unable(number.to_string()) + &self.name))
            .xy(Point2::from(xy))
            .center_justify()
            .font_size((square_size * 0.3) as FontSize)
            .z(20.)
            .color(WHITE);
    }

    pub const fn abs_child(&self) -> (u8, u8) {
        (
            (self.protonen as i8 + self.zerfalls_art.delta_prot()) as u8,
            (self.neutronen as i8 + self.zerfalls_art.delta_neut()) as u8
        )
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
            Unknown => 0,
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
            Unknown => 0,
        };
    }

    pub fn color(&self) -> Srgb<u8> {
        return match self {
            Alpha => Srgb {
                red: 254,
                green: 255,
                blue: 69,
                standard: Default::default(),
            },
            BetaPlus => Srgb {
                red: 238,
                green: 58,
                blue: 250,
                standard: Default::default(),
            },
            BetaPlus2 => BetaPlus.color(),
            P => Srgb {
                red: 237,
                green: 47,
                blue: 32,
                standard: Default::default(),
            },
            P2 => P.color(),
            BetaPlusP => BetaPlus.color(),
            BetaMinus => Srgb {
                red: 65,
                green: 106,
                blue: 249,
                standard: Default::default(),
            },
            BetaMinus2 => BetaMinus.color(),
            N => Srgb {
                red: 118,
                green: 200,
                blue: 72,
                standard: Default::default(),
            },
            N2 => N.color(),
            BetaMinusN => BetaMinus.color(),
            BetaMinusN2 => BetaMinus.color(),
            SF => Srgb {
                red: 220,
                green: 138,
                blue: 162,
                standard: Default::default(),
            },
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
            &_ => {
                /*eprintln!("Rest Triggert: {}", str);*/
                Unknown
            }
        };
    }
}

impl Display for ZerfallsArt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
