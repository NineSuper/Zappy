/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   player.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/02 08:33:16 by tde-los-          #+#    #+#             */
/*   Updated: 2025/05/05 13:45:10 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::inventory::Inventory;
use super::object::Objet;

/*
	* Player._id: ID_TEAM + _ + ID_PLAYER
*/

#[derive(Debug, Clone, PartialEq)]
pub struct	Player
{
	_id: String,
	_pos_x: i64,
	_pos_y: i64,
	_dead: bool,
	_inventory: Inventory,
}

impl	Player
{
    pub fn	new(id: String) -> Self
	{
        Self
		{
			_id: id,
			_pos_x: 15, // TODO
			_pos_y: 16, // TODO
			_dead: false,
			_inventory: Inventory::new(),
        }
    }

	pub fn	take_object(&mut self, obj: Objet, amount: u32) -> bool
	{
		// TODO gérer le cas où il ne peut pas prendre l'objet
		self._inventory.add(obj, amount);
		return true;
	}

	pub fn	drop_object(&mut self, obj: Objet, amount: u32) -> bool
	{
		// TODO gérer le cas où il ne peut pas drop l'objet
		// + Mettre au sol l'objet laché
		return self._inventory.remove(obj, amount);
	}

	pub fn	get_id(&self) -> String
	{
		return self._id.clone();
	}

	pub fn	is_dead(&self) -> bool
	{
		return self._dead;
	}

	// Retourne un tuple de coordonées (x, y)
	pub fn	get_position(&self) -> (i64, i64)
	{
		return (self._pos_x, self._pos_y);
	}

	// Le nom de la fonction est vraiment pas fou..
	pub fn	eat(&mut self) -> bool
	{
		if self._inventory.get(Objet::_Food) > 0
		{
			// TODO Ajouter du temps de vie
			self._inventory.remove(Objet::_Food, 1);
			println!("[DEBUG] Joueur: {} vient de manger !", self._id);
			return true;
		}
		println!("[DEBUG] Joueur: {} ne peut pas manger !", self._id);
		return false;
	}
}
