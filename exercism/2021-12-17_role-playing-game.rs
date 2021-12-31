// https://github.com/exercism/rust/tree/630a56517c9015341cdd96233632eda9ad8f44c4/exercises/concept/role-playing-game
// https://exercism.org/tracks/rust/exercises/role-playing-game/solutions/mrl5

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health == 0 {
            return Some(get_new_player(self.level));
        }
        return None;
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(x) =>
                if x >= mana_cost {
                    self.mana = Some(x - mana_cost);
                    return mana_cost * 2;
                } else {
                    return 0;
                },
            None =>
                if mana_cost >= self.health {
                    self.health = 0;
                    return 0;
                } else {
                    self.health -= mana_cost;
                    return 0;
                }
        }
    }
}

fn get_new_player(level: u32) -> Player {
    Player {
        health: 100,
        mana: if can_have_mana(level) { Some(100) } else { None },
        level,
    }
}

fn can_have_mana(level: u32) -> bool {
    level >= 10
}
