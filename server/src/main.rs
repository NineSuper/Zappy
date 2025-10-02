/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/04/29 15:23:47 by tde-los-          #+#    #+#             */
/*   Updated: 2025/07/01 11:49:15 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod app;
mod client;
mod config;
mod game;
mod gfx;
mod gui;
mod server;
mod utils;

use crate::gfx::graphics_broadcaster::GraphicsBroadcaster;
use app::AppState;
use game::core::state::{game_init, game_loop};
use gui::display::{display_cleanup, display_init};
use server::{init_server, ServerSettings, ServerState};

use std::collections::HashMap;

fn main()
{
	let mut config: ServerSettings = config::env::init_env();

	let mut app_state: AppState = AppState {
		game: game_init(&mut config),
		server: ServerState {
			clients: HashMap::new(),
			listener: init_server(&config),
			next_id: 0,
		},
		settings: config.clone(),
		gfx: GraphicsBroadcaster::new("0.0.0.0".to_string(), 4242),
	};
	if !config.display
	{
		display_init();
	}
	game_loop(&mut app_state);
	if !config.display
	{
		display_cleanup();
	}
}
