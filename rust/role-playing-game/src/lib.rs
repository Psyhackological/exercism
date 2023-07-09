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
        // Check if player is dead
        if self.health > 0 {
            return None;
        }

        // Create a new player with restored health
        let mut new_player = Player {
            health: 100,
            // Mana is restored only if player level is 10 or higher
            mana: if self.level >= 10 { Some(100) } else { None },
            level: self.level,
        };

        Some(new_player)
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        // Player doesn't have a mana pool
        if self.mana.is_none() {
            self.health = self.health.saturating_sub(mana_cost);
            return 0;
        }

        // Player has a mana pool but insufficient mana
        if self.mana.unwrap() < mana_cost {
            return 0;
        }

        // Player has sufficient mana, cast the spell
        self.mana = Some(self.mana.unwrap() - mana_cost);
        mana_cost * 2
    }
}
