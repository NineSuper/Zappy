/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   player.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/02 08:33:16 by tde-los-          #+#    #+#             */
/*   Updated: 2025/05/21 15:57:06 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::game::core::state::GameState;
use super::inventory::Inventory;
use crate::game::world::{map, object::Objet};

use colored::*;

/*
	* Player._id: ID_TEAM + _ + ID_PLAYER
*/

#[derive(Debug, Clone, PartialEq)]
pub enum Direction
{
	North,
	South,
	East,
	West,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Player
{
	pub id: String,
	pos_x: i32,
	pos_y: i32,
	pub inventory: Inventory,
	life_unit: i32,
	level: u32,
	direction: Direction,
	health_points: i32,
	pub client_id: Option<i32>,
}

impl	Player
{
    pub fn	new(id: String) -> Self
	{
        Self
		{
			id: id,
			pos_x: 1, // TODO
			pos_y: 1, // TODO
			inventory: Inventory::new(),
			life_unit: 10,
			level: 1,
			direction: Direction::North,
			health_points: 100,
			client_id: None,
        }
    }

	pub fn	take_object(&mut self, map: &mut Vec<Vec<map::Cell>>, obj: Objet) -> bool
	{
		if map::take_object(map, self.pos_x, self.pos_y, obj.clone())
		{
			// DEBUG
			println!("{} {}", format!("[DEBUG] Client #{} a récupéré:", self.id).cyan().italic(), obj.name());

			self.inventory.add(obj, 1);
			return true;
		}
		return false;
	}

	pub fn	drop_object(&mut self, map: &mut Vec<Vec<map::Cell>>, obj: Objet) -> bool
	{
		if map::drop_object(map, self.pos_x, self.pos_y, obj.clone())
		{
			// DEBUG
			println!("{} {}", format!("[DEBUG] Client #{} a laché:", self.id).cyan().italic(), obj.name());

			return self.inventory.remove(obj, 1);
		}
		return false;
	}

	/*
		TODO Ajouter du temps de vie
		One 'nourriture' unit allows him to survive 126 time units,
		therefore 126/t seconds.
	*/
	pub fn	eat(&mut self) -> bool
	{
		if self.inventory.get(Objet::Food) > 0
		{
			// DEBUG
			println!("{}", format!("[DEBUG] Joueur: {} vient de manger !", self.id).cyan().italic());

			return self.inventory.remove(Objet::Food, 1);
		}
		// DEBUG
		println!("{}", format!("[DEBUG] Joueur: {} n'a pas de nourriture dans son inventaire !", self.id).cyan().italic());
		return false;
	}

	// Retourne un tuple de coordonées (x, y)
	pub fn	get_position(&self) -> (i32, i32)
	{
		return (self.pos_x, self.pos_y);
	}

	pub fn move_forward(&mut self)
	{
		match self.direction
		{
			Direction::North => self.pos_y -= 1,
			Direction::South => self.pos_y += 1,
			Direction::East => self.pos_x += 1,
			Direction::West => self.pos_x -= 1,
		}
	}

	pub fn turn_right(&mut self)
	{
		self.direction = match self.direction
		{
			Direction::North => Direction::East,
			Direction::East => Direction::South,
			Direction::South => Direction::West,
			Direction::West => Direction::North,
		}
	}

	pub fn turn_left(&mut self)
	{
		self.direction = match self.direction
		{
			Direction::North => Direction::West,
			Direction::West => Direction::South,
			Direction::South => Direction::East,
			Direction::East => Direction::North,
		}
	}

	pub fn get_direction(&self) -> &Direction
	{
		&self.direction
	}

	pub fn get_health_points(&self) -> i32
	{
		self.health_points
	}

	pub fn get_inventory(&self) -> String
	{
        let mut response = String::from("{");
        let inventory = self.inventory.get_all_objects();

        for (i, (obj, count)) in inventory.iter().enumerate()
		{
            if i > 0 {
                response.push_str(", ");
            }
            response.push_str(&format!("{} {}", obj.name().to_lowercase(), count));
        }
        response.push_str("}\n");
        response
    }
}

pub fn	get_player_by_client_id(game_state: &mut GameState, client_id: i32) -> Option<&mut Player>
{
    for team in &mut game_state.teams
	{
        for player in &mut team.players
		{
            if player.client_id == Some(client_id) {
                return Some(player);
            }
        }
    }
    None
}
