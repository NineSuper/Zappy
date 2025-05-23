/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   inventory.rs                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/02 08:41:42 by tde-los-          #+#    #+#             */
/*   Updated: 2025/05/23 12:39:33 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::collections::HashMap;
use crate::game::world::object::Objet;

#[derive(Debug, Clone, PartialEq)]
pub struct	Inventory
{
	objects: HashMap<Objet, u32>,
}

impl	Inventory
{
	pub fn	new() -> Self
	{
		Self
		{
			objects: HashMap::new(),
		}
	}

	pub fn	add(&mut self, obj: Objet, amount: u32)
	{
		let entry = self.objects.entry(obj).or_insert(0);
		*entry += amount;
	}

	pub fn	remove(&mut self, obj: Objet, amount: u32) -> bool
	{
		if let Some(entry) = self.objects.get_mut(&obj)
		{
			if *entry >= amount
			{
				*entry -= amount;
				if *entry == 0
				{
					self.objects.remove(&obj);
				}
				return true;
			}
		}
		return false;
	}

	pub fn	get(&self, obj: Objet) -> u32
	{
		return *self.objects.get(&obj).unwrap_or(&0);
	}

	pub fn get_all_objects(&self) -> Vec<(Objet, u32)>
	{
        let mut result = Vec::new();

        result.push((Objet::food, self.get(Objet::food)));
        result.push((Objet::linemate, self.get(Objet::linemate)));
        result.push((Objet::deraumere, self.get(Objet::deraumere)));
        result.push((Objet::sibur, self.get(Objet::sibur)));
        result.push((Objet::mendiane, self.get(Objet::mendiane)));
        result.push((Objet::phiras, self.get(Objet::phiras)));
        result.push((Objet::thystame, self.get(Objet::thystame)));
        return result;
    }
}


