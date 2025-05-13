/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   server.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/01 22:12:16 by tde-los-          #+#    #+#             */
/*   Updated: 2025/05/13 16:50:32 by tde-los-         ###   ########.fr       */
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
	pub connexion_max: u32,
}

fn	setup_listener(addr: &String) -> TcpListener
{
	let	listener = TcpListener::bind(addr);

	match listener
	{
		Ok(listener) => {
			listener.set_nonblocking(true).expect("Cannot set non-blocking");
			println!("🌍 Serveur ouvert sur: {}\n", addr);
			return listener;
		}
		Err(e) => {
			eprintln!("❌ Erreur lors de l'écoute sur {}: {}", addr, e);
			exit(-1);
		}
	}
}

fn	accept_new_client(server: &mut ServerState)
{
	if let Ok((stream, addr)) = server.listener.accept()
	{
		let client = Client::new(stream, addr, server.next_id);
		client.set_nonblocking();
		server.clients.insert(server.next_id, client);
		server.next_id += 1;
	}
}

fn	disconnect_client(clients: &mut HashMap<i32, Client>, id: i32)
{
	if let Some(_client) = clients.get(&id) {
		clients.remove(&id);
	}
	else {
		print!("{}", "[ERROR] Client inconnu déconnecté !".red().bold())
	}
}

//TODO
fn	handle_client(clients: &mut HashMap<i32, Client>)
{
	let mut to_remove: Vec<i32> = vec![];

	for (id, client) in clients.iter_mut()
	{
		let mut stream = client.get_stream();
		let mut buf = [0; 1024];

		match stream.read(&mut buf)
		{
			Ok(0) => {to_remove.push(*id);}
			Ok(received) =>
			{
				let msg = String::from_utf8_lossy(&buf[..received]);

				client.add_command(msg.to_string());
				client.remove_command(); // DEBUG
			}
			Err(_) => {
				// to_remove.push(*id);
			}
		}
	}
	for id in to_remove {
		disconnect_client(clients, id);
	}
}

pub fn	server_loop(server: &mut ServerState)
{
	if server.clients.len() < server.connexion_max.try_into().unwrap() {
		accept_new_client(server);
	}
	handle_client(&mut server.clients);
}

pub fn	init_server(config: &ServerSettings) -> TcpListener
{
	let	addr: String = format!("127.0.0.1:{}", config.port);
	let	listener: TcpListener = setup_listener(&addr);

	return listener;
}
