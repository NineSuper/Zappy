/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   object.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/02 08:46:32 by tde-los-          #+#    #+#             */
/*   Updated: 2025/05/23 12:39:56 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum	Objet
{
	food,
	linemate,
	deraumere,
	sibur,
	mendiane,
	phiras,
	thystame,
}

impl	Objet
{
	pub fn	name(&self) -> &'static str
	{
		match self
		{
			Objet::food => "nourriture",
			Objet::linemate => "linemate",
			Objet::deraumere => "deraumere",
			Objet::sibur => "sibur",
			Objet::mendiane => "mendiane",
			Objet::phiras => "phiras",
			Objet::thystame => "thystame",
		}
	}

	pub fn	from_name(name: &str) -> Option<Objet>
	{
		match name.to_lowercase().as_str()
		{
			"nourriture" => Some(Objet::food),
			"linemate" => Some(Objet::linemate),
			"deraumere" => Some(Objet::deraumere),
			"sibur" => Some(Objet::sibur),
			"mendiane" => Some(Objet::mendiane),
			"phiras" => Some(Objet::phiras),
			"thystame" => Some(Objet::thystame),
			_ => None,
		}
	}
}
