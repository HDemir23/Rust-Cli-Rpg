/// Runtime state for the player across dungeon encounters.
pub struct Player {
    pub name: String,
    pub health: i32,
    pub health_amount: i32,
    pub max_health: i32,
    pub damage: i32,
    pub gold: u32,
    pub level: u32,
    pub xp: u32,
    pub xp_to_next_level: u32,
}

pub enum PlayerAction {
    Attack,
    Heal,
    Run,
    Quit,
}

/// Enemy rarity tier used for stat and reward scaling.
#[derive(Debug)]
pub enum EnemyRarity {
    Common,
    Rare,
    Elite,
    Boss,
}

/// Runtime state for the active enemy encounter.
#[derive(Debug)]
pub struct Enemy {
    pub name: String,
    pub level: u32,
    pub health: i32,
    pub damage: i32,
    pub gold_reward: u32,
    pub xp_reward: u32,
    pub rarity: EnemyRarity,
}

/// Base template used before level, rarity, and modifier scaling.
pub struct EnemyArchetype {
    pub name: &'static str,
    pub base_health: i32,
    pub base_damage: i32,
    pub base_gold: u32,
    pub base_xp: u32,
}

pub enum TurnResult {
    Continue,
    EndBattle(BattleOutcome),
}

pub struct BattleTurnResult {
    pub outcome: Option<BattleOutcome>,
    pub messages: Vec<String>,
}
pub enum BattleOutcome {
    Victory,
    Defeat,
    Fled,
    Quit,
}

pub struct Dungeon {
    pub current_room: u32,
    pub max_room: u32,
}


impl Dungeon {
    pub fn new(level: u32) -> Self {
        Self {
            current_room: 1,
            max_room: 3 + level,
        }
    }

    pub fn is_boss_room(&self) -> bool {
        self.current_room == self.max_room
    }

    pub fn is_cleared(&self) -> bool {
        self.current_room > self.max_room
    }

    pub fn advance_room(&mut self) {
        self.current_room += 1;
    }
}
