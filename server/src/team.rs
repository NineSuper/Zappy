/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   team.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/02 08:31:03 by tde-los-          #+#    #+#             */
/*   Updated: 2025/05/02 14:20:05 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use colored::*;
use crate::player::Player;

#[derive(Debug, Clone)]
pub struct Team
{
	_id: u32,
	_name: String,
	_level: u16,
	_next_player_id: u32,
	_connect_nbr: u32,
	pub _players: Vec<Player>,
}

impl	Team {
    pub fn new(team_name: &str, id: u32) -> Self
	{
		let mut team = Self {
			_id: id,
			_name: team_name.to_string(),
			_level: 1,
			_players: Vec::new(),
			_next_player_id: 1,
			_connect_nbr: 1, // TODO à modifier je met par défaut à 1 le temps de trouver une solution
		};

		team.add_player();
		return team;
	}

	pub fn add_player(&mut self)
	{
		let str = format!("{}_{}", self._id, self._next_player_id);
		let player: Player = Player::new(str);

		self._next_player_id += 1;
		self._players.push(player);
	}

    pub fn	remove_player(&mut self, player: &Player)
	{
		if let Some(pos) = self._players.iter().position(|p: &Player| p == player)
		{
			self._players.remove(pos);
		}
    }

	pub fn	get_level(&mut self) -> u16
	{
		return self._level;
	}

	pub fn	add_level(&mut self)
	{
		if self._level < 6
		{
			self._level += 1;
		}
	}

	pub fn	get_players(&self) -> &Vec<Player>
	{
		return &self._players;
	}

	pub fn	get_players_mut(&mut self) -> &mut Vec<Player>
	{
		return &mut self._players;
	}

	pub fn	add_connect_nbr(&mut self)
	{
		self._connect_nbr += 1;
	}

	pub fn	get_connect_nbr(&self) -> u32
	{
		return self._connect_nbr;
	}
}

pub fn	create_team(teams: Vec<String>) -> Vec<Team>
{
	let mut	all_team: Vec<Team> = vec![];
	let mut i = 1;

	for team_name in teams
	{
		let new_team: Team = Team::new(&team_name.to_string(), i);

		all_team.push(new_team);
		println!("{}", format!("Team[{}]: {} ", i, team_name).bold().yellow());
		i += 1;
	}
	return all_team;
}
