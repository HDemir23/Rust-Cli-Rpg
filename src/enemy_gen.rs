use rand::RngExt;
use rand::seq::IndexedRandom;
use crate::structs::{Enemy, EnemyRarity, Player, EnemyArchetype};




    pub fn gen_enemy(player: &Player) -> Enemy {
    let mut rng = rand::rng();

    let archetypes = [
        EnemyArchetype {
            name: "Goblin",
            base_health: 35,
            base_damage: 10,
            base_gold: 8,
        },
        EnemyArchetype {
            name: "Skeleton",
            base_health: 45,
            base_damage: 10,
            base_gold: 12,
        },
        EnemyArchetype {
            name: "Orc",
            base_health: 70,
            base_damage: 15,
            base_gold: 20,
        },
        EnemyArchetype {
            name: "Wolf",
            base_health: 40,
            base_damage: 13,
            base_gold: 10,
        },
    ];

    let selected = archetypes
        .choose(&mut rng)
        .expect("Enemy arcetypes is empty");

    let enemy_level = gen_enemy_level(player.level);
    let rarity = gen_rarity();
    let modifier = gen_modifiers();

    let level_multipilier = enemy_level as i32;

    let mut health = selected.base_health + level_multipilier * rng.random_range(5..=12);
    let mut damage = selected.base_damage + level_multipilier * rng.random_range(1..=4);
    let mut gold_reward = selected.base_gold + enemy_level * rng.random_range(2..=6);

    rarity_bonus(&rarity, &mut health, &mut damage, &mut gold_reward);
    apply_modifiers(&modifier, &mut health, &mut damage, &mut gold_reward);

    Enemy {
        name: format!("{} {}", modifier, selected.name),
        level: enemy_level,
        health,
        damage,
        gold_reward,
        rarity
    }
}



    fn gen_enemy_level (player_level: u32) -> u32 {
        let mut rng = rand::rng();

        let min_level =
            if player_level > 2 {
            player_level - 2
        } else { 1 };

        let max_level = player_level + 2;

        rng.random_range(min_level..=max_level)
    }

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

    fn rarity_bonus(
        rarity: &EnemyRarity,
        health: &mut i32,
        damage: &mut i32,
        gold_reward: &mut u32,
    ) {
    match rarity {
        crate::structs::EnemyRarity::Common => {}
        crate::structs::EnemyRarity::Rare => {
            *health = (*health as f32 * 1.25) as i32;
            *damage = (*damage as f32 * 1.25) as i32; 
            *gold_reward = (*gold_reward as f32 * 1.50) as u32;
        }
        crate::structs::EnemyRarity::Elite => {
            *health = (*health as f32 * 1.75) as i32;
            *damage = (*damage as f32 * 1.50) as i32;
            *gold_reward = (*gold_reward as f32 * 2.50) as u32;
        }
        crate::structs::EnemyRarity::Boss => {
            *health = (*health as f32 * 2.50) as i32;
            *damage = (*damage as f32 * 2.00) as i32;
            *gold_reward = (*gold_reward as f32 * 4.00) as u32;
        }
    }
}


    fn gen_modifiers() -> &'static str {
        let mut rng = rand::rng();

        let modifiers = [
            "Angry",
            "Wild",
            "Cursed",
            "Ancient",
            "Hungry",
            "Dark",
            "Broken",
        ];

        modifiers
            .choose(&mut rng)
            .expect("modifiers is empty")


    }

    fn apply_modifiers(
        modifier: &str,
        health: &mut i32,
        damage: &mut i32,
        gold_reward: &mut u32,
    ) {
        match modifier {
            "Angry" => *damage += 5,
            "Wild" => {
                *health += 10;
                *damage += 2;
            },
            "Cursed" => {
                *damage += 8;
                *health -= 5;
            },
            "Ancient" => {
                *health += 25;
                *gold_reward += 10;
            },
            "Hungry" => {
                *damage += 3;
            },
            "Dark" => {
                *health += 15;
                *damage += 4;
            },
            "Broken" => {
                *health -= 10;
                *gold_reward += 5;
            },
            _ => {}
        }
    }


    pub fn check_level_up (player: &mut Player) {
        while player.xp >= player.xp_to_next_level {
            player.xp -= player.xp_to_next_level;
            player.level += 1;

            player.max_health += 20;
            player.health = player.max_health;
            player.damage += 5;
            player.xp_to_next_level += player.level * 50;

            println!("LEVEL UP!");
            println!("You are now level {}", player.level);
            println!("Max health increased to {}", player.max_health);
            println!("Damage increased to {}", player.damage);

        }
    }
