/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   server.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/01 22:12:16 by tde-los-          #+#    #+#             */
/*   Updated: 2025/05/06 19:24:35 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::net::{TcpListener, TcpStream};
use std::process::exit;
use std::collections::HashMap;
use colored::*;

#[derive(Debug, Clone)]
pub struct ServerConfig
{
    pub _port: u32,
    pub width: u32,
    pub height: u32,
    pub _connexion_max: u32,
    pub _time_unit: u32,
    pub teams: Vec<String>,
}

fn	setup_listener(addr: &String) -> TcpListener
{
    let	listener = TcpListener::bind(addr);

    match listener
    {
        Ok(listener) => {
			// listener.set_nonblocking(true);
            println!("✅ Serveur ouvert sur: {}\n", addr);
            return listener;
        }
        Err(e) => {
            eprintln!("❌ Erreur lors de l'écoute sur {}: {}", addr, e);
			exit(-1);
        }
    }
}

fn	handle_client(clients: &mut HashMap<i32, TcpStream>)
{
	for (id, stream) in clients.iter()
	{
		println!("[DEBUG] {id} {stream:?}");
	}
}

fn	accept_new_client(clients: &mut HashMap<i32, TcpStream>, listener: &TcpListener, next_id: &mut i32)
{
	if let Ok((stream, addr)) = listener.accept()
	{
		println!("{}{addr:?}", "Nouveau client: ".green());
		clients.insert(*next_id, stream);
		*next_id += 1;
	}
}

pub fn	init_server(config: ServerConfig)
{
	let	addr = format!("127.0.0.1:{}", config._port);
	let	listener = setup_listener(&addr);
	let mut	clients: HashMap<i32, TcpStream> = HashMap::new();
	let mut next_id = 0;

	loop
	{
		accept_new_client(&mut clients, &listener, &mut next_id);
		handle_client(&mut clients);
	}
}
