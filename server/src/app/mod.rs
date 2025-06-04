/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   mod.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/09 11:26:44 by tde-los-          #+#    #+#             */
/*   Updated: 2025/06/04 12:09:49 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::game::core::gamestate::GameState;
use crate::server::ServerSettings;
use crate::server::ServerState;

#[derive(Debug)]
pub struct AppState
{
	pub game: GameState,
	pub server: ServerState,
	pub settings: ServerSettings,
}
