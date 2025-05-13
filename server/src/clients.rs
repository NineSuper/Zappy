/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   clients.rs                                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/07 15:11:11 by tde-los-          #+#    #+#             */
/*   Updated: 2025/05/13 16:29:11 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::net::{IpAddr, SocketAddr, TcpStream};
use colored::*;

#[derive(Debug)]
pub struct Client
{
	pub id: i32,
	stream: TcpStream,
	addr: SocketAddr,
	commands: Vec<String>,
	pub online: bool,
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
		}
	}

    pub fn add_command(&mut self, command: String)
	{
        if self.commands.len() < 10
		{
        	self.commands.push(command.clone());
			print!("{} {}", format!("[DEBUG] Client #{} a envoyé :", self.id).cyan().italic(), command);
		}
    }

    pub fn remove_command(&mut self)
	{
        if !self.commands.is_empty()
		{
            self.commands.remove(0);
        }
    }

	pub fn	disconnect(&mut self)
	{
		println!("{} Client #{} (IP: {})", format!("[-]").red().bold(), self.id, self.addr);
	}

	pub fn	set_nonblocking(&self)
	{
		// TODO gérer le cas où il y'a une erreur
		self.stream.set_nonblocking(true).expect("Cannot set non-blocking");
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
