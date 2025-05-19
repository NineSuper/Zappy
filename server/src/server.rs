/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   server.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/01 22:12:16 by tde-los-          #+#    #+#             */
/*   Updated: 2025/05/19 15:23:47 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::net::TcpListener;
use std::process::exit;
use std::collections::HashMap;
use colored::*;

use crate::client::Client;
use crate::game::core::state::GameState;
use crate::game::entities::team::add_client_team;

#[derive(Debug, Clone)]
pub struct	ServerSettings
{
	pub port: u32,
	pub width: u32,
	pub height: u32,
	pub connexion_max: u32,
	pub time_unit: f64,
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
		Ok(listener) =>
		{
			listener.set_nonblocking(true).expect("Cannot set non-blocking");
			println!("🌍 Serveur ouvert sur: {}\n", addr);
			return listener;
		}
		Err(e) =>
		{
			println!("❌ Erreur lors de l'écoute sur {}: {}", addr, e);
			exit(-1);
		}
	}
}

fn	accept_new_client(server: &mut ServerState)
{
	if let Ok((stream, addr)) = server.listener.accept()
	{
		let mut client = Client::new(stream, addr, server.next_id);

		client.send_message("BIENVENUE\n".to_string());
		server.clients.insert(server.next_id, client);
		server.next_id += 1;
	}
}

fn	disconnect_client(clients: &mut HashMap<i32, Client>, id: i32)
{
	clients.remove(&id);
}

pub fn	handle_client(client: &mut Client)
{
	if let Some(command) = client.get_command()
	{
		let mut	parts = command.trim().splitn(2, ' ');
		let	action = parts.next().unwrap_or("");
		let	args = parts.next();

		match action
		{
			"avance" => {
				client.send_message("ok\n".to_string());
			}
			"droite" => {
				client.send_message("ok\n".to_string());
			}
			"gauche" => {
				client.send_message("ok\n".to_string());
			}
			"voir" => {
				client.send_message("{case1, case2, ...}\n".to_string());
			}
			"inventaire" => {
				client.send_message("{phiras n, sibur n, ...}\n".to_string());
			}
			"prend" => {
				if let Some(object) = args
				{
					client.send_message("ok\n".to_string());
				}
				else {
					client.send_message("ko\n".to_string());
				}
			}
			"pose" => {
				if let Some(object) = args
				{
					client.send_message("ok\n".to_string());
				}
				else {
					client.send_message("ko\n".to_string());
				}
			}
			"expulse" => {
				client.send_message("ok\n".to_string());
			}
			"broadcast" => {
				if let Some(message) = args {
					client.send_message("ok\n".to_string());
				} else {
					client.send_message("ko\n".to_string());
				}
			}
			"incantation" => {
				client.send_message("elevation en cours\nniveau actuel: K\n".to_string());
			}
			"fork" => {
				client.send_message("ok\n".to_string());
			}
			"connect_nbr" => {
				client.send_message("0\n".to_string());
			}
			_ => {
				client.send_message("Commande Inconnue\n".to_string());
			}
		}
		client.remove_command();
	}
}

pub fn	server_loop(server: &mut ServerState)
{
	let mut to_remove = vec![];

	if server.clients.len() < server.connexion_max.try_into().unwrap() {
		accept_new_client(server);
	}
	for (id, client) in server.clients.iter_mut()
	{
		if !client.read_from_stream() {
			to_remove.push(*id);
		}
		// handle_client(client);
	}
	for id in to_remove
	{
		disconnect_client(&mut server.clients, id);
	}
}

pub fn	init_server(config: &ServerSettings) -> TcpListener
{
	let	addr: String = format!("127.0.0.1:{}", config.port);
	let	listener: TcpListener = setup_listener(&addr);

	return listener;
}
