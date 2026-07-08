pub mod battle;
pub mod enemy_gen;
pub mod player_progression;
mod read_input;
pub mod structs;
pub mod dungeon;
pub mod game_state;
mod ui;

use crate::game_state::App;
use crate::structs::PlayerAction;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use read_input::read_input;
use structs::Player;

fn main() -> std::io::Result<()> {
    let player = create_player();

    run_tui_loop(player)
    // run_app_loop(player);
    // run_game_loop(&mut player);
}

fn create_player() -> Player {
    Player {
        name: read_input("Enter Ur name"),
        health: 100,
        health_amount: 25,
        max_health: 100,
        damage: 25,
        gold: 0,
        level: 1,
        xp: 0,
        xp_to_next_level: 35,
    }
}


fn run_tui_loop(player: Player) -> std::io::Result<()> {
    let mut app = App::new(player);

    ratatui::run(|terminal| {
        loop {
            app.spawn_next_enemy();

            terminal.draw(|frame| {
                ui::draw(frame, &app);
            })?;

            if app.should_quit {
                break Ok(());
            };


            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                let action = match key.code {
                    KeyCode::Char('a') => Some(PlayerAction::Attack),
                    KeyCode::Char('e') => Some(PlayerAction::Heal),
                    KeyCode::Char('r') => Some(PlayerAction::Run),
                    KeyCode::Char('q') => Some(PlayerAction::Quit),
                    KeyCode::Esc => {
                        app.should_quit = true;
                        None
                    }
                    _ => None,
                };
                if let Some(action) = action {
                    app.handle_battle_action(action);
                }
            }
        }
    })
}

// fn run_game_loop(player: &mut Player) {
//     let mut dungeon_level = 1;
//
//     loop {
//         let dungeon_res = run_dungeon(player, dungeon_level);
//
//         match dungeon_res {
//             DungeonOutcome::Cleared => {
//                 dungeon_level += 1;
//
//                 if !should_continue_playing() {
//                     println!("Exiting Dungeon");
//                     break;
//                 }
//             }
//             DungeonOutcome::PlayerDied => {
//                 print_game_over(player);
//                 break;
//             }
//             DungeonOutcome::Left => {
//                 println!("Left dungeon");
//                 break;
//             }
//         }
//     }
// }
//
//
// fn run_app_loop(player: Player) {
//     let mut app = App::new(player);
//
//     loop {
//         app.spawn_next_enemy();
//
//         if app.should_quit {
//             break;
//         }
//
//         if let Some(enemy) = &app.enemy {
//             println!("Enemy: {} Enemy \n HP: {}", enemy.name, enemy.health);
//         }
//
//         println!("Player Hp {}", app.player.health);
//
//         let command = read_input("a attack / e heal / r run / q quit");
//
//         let action = match command.as_str() {
//             "a" => PlayerAction::Attack,
//             "e" => PlayerAction::Heal,
//             "r" => PlayerAction::Run,
//             "q" => PlayerAction::Quit,
//             _ => {
//                 println!("unknown command");
//                 continue;
//             }
//         };
//
//         app.handle_battle_action(action);
//
//         for message in &app.message {
//             println!("{}", message);
//         }
//
//         app.message.clear();
//     }
// }
//
//
// fn player_is_dead(player: &Player) -> bool {
//     player.health <= 0
// }
//
// fn print_game_over(player: &Player) {
//     println!("Game Over");
//     println!("Gained Gold: {}", player.gold);
//     println!("Gained Xp: {}", player.xp);
// }
//
// fn should_continue_playing() -> bool {
//     let command = read_input("continue playing y / n");
//
//     match command.as_str() {
//         "y" => true,
//         "n" => {
//             println!("Exiting From Dungeon");
//             false
//         }
//         _ => {
//             println!("Invalid command");
//             false
//         }
//     }
// }


