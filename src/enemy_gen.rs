use crate::structs::{Enemy, EnemyArchetype, EnemyRarity, Player};
use rand::seq::IndexedRandom;
use rand::RngExt;


/// Creates an enemy scaled around the current player level.
pub fn gen_enemy(player: &Player, dungeon_level: u32, is_boss_room: bool) -> Enemy {
    let mut rng = rand::rng();

    let archetypes = [
        EnemyArchetype {
            name: "Goblin",
            base_health: 35,
            base_damage: 10,
            base_gold: 8,
            base_xp: 15,
        },
        EnemyArchetype {
            name: "Skeleton",
            base_health: 45,
            base_damage: 10,
            base_gold: 12,
            base_xp: 18,
        },
        EnemyArchetype {
            name: "Orc",
            base_health: 70,
            base_damage: 15,
            base_gold: 20,
            base_xp: 25,
        },
        EnemyArchetype {
            name: "Wolf",
            base_health: 40,
            base_damage: 13,
            base_gold: 10,
            base_xp: 17,
        },
    ];

    let selected = archetypes
        .choose(&mut rng)
        .expect("Enemy arcetypes is empty");

    let base_level = player.level.max(dungeon_level); // i am dumb af
    let enemy_level = gen_enemy_level(base_level);


    let rarity = if is_boss_room {
        gen_rarity()
    } else {
        EnemyRarity::Boss
    };


    let modifier = gen_modifiers();

    let dungeon_multipilier = dungeon_level as i32;
    let level_multipilier = enemy_level as i32;

    let mut health = selected.base_health + level_multipilier * rng.random_range(5..=12);
    let mut damage = selected.base_damage + level_multipilier * rng.random_range(1..=4);
    let mut gold_reward = selected.base_gold + enemy_level * rng.random_range(3..=8);
    let mut xp_reward: u32 = selected.base_xp + enemy_level * rng.random_range(9..=14);

    rarity_bonus(&rarity, &mut health, &mut damage, &mut gold_reward, &mut xp_reward);
    apply_modifiers(&modifier, &mut health, &mut damage, &mut gold_reward, &mut xp_reward);


    health += dungeon_multipilier * rng.random_range(8..=16);
    damage += dungeon_multipilier * rng.random_range(1..=3);
    gold_reward += dungeon_level * rng.random_range(5..=12);
    xp_reward += dungeon_level * rng.random_range(8..=16);


    Enemy {
        name: format!("{} {}", modifier, selected.name),
        level: enemy_level,
        health,
        damage,
        gold_reward,
        rarity,
        xp_reward,
    }
}

/// Picks an enemy level close to the player's current level.
fn gen_enemy_level(player_level: u32) -> u32 {
    let mut rng = rand::rng();

    let min_level = if player_level > 2 {
        player_level - 2
    } else {
        1
    };

    let max_level = player_level + 2;

    rng.random_range(min_level..=max_level)
}

/// Rolls the rarity tier for a newly generated enemy.
fn gen_rarity() -> EnemyRarity {
    let mut rng = rand::rng();
    let roll = rng.random_range(1..=100);

    match roll {
        1..=70 => crate::structs::EnemyRarity::Common,
        71..=90 => crate::structs::EnemyRarity::Rare,
        91..=98 => crate::structs::EnemyRarity::Elite,
        _ => crate::structs::EnemyRarity::Boss,
    }
}

/// Applies stat and reward scaling based on rarity.
fn rarity_bonus(rarity: &EnemyRarity, health: &mut i32, damage: &mut i32, gold_reward: &mut u32, xp_reward: &mut u32) {
    match rarity {
        crate::structs::EnemyRarity::Common => {}
        crate::structs::EnemyRarity::Rare => {
            *health = (*health as f32 * 1.25) as i32;
            *damage = (*damage as f32 * 1.25) as i32;
            *gold_reward = (*gold_reward as f32 * 1.50) as u32;
            *xp_reward = (*xp_reward as f32 * 1.35) as u32;
        }
        crate::structs::EnemyRarity::Elite => {
            *health = (*health as f32 * 1.75) as i32;
            *damage = (*damage as f32 * 1.50) as i32;
            *gold_reward = (*gold_reward as f32 * 2.50) as u32;
            *xp_reward = (*xp_reward as f32 * 2.00) as u32;
        }
        crate::structs::EnemyRarity::Boss => {
            *health = (*health as f32 * 2.50) as i32;
            *damage = (*damage as f32 * 2.00) as i32;
            *gold_reward = (*gold_reward as f32 * 4.00) as u32;
            *xp_reward = (*xp_reward as f32 * 3.50) as u32;
        }
    }
}

/// Picks a text modifier that also affects enemy stats.
fn gen_modifiers() -> &'static str {
    let mut rng = rand::rng();

    let modifiers = [
        "Angry", "Wild", "Cursed", "Ancient", "Hungry", "Dark", "Broken",
    ];

    modifiers.choose(&mut rng).expect("modifiers is empty")
}

/// Applies stat and reward changes for the selected modifier.
fn apply_modifiers(modifier: &str, health: &mut i32, damage: &mut i32, gold_reward: &mut u32, xp_reward: &mut u32) {
    match modifier {
        "Angry" => {
            *damage += 5;
            *xp_reward += 5;
        }

        "Wild" => {
            *health += 10;
            *damage += 2;
            *xp_reward += 4;
        }

        "Cursed" => {
            *damage += 8;
            *health -= 5;
            *xp_reward += 8;
            *gold_reward += 3;
        }

        "Ancient" => {
            *health += 25;
            *gold_reward += 10;
            *xp_reward += 10;
        }

        "Hungry" => {
            *damage += 3;
            *xp_reward += 3;
        }

        "Dark" => {
            *health += 15;
            *damage += 4;
            *xp_reward += 7;
            *gold_reward += 5;
        }

        "Broken" => {
            *health -= 10;
            *gold_reward += 5;
            *xp_reward += 2;
        }

        _ => {}
    }
}
