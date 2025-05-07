/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/04/29 15:23:47 by tde-los-          #+#    #+#             */
/*   Updated: 2025/05/07 15:16:13 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod env;
mod clients;
mod server;
mod map;
mod game;
mod entities;

use game::{game_init, game_loop};
use	server::{init_server, ServerSettings};

/*
	TODO faire une structure "Client" que init_server doit renvoyer,
	afin de pouvoir traiter les infos client dans game_loop
*/

fn	main()
{
	let mut config: ServerSettings;

	config = env::init_env();
	game_init(&mut config);
	init_server(config.port);
	game_loop(); // envoyer config et la structure client
}

