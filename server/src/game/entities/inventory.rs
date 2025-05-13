/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   inventory.rs                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/02 08:41:42 by tde-los-          #+#    #+#             */
/*   Updated: 2025/05/13 10:46:00 by tde-los-         ###   ########.fr       */
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
}


