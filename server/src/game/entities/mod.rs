/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   mod.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/07 23:51:57 by tde-los-          #+#    #+#             */
/*   Updated: 2025/06/04 12:40:28 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub mod inventory;
pub mod object;
pub mod player;
pub mod team;

use crate::game::entities::{inventory::Inventory, object::Objet, player::Player, team::Team};

pub enum Entities
{
	Team(Team),
	Player(Player),
	Inventory(Inventory),
	Object(Objet),
}
