/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/04/29 15:23:47 by tde-los-          #+#    #+#             */
/*   Updated: 2025/05/09 12:08:55 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod app;
mod server;
mod game;
mod env;
mod clients;
mod utils;

use app::AppState;
use game::core::state::{game_init, game_loop};
use	server::{init_server, ServerSettings, ServerState};

use std::collections::HashMap;

fn	main()
{
	let mut	config: ServerSettings = env::init_env();

	let mut app_state: AppState = AppState
	{
		game: game_init(&mut config),
		server: ServerState
		{
			clients: HashMap::new(),
			listener: init_server(&config),
			next_id: 0,
		},
		config: config,
	};
	game_loop(&mut app_state);
}

