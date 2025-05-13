/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   map.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/01 21:54:08 by tde-los-          #+#    #+#             */
/*   Updated: 2025/05/13 15:52:46 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use rand::prelude::*;
use rand::rngs::ThreadRng;
use rand::rng;

use colored::*;
use super::object::Objet;

#[derive(Debug, Clone)]
pub struct	Cell
{
	pub content: Option<Objet>,
}

pub type	Map = Vec<Vec<Cell>>;

pub fn	create_map(width: u32, height: u32) -> Map
{
	println!("{}", "[INFO] Création du monde...".bold().green());
	let mut rng: ThreadRng = rng();
	let mut map: Vec<Vec<Cell>> = vec![vec![Cell { content: None }; width as usize]; height as usize];

	for row in map.iter_mut()
	{
		for cell in row.iter_mut()
		{
			if rng.random_bool(0.30) // 30% de chance d'avoir un objet sur une cellule
			{
				let res: Objet = match rng.random_range(0..7)
				{
					0 => Objet::Thystame,
					1 => Objet::Linemate,
					2 => Objet::Deraumere,
					3 => Objet::Sibur,
					4 => Objet::Mendiane,
					5 => Objet::Phiras,
					_ => Objet::Food,
				};
				cell.content = Some(res);
			}
		}
	}
	println!("{}", "[INFO] Monde généré !\n".bold().green());
	return map;
}

