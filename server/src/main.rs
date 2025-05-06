/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/04/29 15:23:47 by tde-los-          #+#    #+#             */
/*   Updated: 2025/05/06 18:56:33 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod server;
mod env;
mod map;
mod entities;

use	server::{init_server, ServerConfig};
use entities::team::{Team, create_team};
use entities::object::Objet;
use entities::player::Player;

fn	exemple(teams: &mut Vec<Team>, map: &mut Vec<Vec<map::Cell>>)
{
	println!("\n\nExemple\n");
	let player: &mut Player = &mut teams[0].get_players_mut()[0];

	player.take_object(Objet::_Food, 3);
	player.eat();
	player.eat();
	player.take_object(Objet::_Linemate, 1);
	player.take_object(Objet::_Sibur, 1);
	player.take_object(Objet::_Phiras, 1);
	player.drop_object(Objet::_Food, 1);
	player.eat();

	println!("\n\n{:?}", teams.get(0));
	println!("{:?}", map[0][0]);
}

fn	main()
{
	let	config: ServerConfig = env::init_env();
	let mut	_map: Vec<Vec<map::Cell>>;
	let mut	_teams: Vec<Team>;

	_map = map::create_map(config.width, config.height);
	_teams = create_team(config.teams.clone());
	init_server(config.clone());
	// exemple(&mut _teams, &mut _map);
}

