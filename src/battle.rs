use crate::read_input::read_input;
use crate::structs::{Enemy, Player};

pub fn battle (player: &mut Player, enemy: &mut Enemy) {

    // create a loop for battle
    loop {

        println!("Player: {}, Health: {}", player.name, player.health);

        println!("Enemy: {} Health: {} , Damage: {}", enemy.name ,enemy.health ,enemy.damage);
        // get user input
           let command = read_input("command: attack => a / heal => e / run => r / quit q" );

        // match user inputs for the events
        match command.as_ref() {
            "a" => {
                enemy.health -= player.damage;
                println!("You attacked {} for {} damage", enemy.name, player.damage);
            },
            "e" => {
                player.health += 15;
                println!("Healed 15 health: {}", player.health);
            },
            "r" => {
                println!("you run away");
                break
            },
            "q" => break,
            _ => println!("Unknown command: {}", command)
        }

        // if enemy health <= 0 gain gold and move to the next enemy
        if enemy.health <= 0 {
            player.gold += enemy.gold_reward;
            player.health += 20;
            player.level += 1;


            println!("You defeated {}", enemy.name);
            println!("Gold gained: {}", enemy.gold_reward);
            println!("Total gold: {}", player.gold);
            println!("20 health restored");
            println!("Health: {}", player.health);
            println!("Level up! Current level: {}", player.level);

            break
        }


        player.health -= enemy.damage;
        println!("Ouch! health: {}", player.health);

        // game over logic based on user health
        if player.health <= 0 {
            break;
        }
    }
}

