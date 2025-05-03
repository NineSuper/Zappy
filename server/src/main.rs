/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/04/29 15:23:47 by tde-los-          #+#    #+#             */
/*   Updated: 2025/05/03 12:52:12 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod server_state;
mod init;
mod map;
mod entities;

use	server_state::ServerConfig;
use entities::team::{Team, create_team};
use entities::object::Objet;
use entities::player::Player;

fn	main()
{
	let	config: ServerConfig = init::init_env();
	let mut	_map: Vec<Vec<map::Cell>>;
	let mut	_teams: Vec<Team>;

	_map = map::create_map(config.width, config.height);
	_teams = create_team(config.teams);

	// Exemple
	println!("\n\nExemple\n");
	let player: &mut Player = &mut _teams[0].get_players_mut()[0];

	player.take_object(Objet::_Food, 3);
	player.eat();
	player.eat();
	player.take_object(Objet::_Linemate, 1);
	player.take_object(Objet::_Sibur, 1);
	player.take_object(Objet::_Phiras, 1);
	player.drop_object(Objet::_Food, 1);
	player.eat();

	println!("\n\n{:?}", _teams.get(0));
	println!("{:?}", _map[0][0]);
}

// EXEMPLE pour retirer un joueur
// let player_to_remove = _teams[3]._players[0].to_owned();
// _teams[3].remove_player(&player_to_remove);

