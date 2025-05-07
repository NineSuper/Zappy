/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   map.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/01 21:54:08 by tde-los-          #+#    #+#             */
/*   Updated: 2025/05/02 10:15:37 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use rand::prelude::*;
use rand::rngs::ThreadRng;
use rand::rng;

use colored::*;

#[derive(Debug, Clone)]
pub enum	Resource
{
    Food,
    Linemate,
    Deraumere,
    Sibur,
    Mendiane,
    Phiras,
    Thystame,
}

#[derive(Debug, Clone)]
pub struct	Cell
{
    pub content: Option<Resource>,
}

pub type	Map = Vec<Vec<Cell>>;

pub fn	create_map(width: u32, height: u32) -> Map
{
	println!("{}", "Création du monde...".bold());
    let mut rng: ThreadRng = rng();
    let mut map: Vec<Vec<Cell>> = vec![vec![Cell { content: None }; width as usize]; height as usize];

    for row in map.iter_mut()
	{
        for cell in row.iter_mut()
		{
            if rng.random_bool(0.30)
			{
                let res: Resource = match rng.random_range(0..7)
				{
                    0 => Resource::Food,
                    1 => Resource::Linemate,
                    2 => Resource::Deraumere,
                    3 => Resource::Sibur,
                    4 => Resource::Mendiane,
                    5 => Resource::Phiras,
                    _ => Resource::Thystame,
                };
                cell.content = Some(res);
            }
        }
    }
	println!("{}", "Monde généré !\n".bold());
    return map;
}

