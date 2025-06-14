/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   state.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/07 13:10:07 by tde-los-          #+#    #+#             */
/*   Updated: 2025/06/11 09:15:09 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::app::AppState;
use crate::game::core::gamestate::GameState;
use crate::game::entities::team::{self};
use crate::game::world::map::{self, spawn_object};
use crate::gui::display::{display_gui, handle_input};
use crate::server::{handle_client, server_loop, ServerSettings};

use std::time::Instant;

pub fn update_game(app_state: &mut AppState)
{
	for team in app_state.game.teams.iter_mut()
	{
		team.players.retain_mut(|player| player.eat());
	}
	spawn_object(&mut app_state.game.map);
}

pub fn game_loop(app_state: &mut AppState)
{
	let tick_duration = std::time::Duration::from_secs_f64(1.0 / app_state.settings.time_unit);
	let display_tick = std::time::Duration::from_secs_f64(1.0 / 10.0); // 10/secondes
	let mut last_tick = Instant::now();
	let mut last_tick_display = Instant::now();

	loop
	{
		let now = Instant::now();

		if handle_input() { break; }
		if now.duration_since(last_tick) >= tick_duration
		{
			for client in app_state.server.clients.values_mut()
			{
				handle_client(client, &mut app_state.game);
			}
			update_game(app_state);
			last_tick = now;
		}
		if now.duration_since(last_tick_display) >= display_tick
		{
			display_gui(&app_state);
			last_tick_display = now;
		}
		server_loop(&mut app_state.server, &mut app_state.game);
	}
}

pub fn game_init(config: &mut ServerSettings) -> GameState
{
	GameState {
		map: map::create_map(config.width, config.height),
		teams: team::create_team(config.teams_name.clone(), config.connexion_max),
	}
}
