/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   game.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/07 13:10:07 by tde-los-          #+#    #+#             */
/*   Updated: 2025/05/07 13:23:05 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

/*
	TODO faire game init [x]
	TODO faire game loop [ ]
	TODO faire en sorte que les IA jouent seuls [ ]
	TODO les clients doivent pouvoir jouer une IA [ ]
*/

use crate::{server, map, entities};

use colored::Colorize;
use server::ServerConfig;
use entities::team::{Team, create_team};
use entities::object::Objet;
use entities::player::Player;

fn	exemple(teams: &mut Vec<Team>, map: &mut Vec<Vec<map::Cell>>)
{
	println!("\n{}\n", "-------Exemple------".on_bright_purple());
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
	println!("\n{}\n", "-------Exemple------".on_bright_purple());
}

pub fn	game_init(config: ServerConfig)
{
	let mut	map: Vec<Vec<map::Cell>>;
	let mut	teams: Vec<Team>;

	map = map::create_map(config.width, config.height);
	teams = create_team(config.teams.clone());
	exemple(&mut teams, &mut map);
}
