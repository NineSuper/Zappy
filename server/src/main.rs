/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/04/29 15:23:47 by tde-los-          #+#    #+#             */
/*   Updated: 2025/05/01 23:07:36 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod init_env;
mod server_state;
mod map;

use server_state::ServerConfig;

fn	main()
{
	let config: ServerConfig = init_env::init_env();
	let map: Vec<Vec<map::Cell>> = map::create_map(config.width, config.height);

    println!("{:?}", map[0][0]);
}
