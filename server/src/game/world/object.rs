/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   object.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/02 08:46:32 by tde-los-          #+#    #+#             */
/*   Updated: 2025/05/14 14:04:22 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum	Objet
{
	Food,
	Linemate,
	Deraumere,
	Sibur,
	Mendiane,
	Phiras,
	Thystame,
}

impl	Objet
{
	pub fn	name(&self) -> &'static str
	{
		match self
		{
			Objet::Food => "Nourriture",
			Objet::Linemate => "Linemate",
			Objet::Deraumere => "Deraumere",
			Objet::Sibur => "Sibur",
			Objet::Mendiane => "Mendiane",
			Objet::Phiras => "Phiras",
			Objet::Thystame => "Thystame",
		}
	}
}
