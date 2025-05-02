/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   player.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/02 08:33:16 by tde-los-          #+#    #+#             */
/*   Updated: 2025/05/02 13:44:00 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::inventory::Inventory;
use crate::object::Objet;

/*
	* Player._id: ID_TEAM + _ + ID_PLAYER
	* 3_7 le joueur est dans la team 3 et c'est le numéro 7
*/

#[derive(Debug, Clone, PartialEq)]
pub struct	Player
{
	pub	_id: String,
	pub _pos_x: i32,
	pub _pos_y: i32,
	pub _dead: bool,
	pub _inventory: Inventory,
}

impl	Player
{
    pub fn	new(id: String) -> Self
	{
        Self
		{
			_id: id,
			_dead: false,
			_pos_x: 15,
			_pos_y: 16,
			_inventory: Inventory::new(),
        }
    }

	pub fn	take_object(&mut self, obj: Objet, amount: u32)
	{
		self._inventory.add(obj, amount);
	}

	pub fn	drop_object(&mut self, obj: Objet, amount: u32) -> bool
	{
		return self._inventory.remove(obj, amount);
	}
}
