/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   team.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/02 08:31:03 by tde-los-          #+#    #+#             */
/*   Updated: 2025/06/11 10:59:11 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::player::Player;
use crate::{game_log, utils::utils::get_random_color};
use colored::*;

#[derive(Debug, Clone)]
pub struct Team
{
	pub id: u32,
	pub name: String,
	_level: u16,
	next_player_id: u32,
	connect_nbr: u32,
	pub players: Vec<Player>,
}

impl Team
{
	pub fn new(team_name: &str, connect_nbr: u32, id: u32) -> Self
	{
		let team = Self {
			id,
			name: team_name.to_string(),
			_level: 1,
			players: Vec::new(),
			next_player_id: 1,
			connect_nbr: connect_nbr,
		};
		return team;
	}

	pub fn add_player(&mut self)
	{
		let str = format!("{}_{}", self.id, self.next_player_id);
		let player: Player = Player::new(str);

		self.players.push(player);

		game_log!(
			"{} Joueur {} a rejoint l'équipe {}",
			"[GAME]".magenta().bold(),
			self.next_player_id,
			self.name.bold().yellow(),
		);

		self.next_player_id += 1;
	}

	pub fn _remove_player(&mut self, player: &Player)
	{
		if let Some(pos) = self.players.iter().position(|p: &Player| p == player)
		{
			self.players.remove(pos);
		}
	}

	pub fn _add_level(&mut self)
	{
		self._level += 1;
	}

	pub fn _add_connect_nbr(&mut self)
	{
		self.connect_nbr += 1;
	}

	pub fn assign_player(&mut self, client_id: i32) -> Option<String>
	{
		for player in &mut self.players
		{
			if player.client_id.is_none()
			{
				self.connect_nbr -= 1;
				player.client_id = Some(client_id);

				return Some(player.id.clone());
			}
		}
		return None;
	}

	pub fn unassign_player(&mut self, client_id: i32)
	{
		for player in &mut self.players.iter_mut()
		{
			if player.client_id == Some(client_id)
			{
				self.connect_nbr += 1;
				player.client_id = None;
			}
		}
	}

	pub fn _get_level(&mut self) -> u16
	{
		self._level
	}
	pub fn _get_players(&self) -> &Vec<Player>
	{
		&self.players
	}
	pub fn _get_players_mut(&mut self) -> &mut Vec<Player>
	{
		&mut self.players
	}
	pub fn get_connect_nbr(&self) -> u32
	{
		self.players.len() as u32 - self.connect_nbr
	}
}

pub fn add_client_team(name: String, teams: &mut Vec<Team>, client_id: i32) -> Option<String>
{
	for team in teams.iter_mut()
	{
		if team.name == name
		{
			if team.connect_nbr > 0
			{
				let id = match team.assign_player(client_id)
				{
					Some(id) => id,
					None => {
						return None;
					}
				};

				game_log!(
					"{} ({}) Client #{client_id} assigné au joueur: {}",
					"[GAME]".magenta().bold(),
					name.green().bold(),
					id,
				);

				return Some(id);
			}
			return None;
		}
	}
	return None;
}

pub fn create_team(teams: Vec<String>, connect_max: u32) -> Vec<Team>
{
	let mut all_team: Vec<Team> = vec![];
	let mut i: u32 = 1;

	game_log!("{}", "[INFO] Création des équipes...".bold().green());
	for team_name in teams
	{
		let mut new_team: Team = Team::new(&team_name.to_string(), connect_max, i);

		game_log!(
			"{} #{i}: {}",
			format!("[Team]").magenta().bold(),
			team_name.color(get_random_color()).bold()
		);

		for _ in 0..connect_max
		{
			new_team.add_player();
		}

		all_team.push(new_team);
		i += 1;
	}
	game_log!("{}", "[INFO] Les équipes ont été créées !".bold().green());
	return all_team;
}
