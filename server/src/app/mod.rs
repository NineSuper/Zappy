/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   mod.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/09 11:26:44 by tde-los-          #+#    #+#             */
/*   Updated: 2025/05/13 12:27:18 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::game::core::state::GameState;
use crate::server::ServerState;
use crate::server::ServerSettings;

#[derive(Debug)]
pub struct AppState
{
	pub	game: GameState,
	pub	server: ServerState,
	pub	settings: ServerSettings,
}
