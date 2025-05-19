/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   state.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/07 13:10:07 by tde-los-          #+#    #+#             */
/*   Updated: 2025/05/19 15:23:37 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

/*
	TODO faire game init [x]
	TODO faire une structure game [x]
	TODO faire game loop [x]
*/

use crate::app::AppState;
use crate::game::entities::team::{self, add_client_team, remove_client_team, Team};
use crate::game::world::map;
use crate::game::world::map::Cell;
use crate::server::{handle_client, server_loop, ServerSettings};

use std::time::Instant;

#[derive(Debug, Clone)]
pub struct	GameState
{
	pub	map: Vec<Vec<Cell>>,
	pub	teams: Vec<Team>,
}

pub fn	update_game(_app_state: &mut AppState)
{

}

pub fn	game_loop(app_state: &mut AppState)
{
	let	tick_duration = std::time::Duration::from_secs_f64(1.0 / app_state.settings.time_unit);
	let mut last_tick = Instant::now();

	loop
	{
		let now = Instant::now();

		if now.duration_since(last_tick) >= tick_duration
		{
			for (id, client) in app_state.server.clients.iter_mut() {
				handle_client(client);
			}
			update_game(app_state);
            last_tick = now;
		}
		server_loop(&mut app_state.server);
	}
}

pub fn	game_init(config: &mut ServerSettings) -> GameState
{
	GameState
	{
		map: map::create_map(config.width, config.height),
		teams: team::create_team(config.teams_name.clone()),
	}
}
