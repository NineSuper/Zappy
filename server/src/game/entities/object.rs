/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   object.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/02 08:46:32 by tde-los-          #+#    #+#             */
/*   Updated: 2025/06/03 17:31:31 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Objet
{
	Food,
	Linemate,
	Deraumere,
	Sibur,
	Mendiane,
	Phiras,
	Thystame,
}

impl Objet
{
	pub fn name(&self) -> &'static str
	{
		match self
		{
			Objet::Food => "nourriture",
			Objet::Linemate => "linemate",
			Objet::Deraumere => "deraumere",
			Objet::Sibur => "sibur",
			Objet::Mendiane => "mendiane",
			Objet::Phiras => "phiras",
			Objet::Thystame => "thystame",
		}
	}

	pub fn from_name(name: &str) -> Option<Objet>
	{
		match name.to_lowercase().as_str()
		{
			"nourriture" => Some(Objet::Food),
			"linemate" => Some(Objet::Linemate),
			"deraumere" => Some(Objet::Deraumere),
			"sibur" => Some(Objet::Sibur),
			"mendiane" => Some(Objet::Mendiane),
			"phiras" => Some(Objet::Phiras),
			"thystame" => Some(Objet::Thystame),
			_ => None,
		}
	}
}
