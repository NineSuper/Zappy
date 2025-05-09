/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   team.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/02 08:31:03 by tde-los-          #+#    #+#             */
/*   Updated: 2025/05/09 12:18:13 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use colored::*;
use crate::utils::get_random_color;
use super::player::Player;

#[derive(Debug, Clone)]
pub struct Team
{
	id: u32,
	name: String,
	level: u16,
	next_player_id: u32,
	connect_nbr: u32,
	pub players: Vec<Player>,
}

impl	Team
{
	pub fn	new(team_name: &str, id: u32) -> Self
	{
		let mut team = Self {
			id,
			name: team_name.to_string(),
			level: 1,
			players: Vec::new(),
			next_player_id: 1,
			connect_nbr: 1, // TODO
		};

		team.add_player();
		return team;
	}

	pub fn	add_player(&mut self)
	{
		let str = format!("{}_{}", self.id, self.next_player_id);
		let player: Player = Player::new(str);

		self.next_player_id += 1;
		self.players.push(player);
	}

	pub fn	remove_player(&mut self, player: &Player)
	{
		if let Some(pos) = self.players.iter().position(|p: &Player| p == player)
		{
			self.players.remove(pos);
		}
	}

	pub fn	get_level(&mut self) -> u16
	{
		return self.level;
	}

	pub fn	add_level(&mut self)
	{
		if self.level < 6
		{
			self.level += 1;
		}
	}

	pub fn	get_players(&self) -> &Vec<Player>
	{
		return &self.players;
	}

	pub fn	get_players_mut(&mut self) -> &mut Vec<Player>
	{
		return &mut self.players;
	}

	pub fn	add_connect_nbr(&mut self)
	{
		self.connect_nbr += 1;
	}

	pub fn	get_connect_nbr(&self) -> u32
	{
		return self.connect_nbr;
	}
}

pub fn	create_team(teams: Vec<String>) -> Vec<Team>
{
	let mut	all_team: Vec<Team> = vec![];
	let mut i: u32 = 1;

	for team_name in teams
	{
		let new_team: Team = Team::new(&team_name.to_string(), i);

		all_team.push(new_team);
		println!("{}", format!("Team[{i}]: {}", team_name.color(get_random_color())).yellow());
		i += 1;
	}
	print!("\n");
	return all_team;
}
