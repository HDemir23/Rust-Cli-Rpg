mod battle;
mod enemy_gen;
mod game_state;
mod player_progression;
mod read_input;
mod structs;
mod ui;
mod save;

use crate::game_state::App;
use crate::read_input::read_input;
use crate::structs::{Player, PlayerAction};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};

fn main() -> std::io::Result<()> {
    let player = match save::load_player() {
        Some(player) => player,
        None => create_player(),
    };

    run_tui_loop(player)
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

    ratatui::run(|terminal| loop {
        app.spawn_next_enemy();
        terminal.draw(|frame| ui::draw(frame, &app))?;

        if app.should_quit {
            break Ok(());
        }

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
    })
}
