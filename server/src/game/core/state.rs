/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   state.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/07 13:10:07 by tde-los-          #+#    #+#             */
/*   Updated: 2025/05/09 12:10:20 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

/*
	TODO faire game init [x]
	TODO faire une structure game [x]
	TODO faire game loop [x]
	TODO faire en sorte que les IA jouent seuls [ ]
	TODO les clients doivent pouvoir jouer une IA [ ]
*/

use crate::app::AppState;

use crate::game::world::map;
use crate::game::entities::team::{self, Team};
use crate::game::world::map::Cell;
use crate::server::{server_loop, ServerSettings};

#[derive(Debug, Clone)]
pub struct	GameState
{
	pub	map: Vec<Vec<Cell>>,
	pub	teams: Vec<Team>,
}

pub fn	game_loop(app_state: &mut AppState)
{
	loop
	{
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
