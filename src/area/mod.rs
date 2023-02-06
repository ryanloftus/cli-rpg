mod caves;
mod forest;
mod icefields;
mod islands;
mod kingdom;
mod mountains;
mod plains;

use serde::{Deserialize, Serialize};

use crate::enemy::Enemy;
use crate::prompt::PromptOption;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Area {
    Kingdom = 0,
    Plains,
    Forest,
    Islands,
    Caves,
    Icefields,
    Mountains,
}

impl Area {
    pub fn area_at(idx: usize) -> Area {
        return match idx {
            0 => Area::Kingdom,
            1 => Area::Plains,
            2 => Area::Forest,
            3 => Area::Islands,
            4 => Area::Caves,
            5 => Area::Icefields,
            6 => Area::Mountains,
            _ => panic!("Invalid area idx"),
        };
    }

    // TODO: add stat multipliers for bosses in each area
    pub fn story(&self) -> Vec<StoryComponent> {
        return match self {
            Area::Kingdom => kingdom::story(),
            Area::Plains => plains::story(),
            Area::Forest => forest::story(),
            Area::Islands => islands::story(),
            Area::Caves => caves::story(),
            Area::Icefields => icefields::story(),
            Area::Mountains => mountains::story(),
        };
    }
}

impl PromptOption for Area {
    fn option_name(&self) -> String {
        return String::from(match self {
            Area::Kingdom => "The Kingdom",
            Area::Plains => "The Plains",
            Area::Forest => "The Forest",
            Area::Islands => "The Islands",
            Area::Caves => "The Caves",
            Area::Icefields => "The Icefields",
            Area::Mountains => "The Mountains",
        });
    }

    fn short_option_name(&self) -> Option<String> {
        return Some(String::from(match self {
            Area::Kingdom => "K",
            Area::Plains => "P",
            Area::Forest => "F",
            Area::Islands => "I",
            Area::Caves => "C",
            Area::Icefields => "I",
            Area::Mountains => "M",
        }));
    }
}

#[derive(Debug, Clone)]
pub enum StoryComponent {
    Text(String),
    Enemy(Enemy),
    Boss(Enemy),
    // TODO: GainAttribute once player attributes are added (i.e. DemonSlayer after progressing forest)
}

pub const NUM_AREAS: usize = 7;
