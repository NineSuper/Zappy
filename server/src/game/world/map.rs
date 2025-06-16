/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   map.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/01 21:54:08 by tde-los-          #+#    #+#             */
/*   Updated: 2025/06/15 13:34:56 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::game::entities::object::Objet;
use crate::game_log;
use crate::gfx::graphics_broadcaster::GraphicsBroadcaster;
use colored::*;
use rand::prelude::*;
use rand::rng;
use rand::rngs::ThreadRng;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Cell
{
	pub content: HashMap<Objet, u32>,
}

pub type Map = Vec<Vec<Cell>>;

pub fn create_map(width: u32, height: u32) -> Map
{
	game_log!("{}", "[INFO] Création du monde...".bold().green());
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
			if rng.random_bool(0.50)
			// 50% de chance d'avoir un objet sur une cellule
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
	game_log!("{}", "[INFO] Monde généré !\n".bold().green());
	map
}

pub fn _get_cell(map: &Map, x: i32, y: i32) -> Option<&Cell>
{
	map.get(y as usize).and_then(|row| row.get(x as usize))
}

pub fn get_cell_mut(map: &mut Map, x: i32, y: i32) -> Option<&mut Cell>
{
	map.get_mut(y as usize).and_then(|row| row.get_mut(x as usize))
}

pub fn get_cell_update_json(x: usize, y: usize, cell: &Cell) -> String
{
	let mut objets_list = Vec::new();
	for (objet, &count) in &cell.content
	{
		for _ in 0..count
		{
			objets_list.push(format!("\"{}\"", objet.name()));
		}
	}

	let objets_json = if objets_list.is_empty()
	{
		"[]".to_string()
	}
	else
	{
		format!("[{}]", objets_list.join(","))
	};

	format!(r#"{{"type":"cell_update","x":{},"y":{},"objets":{}}}"#, x, y, objets_json)
}

pub fn drop_object(
	map: &mut Map,
	x: i32,
	y: i32,
	obj: Objet,
	gfx: Option<&mut GraphicsBroadcaster>,
) -> bool
{
	if let Some(cell) = get_cell_mut(map, x, y)
	{
		*cell.content.entry(obj).or_insert(0) += 1;

		if let Some(broadcaster) = gfx
		{
			let update_json = get_cell_update_json(x as usize, y as usize, cell);
			broadcaster.broadcast_message(&update_json);
		}

		return true;
	}
	return false;
}

pub fn take_object(
	map: &mut Map,
	x: i32,
	y: i32,
	obj: Objet,
	gfx: Option<&mut GraphicsBroadcaster>,
) -> bool
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

				if let Some(broadcaster) = gfx
				{
					let update_json = get_cell_update_json(x as usize, y as usize, cell);
					broadcaster.broadcast_message(&update_json);
				}

				return true;
			}
		}
	}
	return false;
}

pub fn spawn_object(map: &mut Map, mut gfx: Option<&mut GraphicsBroadcaster>)
{
	let mut rng: ThreadRng = rng();
	// il y'a 1% de chance par tick que des objets spawn sur la map
	if !rng.random_bool(0.01)
	{
		return;
	}
	// entre 1 & 32 objets
	let objects_to_spawn = rng.random_range(1..32);
	for _ in 0..objects_to_spawn
	{
		let x = rng.random_range(0..map[0].len());
		let y = rng.random_range(0..map.len());
		let res = match rng.random_range(0..7)
		{
			0 => Objet::Food,
			1 => Objet::Linemate,
			2 => Objet::Deraumere,
			3 => Objet::Sibur,
			4 => Objet::Mendiane,
			5 => Objet::Phiras,
			_ => Objet::Thystame,
		};

		*map[y][x].content.entry(res.clone()).or_insert(0) += 1;

		if let Some(broadcaster) = gfx.as_mut()
		{
			let update_json = get_cell_update_json(x, y, &map[y][x]);
			broadcaster.broadcast_message(&update_json);
		}

		// game_log!("{} {:?} a spawn en: {}x{}", "[DEBUG]".yellow().bold(), res, x, y);
	}
}

pub fn map_update(map: &mut Map, gfx: Option<&mut GraphicsBroadcaster>)
{
	spawn_object(map, gfx);
}

pub fn get_map_json(map: &Map) -> String
{
	let mut cellules_json = Vec::new();
	for (y, row) in map.iter().enumerate()
	{
		for (x, cell) in row.iter().enumerate()
		{
			if !cell.content.is_empty()
			{
				let mut objets_list = Vec::new();
				for (objet, &count) in &cell.content
				{
					for _ in 0..count
					{
						objets_list.push(format!("\"{}\"", objet.name()));
					}
				}
				let objets_json = format!("[{}]", objets_list.join(", "));
				let cellule_json =
					format!(r#"{{"x": {}, "y": {}, "objets": {}}}"#, x, y, objets_json);
				cellules_json.push(cellule_json);
			}
		}
	}
	let json = format!(
		r#"{{
			"type": "map",
			"largeur": {},
			"hauteur": {},
			"cellules": [{}]
			}}"#,
		map[0].len(),
		map.len(),
		cellules_json.join(", ")
	)
	.replace(['\t', '\n', ' '], "");
	return json;
}
