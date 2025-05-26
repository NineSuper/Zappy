/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   display.rs                                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/26 18:44:26 by tde-los-          #+#    #+#             */
/*   Updated: 2025/05/26 18:48:24 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::{
	io,
	sync::Mutex,
	time::Instant,
};

use crossterm::{
	event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
	execute,
	terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use lazy_static::lazy_static;

use ratatui::{
	backend::CrosstermBackend,
	layout::{Constraint, Direction, Layout, Alignment},
	style::{Color, Style, Modifier},
	text::{Span, Line, Text},
	widgets::{Block, Borders, Paragraph, Wrap},
	Terminal, Frame,
};

use crate::AppState;

lazy_static! {
	static ref LOGS: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

pub struct Display {
	terminal: Terminal<CrosstermBackend<io::Stdout>>,
	start_time: Instant,
	scroll_offset: usize,
}

static mut DISPLAY: Option<Display> = None;

pub fn	display_init()
{
	enable_raw_mode().expect("enable_raw_mode failed");

	let mut stdout = io::stdout();
	execute!(stdout, EnterAlternateScreen, EnableMouseCapture).expect("EnterAlternateScreen failed");
	let backend = CrosstermBackend::new(stdout);
	let terminal = Terminal::new(backend).expect("Failed to init terminal");

	let display = Display {
		terminal,
		start_time: Instant::now(),
		scroll_offset: 0,
	};

	unsafe {
		DISPLAY = Some(display);
	}
}

pub fn  display_cleanup()
{
	unsafe {
		if let Some(mut display) = DISPLAY.take()
		{
			disable_raw_mode().expect("disable_raw_mode failed");
			execute!(
				display.terminal.backend_mut(),
				LeaveAlternateScreen,
				DisableMouseCapture
			)
			.expect("LeaveAlternateScreen failed");
			display.terminal.show_cursor().expect("show_cursor failed");
		}
	}
}

#[macro_export]
macro_rules! game_log {
	($msg:expr) => {{
		$crate::gui::display::add_log($msg.to_string());
	}};
	($fmt:expr, $($arg:tt)*) => {{
		$crate::gui::display::add_log(format!($fmt, $($arg)*));
	}};
}

pub fn  add_log(msg: String)
{
	let mut logs = LOGS.lock().unwrap();
	logs.push(msg);

	let len = logs.len();
	if len > 1000 {
		logs.drain(0..(len - 1000));
	}

	// Reset scroll quand un nouveau log arrive (optionnel)
	unsafe {
		if let Some(display) = &mut DISPLAY {
			// Garde le scroll en bas pour les nouveaux messages
			if display.scroll_offset == 0 {
				// L'utilisateur était déjà en bas, on y reste
			}
		}
	}
}

// Fonction pour gérer les événements clavier
// Retourne true si l'utilisateur veut quitter (Q), false sinon
pub fn  handle_input() -> bool
{
	if event::poll(std::time::Duration::from_millis(50)).unwrap_or(false)
	{
		if let Ok(Event::Key(key)) = event::read()
		{
			match key.code {
				KeyCode::Char('q') | KeyCode::Char('Q') => {
					return true;
				}
				KeyCode::Up => {
					unsafe {
						if let Some(display) = &mut DISPLAY {
							let logs = LOGS.lock().unwrap();
							if display.scroll_offset < logs.len().saturating_sub(1) {
								display.scroll_offset += 1;
							}
						}
					}
				}
				KeyCode::Down => {
					unsafe {
						if let Some(display) = &mut DISPLAY {
							if display.scroll_offset > 0 {
								display.scroll_offset -= 1;
							}
						}
					}
				}
				KeyCode::PageUp => {
					unsafe {
						if let Some(display) = &mut DISPLAY {
							let logs = LOGS.lock().unwrap();
							display.scroll_offset = (display.scroll_offset + 10).min(logs.len().saturating_sub(1));
						}
					}
				}
				KeyCode::PageDown => {
					unsafe {
						if let Some(display) = &mut DISPLAY {
							display.scroll_offset = display.scroll_offset.saturating_sub(10);
						}
					}
				}
				KeyCode::Home => {
					unsafe {
						if let Some(display) = &mut DISPLAY {
							let logs = LOGS.lock().unwrap();
							display.scroll_offset = logs.len().saturating_sub(1);
						}
					}
				}
				KeyCode::End => {
					unsafe {
						if let Some(display) = &mut DISPLAY {
							display.scroll_offset = 0;
						}
					}
				}
				_ => {}
			}
		}
	}
	false
}

// Structure pour représenter un segment de texte avec son style
#[derive(Debug, Clone)]
struct StyledSegment {
	text: String,
	color: Option<Color>,
	modifiers: Modifier,
}

// Parse les codes ANSI et convertit en segments stylés
fn	parse_ansi_to_segments(text: &str) -> Vec<StyledSegment>
{
	let mut segments = Vec::new();
	let mut current_text = String::new();
	let mut current_color = None;
	let mut current_modifiers = Modifier::empty();

	let mut chars = text.chars().peekable();

	while let Some(ch) = chars.next() {
		if ch == '\x1b' && chars.peek() == Some(&'[')
		{
			// Début d'une séquence ANSI
			chars.next(); // consume '['

			// Sauvegarder le texte précédent s'il existe
			if !current_text.is_empty()
			{
				segments.push(StyledSegment {
					text: current_text.clone(),
					color: current_color,
					modifiers: current_modifiers,
				});
				current_text.clear();
			}

			let mut ansi_code = String::new();
			while let Some(&next_ch) = chars.peek()
			{
				if next_ch.is_ascii_alphabetic() {
					chars.next(); // consume la lettre finale
					break;
				}
				ansi_code.push(chars.next().unwrap());
			}

			// Parser le code ANSI
			let codes: Vec<u8> = ansi_code
				.split(';')
				.filter_map(|s| s.parse().ok())
				.collect();

			for code in codes
			{
				match code {
					0 => {
						// Reset
						current_color = None;
						current_modifiers = Modifier::empty();
					}
					1 => current_modifiers |= Modifier::BOLD,
					3 => current_modifiers |= Modifier::ITALIC,
					4 => current_modifiers |= Modifier::UNDERLINED,
					30 => current_color = Some(Color::Black),
					31 => current_color = Some(Color::Red),
					32 => current_color = Some(Color::Green),
					33 => current_color = Some(Color::Yellow),
					34 => current_color = Some(Color::Blue),
					35 => current_color = Some(Color::Magenta),
					36 => current_color = Some(Color::Cyan),
					37 => current_color = Some(Color::White),
					90 => current_color = Some(Color::DarkGray),
					91 => current_color = Some(Color::LightRed),
					92 => current_color = Some(Color::LightGreen),
					93 => current_color = Some(Color::LightYellow),
					94 => current_color = Some(Color::LightBlue),
					95 => current_color = Some(Color::LightMagenta),
					96 => current_color = Some(Color::LightCyan),
					97 => current_color = Some(Color::Gray),
					_ => {} // Ignorer les autres codes
				}
			}
		} else {
			current_text.push(ch);
		}
	}

	if !current_text.is_empty()
	{
		segments.push(StyledSegment {
			text: current_text,
			color: current_color,
			modifiers: current_modifiers,
		});
	}

	segments
}

// Convertit une ligne avec codes ANSI en Line ratatui
fn	ansi_to_ratatui_line(text: &str) -> Line
{
	let segments = parse_ansi_to_segments(text);
	let spans: Vec<Span> = segments
		.into_iter()
		.map(|segment| {
			let mut style = Style::default();
			if let Some(color) = segment.color {
				style = style.fg(color);
			}
			style = style.add_modifier(segment.modifiers);
			Span::styled(segment.text, style)
		})
		.collect();

	Line::from(spans)
}

pub fn	display_gui(app_state: &AppState)
{
	unsafe {
		if let Some(display) = &mut DISPLAY {
			let logs = LOGS.lock().unwrap();

			display
				.terminal
				.draw(|f| draw_ui(f, app_state, &display.start_time, &logs, display.scroll_offset))
				.expect("Terminal draw failed");
		}
	}
}

fn	draw_ui(f: &mut Frame, app_state: &AppState, _start_time: &Instant, logs: &[String], scroll_offset: usize)
{
	let map_width = app_state.game.map.len();
	let map_height = if map_width > 0 { app_state.game.map[0].len() } else { 0 };

	let size = f.size();

	let chunks = Layout::default()
		.direction(Direction::Vertical)
		.margin(1)
		.constraints([
			Constraint::Min(5),      // Logs
			Constraint::Length(5),   // Status
			Constraint::Length(3)    // Help
		])
		.split(size);

	let max_log_lines = chunks[0].height.saturating_sub(2) as usize;
	let total_logs = logs.len();

	let (logs_display, scroll_info) = if total_logs == 0 {
		(Vec::new(), String::new())
	}
	else {
		let available_lines = max_log_lines;
		let start_from_end = scroll_offset;
		let end_index = total_logs.saturating_sub(start_from_end);
		let start_index = end_index.saturating_sub(available_lines);

		let displayed_logs: Vec<Line> = logs[start_index..end_index]
			.iter()
			.map(|line| ansi_to_ratatui_line(line))
			.collect();

		let scroll_info = if scroll_offset > 0 {
			format!(" (↑{}/{})", scroll_offset, total_logs.saturating_sub(available_lines))
		} else {
			String::new()
		};

		(displayed_logs, scroll_info)
	};

	let logs_title = format!("Logs{}", scroll_info);
	let logs_paragraph = Paragraph::new(Text::from(logs_display))
		.block(Block::default().borders(Borders::ALL).title(logs_title));

	f.render_widget(logs_paragraph, chunks[0]);

	let info_chunks = Layout::default()
		.direction(Direction::Horizontal)
		.constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
		.split(chunks[1]);

	let status_left = Paragraph::new(Text::from(vec![
		Line::from(Span::raw(format!("Map size: {}x{}", map_width, map_height))),
		Line::from(Span::raw(format!("Clients connected: {}", app_state.server.clients.len()))),
	]))
	.block(Block::default().borders(Borders::ALL).title("Informations"))
	.style(Style::default().fg(Color::Cyan));

	f.render_widget(status_left, info_chunks[0]);

	let status_right = Paragraph::new(Text::from(vec![
		Line::from(Span::styled("Memory Usage: 55% (TODO)", Style::default().fg(Color::Green))),
		Line::from(Span::styled("CPU Load: 23% (TODO)", Style::default().fg(Color::Yellow))),
	]))
	.block(Block::default().borders(Borders::ALL).title("Système"))
	.style(Style::default().fg(Color::Magenta));

	f.render_widget(status_right, info_chunks[1]);

	let help_text = vec![
		Line::from(vec![
			Span::styled("Commandes: ", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
			Span::styled("Q", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
			Span::raw(" = Quitter  "),
			Span::styled("↑↓", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
			Span::raw(" = Scroll  "),
			Span::styled("Home/End", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
			Span::raw(" = Haut/Bas"),
		])
	];

	let help_paragraph = Paragraph::new(Text::from(help_text))
		.block(Block::default().borders(Borders::ALL).title("Help"))
		.style(Style::default().fg(Color::Gray))
		.alignment(Alignment::Center)
		.wrap(Wrap { trim: true });

	f.render_widget(help_paragraph, chunks[2]);
}
