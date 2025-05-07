/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   state.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/07 13:10:07 by tde-los-          #+#    #+#             */
/*   Updated: 2025/05/08 00:37:32 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

/*
	TODO faire game init [x]
	TODO faire une structure game [ ]
	TODO faire game loop [ ]
	TODO faire en sorte que les IA jouent seuls [ ]
	TODO les clients doivent pouvoir jouer une IA [ ]
*/

use crate::game::entities::team::Team;
use crate::game::entities::team::create_team;
use crate::game::entities::player::Player;
use crate::game::world::map;
use crate::game::world::map::create_map;
use crate::game::world::object::Objet;
use crate::server::{server_loop, ServerSettings};

use colored::*;

fn	exemple(teams: &mut Vec<Team>, map: &mut Vec<Vec<map::Cell>>)
{
	println!("\n{}\n", "-------Exemple------".on_bright_purple());
	let player: &mut Player = &mut teams[0].get_players_mut()[0];

	player.take_object(Objet::Food, 3);
	player.eat();
	player.eat();
	player.take_object(Objet::Linemate, 1);
	player.take_object(Objet::Sibur, 1);
	player.take_object(Objet::Phiras, 1);
	player.drop_object(Objet::Food, 1);
	player.eat();

	println!("\n\n{:?}", teams.get(0));
	println!("{:?}", map[0][0]);
	println!("\n{}\n", "-------Exemple------".on_bright_purple());
}

// TODO
pub fn	game_loop()
{
	loop
	{
		server_loop();
	}
}

pub fn	game_init(config: &mut ServerSettings)
{
	let mut	map: Vec<Vec<map::Cell>>;
	let mut	teams: Vec<Team>;

	map = create_map(config.width, config.height);
	teams = create_team(config.teams_name.clone());
	exemple(&mut teams, &mut map);
}
