/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/04/29 15:23:47 by tde-los-          #+#    #+#             */
/*   Updated: 2025/05/02 13:44:43 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod server_state;
mod init;
mod map;
mod team;
mod player;
mod inventory;
mod object;

use	server_state::ServerConfig;
use	team::Team;
use crate::object::Objet;

fn main()
{
	let config: ServerConfig = init::init_env();
	let mut map: Vec<Vec<map::Cell>>;
	let mut _teams: Vec<Team>;

	map = map::create_map(config.width, config.height);
	_teams = team::create_team(config.teams);

	let player: &mut player::Player = &mut _teams[0].get_players_mut()[0];

	player.take_object(Objet::_Food, 3);
	player.take_object(Objet::_Linemate, 1);
	player.take_object(Objet::_Sibur, 1);
	player.take_object(Objet::_Phiras, 1);
	player.drop_object(Objet::_Food, 3);

	println!("\n\n{:?}", _teams.get(0));
	println!("{:?}", map[0][0]);
}



// EXEMPLE
// let player_to_remove = _teams[3]._players[0].to_owned();
// _teams[3].remove_player(&player_to_remove);

