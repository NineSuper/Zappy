/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   team.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/02 08:31:03 by tde-los-          #+#    #+#             */
/*   Updated: 2025/05/13 16:46:48 by tde-los-         ###   ########.fr       */
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

		team.add_player(); // On commence avec un jour dans la team
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

	pub fn	add_level(&mut self)
	{
		self.level += 1;
	}

	pub fn	add_connect_nbr(&mut self)
	{
		self.connect_nbr += 1;
	}

	pub fn	get_level(&mut self) -> u16 { self.level }
	pub fn	get_players(&self) -> &Vec<Player> { &self.players }
	pub fn	get_players_mut(&mut self) -> &mut Vec<Player> { &mut self.players }
	pub fn	get_connect_nbr(&self) -> u32 { self.connect_nbr }
}

pub fn	create_team(teams: Vec<String>) -> Vec<Team>
{
	let mut	all_team: Vec<Team> = vec![];
	let mut i: u32 = 1;

	println!("{}", "[INFO] Création des équipes...".bold().green());
	for team_name in teams
	{
		let new_team: Team = Team::new(&team_name.to_string(), i);

		all_team.push(new_team);
		println!("{} #{i}: {}", format!("[Team]").magenta().bold(), team_name.color(get_random_color()).bold());
		i += 1;
	}
	println!("{}", "[INFO] Les équipes ont été créées !\n".bold().green());
	return all_team;
}
