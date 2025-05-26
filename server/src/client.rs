/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   client.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/07 15:11:11 by tde-los-          #+#    #+#             */
/*   Updated: 2025/05/26 18:04:54 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::net::{IpAddr, SocketAddr, TcpStream};
use colored::*;
use std::io::{Write, Read};

use crate::game_log;

#[derive(Debug)]
pub struct PendingCommand {
	command: String,
	ticks_remaining: u32,
}

#[derive(Debug)]
pub struct Client
{
	pub id: i32,
	stream: TcpStream,
	addr: SocketAddr,
	commands: Vec<String>,
	pub player_id: Option<String>,
	pub team_id: u32,
	pending_commands: Vec<PendingCommand>,
}

impl	Client
{
	pub fn	new(stream: TcpStream, addr: SocketAddr, id: i32) -> Self
	{
		game_log!("{} Client #{id} (IP: {addr})", format!("[+]").green().bold());
		Self
		{
			id: id,
			stream: stream,
			addr: addr,
			commands: vec![],
			player_id: None,
			team_id: 0,
			pending_commands: Vec::new(),
		}
	}

    pub fn	add_command(&mut self, command: String)
	{
		let ticks = match command.trim()
		{
			"avance" | "droite" | "gauche" => 7,
			"voir" => 7,
			"inventaire" => 1,
			"prend" | "pose" => 7,
			"expulse" => 7,
			"broadcast" => 7,
			"incantation" => 300,
			"fork" => 42,
			"connect_nbr" => 0,
			_ => 0,
		};

		if self.pending_commands.len() < 10
		{
			game_log!("{} Client #{}: {}", "[RECV]".cyan().bold(), self.id, command.bold().cyan().italic());
			self.pending_commands.push(PendingCommand {
				command,
				ticks_remaining: ticks,
			});
		}
    }

	pub fn	get_command(&self) -> Option<&String>
	{
		if let Some(msg) = self.commands.get(0)
		{
			game_log!("{}", msg);
			return Some(msg);
		}
		return None;
	}

    pub fn	remove_command(&mut self)
	{
        if !self.commands.is_empty()
		{
            self.commands.remove(0);
        }
    }

	pub fn	send_message(&mut self, msg: String)
	{
		if let Err(e) = self.stream.write_all(msg.as_bytes())
		{
			game_log!("{} {}", format!("[ERROR] impossible d'envoyer un message au client {}:", self.id).red().bold(), e);
		}
		game_log!("{} Server -> Client #{}: {}", "[SEND]".blue().bold(), self.id, msg.trim_end().italic().cyan().bold());
	}

	pub fn	disconnect(&mut self)
	{
		game_log!("{} Client #{} (IP: {})", format!("[-]").red().bold(), self.id, self.addr);
	}

	pub fn	read_from_stream(&mut self) -> bool
	{
		let mut buf = [0; 512];
		let mut stream = self.get_stream();

		match stream.read(&mut buf)
		{
			Ok(0) => return false,
			Ok(received) =>
			{
				let msg = String::from_utf8_lossy(&buf[..received]);
				let clean_msg = msg.trim();

				self.add_command(clean_msg.to_string());
				return true;
			}
			Err(_) =>
			{
				// TODO Gérer en cas d'erreur
				return true;
			}
		}
	}

	pub fn update_commands(&mut self) -> Option<String>
	{
		if let Some(cmd) = self.pending_commands.first_mut()
		{
			if cmd.ticks_remaining > 0
			{
				cmd.ticks_remaining -= 1;
				return None
			}
			else
			{
				let command = cmd.command.clone();
				self.pending_commands.remove(0);
				return Some(command);
			}
		}
		return None;
	}

	pub	fn	get_stream(&self) -> &TcpStream { &self.stream }
	pub fn	get_addr(&self) -> SocketAddr { self.addr }
	pub fn	get_ip(&self) -> IpAddr { self.addr.ip() }
	pub fn	get_port(&self) -> u16 { self.addr.port() }
}

impl	Drop for Client
{
	fn	drop(&mut self)
	{
		self.disconnect();
	}
}
