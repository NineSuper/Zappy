/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   graphics_broadcaster.rs                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/06/13 11:03:30 by tde-los-          #+#    #+#             */
/*   Updated: 2025/07/02 10:11:45 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::{
	collections::HashMap,
	io::{Read, Write},
	net::{TcpListener, TcpStream},
};

use colored::Colorize;

use crate::{
	game::{
		core::gamestate::GameState, entities::team::get_info_teams_json, world::map::get_map_json,
	},
	game_log,
};

#[derive(Debug)]
pub struct GraphicsBroadcaster
{
	pub ip: String,
	pub port: u32,
	pub listener: Option<TcpListener>,
	pub clients: HashMap<i32, TcpStream>,
	pub next_id: i32,
}

impl GraphicsBroadcaster
{
	pub fn new(ip: String, port: u32) -> Self
	{
		let mut new: GraphicsBroadcaster = Self {
			ip: ip,
			port: port,
			listener: None,
			clients: HashMap::new(),
			next_id: 0,
		};
		new.listener = new.setup_listener();
		return new;
	}

	pub fn setup_listener(&mut self) -> Option<TcpListener>
	{
		let addr = self.ip.clone() + ":" + &self.port.to_string();

		match TcpListener::bind(&addr)
		{
			Ok(listener) =>
			{
				listener.set_nonblocking(true).expect("Cannot set non-blocking");
				game_log!("{}\n", "==========================================".blue());
				game_log!("📊 Serveur graphique: {}\n", addr);
				game_log!("{}\n", "==========================================".blue());
				return Some(listener);
			}
			Err(e) =>
			{
				game_log!("❌ Erreur lors de l'écoute sur {}: {}", addr, e);
			}
		}
		return None;
	}

	pub fn accept_new_client(&mut self, game: &GameState)
	{
		match &self.listener
		{
			Some(listener) =>
			{
				if let Ok((mut stream, _)) = listener.accept()
				{
					self.send_json(self.next_id, &mut stream, get_map_json(&game.map));
					self.send_json(self.next_id, &mut stream, get_info_teams_json(&game.teams));
					self.clients.insert(self.next_id, stream);
					self.next_id += 1;
				}
			}
			_ =>
			{}
		}
	}

	pub fn send_json(&self, id: i32, stream: &mut TcpStream, msg: String)
	{
		if let Err(e) = stream.write_all(msg.as_bytes())
		{
			game_log!(
				"{} {}",
				format!("[ERROR] impossible d'envoyer un message au client_gfx {}:", id)
					.red()
					.bold(),
				e
			);
			return;
		}
		// game_log!(
		// 	"{} Server -> gfx #{}: {}",
		// 	"[DEBUG]".yellow().bold(),
		// 	id,
		// 	msg.trim_end().italic().cyan().bold()
		// );
	}

	pub fn disconnect_client(&mut self)
	{
		let mut to_remove = vec![];
		let mut buf = [0; 512];

		for (id, stream) in &mut self.clients
		{
			match stream.read(&mut buf)
			{
				Ok(0) => to_remove.push(*id),
				Ok(_) =>
				{
					continue;
				}
				Err(_) =>
				{
					// TODO Gérer en cas d'erreur
					continue;
				}
			}
		}
		for id in to_remove
		{
			self.clients.remove(&id);
		}
	}

	pub fn broadcast_message(&mut self, message: &String)
	{
		let clients_ptr = &mut self.clients as *mut HashMap<i32, TcpStream>;
		unsafe {
			let client_ids: Vec<i32> = (*clients_ptr).keys().copied().collect();

			for id in client_ids
			{
				if let Some(stream) = (*clients_ptr).get_mut(&id)
				{
					self.send_json(id, stream, message.to_string());
				}
			}
		}
	}

	pub fn update(&mut self, game: &GameState)
	{
		self.accept_new_client(game);
		self.disconnect_client();
	}
}
