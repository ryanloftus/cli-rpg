const EXPERIENCE_PER_LEVEL: u8 = 100;
const EXPERIENCE_FOR_ENEMY_DEFEATED: u8 = 5;
const EXPERIENCE_FOR_BOSS_DEFEATED: u8 = 100;
const EXPERIENCE_FOR_AREA_CLEARED: u8 = 100;

#[derive(Debug, Clone)]
pub struct Experience {
    pub level: u16,
    pub experience_towards_next_level: u8,
}

impl Experience {
    fn add_experience(mut self, experience: u8) {
        self.experience_towards_next_level = self.experience_towards_next_level + experience;
        while self.experience_towards_next_level >= EXPERIENCE_PER_LEVEL {
            self.level = self.level + 1;
            self.experience_towards_next_level = self.experience_towards_next_level - EXPERIENCE_PER_LEVEL;
        }
    }

    pub fn enemies_defeated(self, num_enemies: u8) {
        self.add_experience(EXPERIENCE_FOR_ENEMY_DEFEATED * num_enemies);
    }

    pub fn boss_defeated(self) {
        self.add_experience(EXPERIENCE_FOR_BOSS_DEFEATED);
    }

    pub fn area_cleared(self) {
        self.add_experience(EXPERIENCE_FOR_AREA_CLEARED);
    }
}
