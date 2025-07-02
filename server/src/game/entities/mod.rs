/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   mod.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/07 23:51:57 by tde-los-          #+#    #+#             */
/*   Updated: 2025/07/01 11:35:25 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub mod inventory;
pub mod object;
pub mod player;
pub mod team;

use crate::game::entities::{object::Objet, player::Player};

pub enum Entities
{
	Player(Player),
	Object(Objet),
}
