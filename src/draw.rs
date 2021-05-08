use crate::data::fetch::{Data, User};
use std::io;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::Color;
use tui::widgets::{Block, Borders, Paragraph, Wrap, Table, Row, TableState};
use tui::{backend::CrosstermBackend, style::Style};
use tui::{text::Text, Terminal};
use crate::util::Formatf64;

impl Formatf64 for f64 {}

pub fn draw(data: &Data) -> Result<(), io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(f.size());
        let player_profile = Layout::default()
            .direction(Direction::Horizontal)
            .margin(2)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
            .split(chunks[0]);
        let opponent_profile = Layout::default()
            .direction(Direction::Horizontal)
            .margin(2)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
            .split(chunks[1]);
        let paragraph = draw_player(&data.user);
        let player_detail_winrate = draw_winrate(&data.user);
        f.render_widget(paragraph, player_profile[0]);
        f.render_widget(player_detail_winrate, player_profile[1]);
        let opponent_paragarph = draw_player(&data.opponent);
        let opponent_detail_winrate = draw_winrate(&data.opponent);
        f.render_widget(opponent_paragarph, opponent_profile[0]);
        f.render_widget(opponent_detail_winrate, opponent_profile[1]);
    })?;

    Ok(())
}

fn draw_player(user: &Option<User>) -> Paragraph<'static> {
    if let Some(user) = user {
        let mut text = Text::from(format!("User: {}", user.user_id));
        for stat in &user.stats {
            text.extend(Text::raw(format!("Race: {}", stat.race)));
            text.extend(Text::raw(format!("Winrate: {:.2}", stat.winrate)));
            text.extend(Text::raw(format!("RankingPoints: {}", stat.ranking_point)));
        }
        let paragraph = Paragraph::new(text)
            .block(Block::default().title("Profile").borders(Borders::ALL))
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });
        paragraph
    } else {
        Paragraph::new(Text::from("did not get data"))
            .block(Block::default().title("Profile").borders(Borders::ALL))
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true })
    }
}

fn draw_winrate(user: &Option<User>) -> Table {
    if let Some(user) = user {
        if let Some(detail_winrate) = &user.detail_winrate {
            let table = Table::new(vec![
                Row::new(vec![detail_winrate["random"].to_string_two_bits(), detail_winrate["human"].to_string_two_bits(), detail_winrate["orc"].to_string_two_bits(), detail_winrate["undead"].to_string_two_bits(), detail_winrate["night elf"].to_string_two_bits()])
            ]).header(
                Row::new(vec!["VS Random", "VS Human", "VS Orc", "VS Undead", "VS Night Elf"]),
            )
                .block(Block::default().borders(Borders::ALL).title("Detail info"))
                .widths(&[
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                ]);
            return table;
        }
    }
    Table::new(vec![Row::new(vec!["no data"])])
}