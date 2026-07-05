

pub struct Player {
    pub name: String,
    pub health: i32,
    pub damage: i32,
    pub gold: u32,
    pub level: u32,
}
#[derive(Debug)]
pub enum EnemyRarity {
    Common,
    Rare,
    Elite,
    Boss
}
#[derive(Debug)]
pub struct Enemy {
    pub name: String,
    pub level: u32,
    pub health: i32,
    pub damage: i32,
    pub gold_reward: u32,
    pub rarity: EnemyRarity,
}
pub struct EnemyArchetype {
    pub name:&'static str,
    pub base_health: i32,
    pub base_damage: i32,
    pub base_gold: u32,
}




