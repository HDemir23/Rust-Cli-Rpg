use crate::game_state::App;

use ratatui::widgets::{Block, Borders, Gauge, List, ListItem, Paragraph};
use ratatui::{layout::{Constraint, Direction, Layout}, style::{Color, Style}, Frame};

pub fn draw(frame: &mut Frame, app: &App) {

    // i guess our container??
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(8),
            Constraint::Length(8),
        ])
        .split(frame.area());

    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[0]);

    let player_text = format!(
        "Name: {} - Level:{} - Gold: {} - XP: {}/{}",
        app.player.name,
        app.player.level,
        app.player.gold,
        app.player.xp,
        app.player.xp_to_next_level
    );


    frame.render_widget(
        Paragraph::new(player_text).block(Block::default().title("Player").borders(Borders::ALL)),
        top_chunks[0],
    );

    let hp_ratio = if app.player.max_health <= 0 {
        0.0
    } else {
        (app.player.health as f64 / app.player.max_health as f64).clamp(0.0, 1.0)
    };

    frame.render_widget(
        Gauge::default()
            .block(Block::default().title("Hp").borders(Borders::ALL))
            .gauge_style(Style::default().fg(Color::Red))
            .ratio(hp_ratio)
            .label(format!("{}/{}", app.player.health, app.player.max_health)),
        top_chunks[1],
    );

    let middle_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(chunks[1]);

    let enemy_text = match &app.enemy {
        Some(enemy) => format!(
            "{}\nLevel: {}\nRarity: {:?}\nHP: {}\nDamage: {}",
            enemy.name, enemy.level, enemy.rarity, enemy.health, enemy.damage
        ),
        None => "No enemy".to_string(),
    };

    frame.render_widget(
        Paragraph::new(enemy_text).block(Block::default().title("Enemy").borders(Borders::ALL)),
        middle_chunks[0],
    );

    frame.render_widget(
        Paragraph::new("[a] Attack \n[e] Heal\n[r] Run\n[q] Quit")
            .block(Block::default().title("Actions").borders(Borders::ALL)),
        middle_chunks[1],
    );

    let messages = app.message.iter()
        .rev()
        .take(6)
        .map(|message| ListItem::new(message.as_str()))
        .collect::<Vec<ListItem>>();

    frame.render_widget(
        List::new(messages).block(Block::default().title("Logs").borders(Borders::ALL)),
        chunks[2],
    );
}
