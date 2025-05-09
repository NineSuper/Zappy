/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   server.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/01 22:12:16 by tde-los-          #+#    #+#             */
/*   Updated: 2025/05/09 12:32:34 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::io::Read;
use std::net::TcpListener;
use std::process::exit;
use std::collections::HashMap;
use colored::*;

use crate::clients::Client;

#[derive(Debug, Clone)]
pub struct	ServerSettings
{
	pub port: u32,
	pub width: u32,
	pub height: u32,
	pub connexion_max: u32,
	pub time_unit: u32,
	pub teams_name: Vec<String>,
}

#[derive(Debug)]
pub struct	ServerState
{
	pub	clients: HashMap<i32, Client>,
	pub	listener: TcpListener,
	pub next_id: i32,
}

fn	setup_listener(addr: &String) -> TcpListener
{
	let	listener = TcpListener::bind(addr);

	match listener
	{
		Ok(listener) => {
			listener.set_nonblocking(true).expect("Cannot set non-blocking");
			println!("✅ Serveur ouvert sur: {}\n", addr);
			return listener;
		}
		Err(e) => {
			eprintln!("❌ Erreur lors de l'écoute sur {}: {}", addr, e);
			exit(-1);
		}
	}
}

fn	accept_new_client(clients: &mut HashMap<i32, Client>, listener: &TcpListener, next_id: &mut i32)
{
	if let Ok((stream, addr)) = listener.accept()
	{
		let client = Client::new(stream, addr, *next_id);
		client.set_nonblocking();
		clients.insert(*next_id, client);
		*next_id += 1;
	}
}

fn	disconnect_client(clients: &mut HashMap<i32, Client>, id: i32)
{
	if let Some(client) = clients.get(&id)
	{
		clients.remove(&id);
	}
	else
	{
		print!("{}", "[ERROR] Client inconnu déconnecté !".red())
	}
}

//TODO
fn	handle_client(clients: &mut HashMap<i32, Client>)
{
	let mut to_remove: Vec<i32> = vec![];

	for (id, client) in clients.iter_mut()
	{
		let mut buf = [0; 1024];
		let mut stream = client.get_stream();

		match stream.read(&mut buf)
		{
			Ok(0) => {to_remove.push(*id);}
			Ok(received) =>
			{
				let msg = String::from_utf8_lossy(&buf[..received]);

				println!("[DEBUG] Client [{id}] a envoyé : {}", msg.replace("\n", ""));
			}
			Err(_) => {
				// to_remove.push(*id);
			}
		}
	}
	for id in to_remove
	{
		disconnect_client(clients, id);
	}
}

pub fn	server_loop(server: &mut ServerState)
{
	accept_new_client(&mut server.clients, &server.listener, &mut server.next_id);
	handle_client(&mut server.clients);
}

pub fn	init_server(config: &ServerSettings) -> TcpListener
{
	let	addr: String = format!("127.0.0.1:{}", config.port);
	let	listener: TcpListener = setup_listener(&addr);

	return listener;
}
