// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        // health<u32> so it cannot be negative
        if self.health == u32::MIN {
            Some(Self {
                // full refill with current level
                health: 100,
                mana: if self.level >= 10 { Some(100) } else { None },
                level: self.level,
            })
        }
        // otherwise Player is alive
        else {
            None
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        // getting and mathcing mana_value within without using unwrap()
        match self.mana {
            Some(mana_value) => {
                if mana_cost > mana_value {
                    u32::MIN
                } else {
                    self.mana = Some(mana_value - mana_cost);
                    mana_cost * 2
                }
            }
            None => {
                // health<u32> cannot be negative
                if self.health > mana_cost {
                    self.health -= mana_cost;
                // u32::MIN = 0
                } else {
                    self.health = u32::MIN;
                }
                // in both cases we return 0
                u32::MIN
            }
        }
    }
}
