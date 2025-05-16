/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   client.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/07 15:11:11 by tde-los-          #+#    #+#             */
/*   Updated: 2025/05/16 13:48:54 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::net::{IpAddr, SocketAddr, TcpStream};
use colored::*;
use std::io::{Write, Read};

#[derive(Debug)]
pub struct Client
{
	pub id: i32,
	stream: TcpStream,
	addr: SocketAddr,
	commands: Vec<String>,
	pub online: bool,
	pub player_id: Option<String>,
}

impl	Client
{
	pub fn	new(stream: TcpStream, addr: SocketAddr, id: i32) -> Self
	{
		println!("{} Client #{id} (IP: {addr})", format!("[+]").green().bold());
		Self
		{
			id: id,
			stream: stream,
			addr: addr,
			commands: vec![],
			online: true,
			player_id: None,
		}
	}

    pub fn	add_command(&mut self, command: String)
	{
        if self.commands.len() < 10
		{
        	self.commands.push(command.clone());
			println!("{} Client #{}: {}", "[RECV]".cyan().bold(), self.id, command.bold().cyan().italic());
			// println!("{} {}", format!("[DEBUG] Client #{} a envoyé :", self.id).cyan().italic(), command);
		}
    }

	pub fn	get_command(&self) -> Option<&String>
	{
		if let Some(msg) = self.commands.get(0)
		{
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
			println!("{} {}", format!("[ERROR] impossible d'envoyer un message au client {}:", self.id).red().bold(), e);
		}
		println!("{} Server -> Client #{}: {}", "[SEND]".blue().bold(), self.id, msg.trim_end().italic().cyan().bold());
	}

	pub fn	disconnect(&mut self)
	{
		println!("{} Client #{} (IP: {})", format!("[-]").red().bold(), self.id, self.addr);
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

	pub fn	set_nonblocking(&self)
	{
		// TODO à enlever interdit (sujet)
		// self.stream.set_nonblocking(true).expect("Cannot set non-blocking");
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
		self.online = false;
		self.disconnect();
	}
}
