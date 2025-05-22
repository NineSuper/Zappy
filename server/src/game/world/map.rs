/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   map.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/01 21:54:08 by tde-los-          #+#    #+#             */
/*   Updated: 2025/05/14 13:51:56 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::collections::HashMap;
use rand::prelude::*;
use rand::rngs::ThreadRng;
use rand::rng;
use colored::*;

use super::object::Objet;

#[derive(Debug, Clone)]
pub struct Cell
{
	pub content: HashMap<Objet, u32>,
}

pub type Map = Vec<Vec<Cell>>;

pub fn	create_map(width: u32, height: u32) -> Map
{
	println!("{}", "[INFO] Création du monde...".bold().green());

	let mut rng: ThreadRng = rng();
	let mut map: Vec<Vec<Cell>> = vec![
		vec![
			Cell {
				content: HashMap::new()
			};
			width as usize
		];
		height as usize
	];

	for row in map.iter_mut()
	{
		for cell in row.iter_mut()
		{
			if rng.random_bool(0.30) // 30% de chance d'avoir un objet sur une cellule
			{
				let res: Objet = match rng.random_range(0..7)
				{
					0 => Objet::Food,
					1 => Objet::Linemate,
					2 => Objet::Deraumere,
					3 => Objet::Sibur,
					4 => Objet::Mendiane,
					5 => Objet::Phiras,
					_ => Objet::Thystame,
				};
				*cell.content.entry(res).or_insert(0) += 1;
			}
		}
	}

	println!("{}", "[INFO] Monde généré !\n".bold().green());
	map
}

pub fn	get_cell(map: &Map, x: i32, y: i32) -> Option<&Cell>
{
	map.get(y as usize).and_then(|row| row.get(x as usize))
}

pub fn	get_cell_mut(map: &mut Map, x: i32, y: i32) -> Option<&mut Cell>
{
	map.get_mut(y as usize).and_then(|row| row.get_mut(x as usize))
}

pub fn	drop_object(map: &mut Map, x: i32, y: i32, obj: Objet) -> bool
{
	if let Some(cell) = get_cell_mut(map, x, y)
	{
		*cell.content.entry(obj).or_insert(0) += 1;
		return true;
	}
	return false;
}

pub fn	take_object(map: &mut Map, x: i32, y: i32, obj: Objet) -> bool
{
	if let Some(cell) = get_cell_mut(map, x, y)
	{
		if let Some(count) = cell.content.get_mut(&obj)
		{
			if *count > 0
			{
				*count -= 1;
				if *count == 0
				{
					cell.content.remove(&obj);
				}
				return true;
			}
		}
	}
	return false;
}
