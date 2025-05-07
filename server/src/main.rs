/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/04/29 15:23:47 by tde-los-          #+#    #+#             */
/*   Updated: 2025/05/08 00:11:38 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod server;
mod game;
mod env;
mod clients;

use clients::Client;
use game::core::state::{game_init, game_loop};
use	server::{init_server, ServerSettings};

/*
	TODO faire une structure "Client" que init_server doit renvoyer,
	afin de pouvoir traiter les infos client dans game_loop
*/

fn	main()
{
	let mut config: ServerSettings;
	let mut clients: Vec<Client>;

	config = env::init_env();
	game_init(&mut config);
	clients = init_server(config);
	game_loop(); // envoyer config et la structure client
}

