/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   player.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/02 08:33:16 by tde-los-          #+#    #+#             */
/*   Updated: 2025/05/09 12:20:13 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::inventory::Inventory;
use crate::game::world::object::Objet;

/*
	* Player._id: ID_TEAM + _ + ID_PLAYER
*/

#[derive(Debug, Clone, PartialEq)]
pub struct	Player
{
	id: String,
	pos_x: i64,
	pos_y: i64,
	dead: bool,
	inventory: Inventory,
}

impl	Player
{
    pub fn	new(id: String) -> Self
	{
        Self
		{
			id,
			pos_x: 15, // TODO
			pos_y: 16, // TODO
			dead: false,
			inventory: Inventory::new(),
        }
    }

	pub fn	take_object(&mut self, obj: Objet, amount: u32) -> bool
	{
		// TODO gérer le cas où il ne peut pas prendre l'objet
		self.inventory.add(obj, amount);
		return true;
	}

	pub fn	drop_object(&mut self, obj: Objet, amount: u32) -> bool
	{
		// TODO gérer le cas où il ne peut pas drop l'objet
		// + Mettre au sol l'objet laché
		return self.inventory.remove(obj, amount);
	}

	pub fn	get_id(&self) -> String
	{
		return self.id.clone();
	}

	pub fn	is_dead(&self) -> bool
	{
		return self.dead;
	}

	// Retourne un tuple de coordonées (x, y)
	pub fn	get_position(&self) -> (i64, i64)
	{
		return (self.pos_x, self.pos_y);
	}

	// Le nom de la fonction est vraiment pas fou..
	pub fn	eat(&mut self) -> bool
	{
		if self.inventory.get(Objet::Food) > 0
		{
			// TODO Ajouter du temps de vie
			self.inventory.remove(Objet::Food, 1);
			println!("[DEBUG] Joueur: {} vient de manger !", self.id);
			return true;
		}
		println!("[DEBUG] Joueur: {} ne peut pas manger !", self.id);
		return false;
	}
}
