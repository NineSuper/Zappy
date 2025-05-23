/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   mod.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/09 11:26:44 by tde-los-          #+#    #+#             */
/*   Updated: 2025/05/23 13:51:23 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::game::core::state::GameState;
use crate::server::ServerState;
use crate::server::ServerSettings;
use crate::game::entities::player::Player;
use crate::game::world::object::Objet;

#[derive(Debug)]
pub struct AppState
{
	pub	game: GameState,
	pub	server: ServerState,
	pub	settings: ServerSettings,
}

impl GameState
{
	pub fn	get_player_by_client_id(&mut self, client_id: i32) -> Option<&mut Player>
	{
		for team in &mut self.teams
		{
			for player in &mut team.players
			{
				if player.client_id == Some(client_id)
				{
					return Some(player);
				}
			}
		}
		None
	}

	fn	find_player_position(&self, client_id: i32) -> Option<(usize, usize)>
	{
		for (team_idx, team) in self.teams.iter().enumerate()
		{
			for (player_idx, player) in team.players.iter().enumerate()
			{
				if player.client_id == Some(client_id) {
					return Some((team_idx, player_idx));
				}
			}
		}
		None
	}

	pub fn	move_player_forward(&mut self, client_id: i32) -> bool
	{
		let player_position = self.find_player_position(client_id);

		if let Some((team_idx, player_idx)) = player_position
		{
			let map = &self.map;
			let player = &mut self.teams[team_idx].players[player_idx];
			player.move_forward(map);
			return true;
		}
		return false;
	}

	pub fn	turn_player_right(&mut self, client_id: i32) -> bool
	{
		if let Some(player) = self.get_player_by_client_id(client_id)
		{
			player.turn_right();
			return true;
		}
		return false;
	}

	pub fn	turn_player_left(&mut self, client_id: i32) -> bool
	{
		if let Some(player) = self.get_player_by_client_id(client_id)
		{
			player.turn_left();
			return true;
		}
		return false;
	}

	pub fn	get_player_vision(&self, client_id: i32) -> String
	{
		for team in &self.teams
		{
			for player in &team.players
			{
				if player.client_id == Some(client_id)
				{
					return player.get_vision(self);
				}
			}
		}
		return "Joueur introuvable\n".to_string();
	}

	pub fn	get_player_inventory(&self, client_id: i32) -> String
	{
		for team in &self.teams
		{
			for player in &team.players
			{
				if player.client_id == Some(client_id) {
					return player.get_inventory();
				}
			}
		}
		return "Joueur introuvable\n".to_string();
	}

	pub fn	player_take_object(&mut self, client_id: i32, object_name: &str) -> bool
	{
		if let Some(obj) = Objet::from_name(object_name)
		{
			let player_position = self.find_player_position(client_id);

			if let Some((team_idx, player_idx)) = player_position
			{
				let map = &mut self.map;
				let player = &mut self.teams[team_idx].players[player_idx];

				return player.take_object(map, obj);
			}
		}
		return false;
	}

	pub fn	player_drop_object(&mut self, client_id: i32, object_name: &str) -> bool
	{
		if let Some(obj) = Objet::from_name(object_name)
		{
			let	player_position = self.find_player_position(client_id);

			if let Some((team_idx, player_idx)) = player_position
			{
				let map = &mut self.map;
				let player = &mut self.teams[team_idx].players[player_idx];

				return player.drop_object(map, obj);
			}
		}
		return false;
	}

	pub fn	player_expulse(&mut self, client_id: i32)
	{
	}

	pub fn	player_broadcast(&mut self, client_id: i32, message: &str)
	{
	}

	pub fn	get_player_level(&self, client_id: i32) -> u32
	{
		for team in &self.teams
		{
			for player in &team.players
			{
				if player.client_id == Some(client_id)
				{
					return player.level;
				}
			}
		}
		return 1;
	}
}
