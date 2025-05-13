/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   clients.rs                                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/07 15:11:11 by tde-los-          #+#    #+#             */
/*   Updated: 2025/05/13 10:30:46 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::net::{TcpStream, SocketAddr, IpAddr};
use colored::*;

#[derive(Debug)]
pub struct Client
{
	pub id: i32,
	stream: TcpStream,
	addr: SocketAddr,
	pub online: bool,
}

impl	Client
{
	pub fn	new(stream: TcpStream, addr: SocketAddr, id: i32) -> Self
	{
		println!("{} {addr}", format!("[INFO] Client [{id}] connecté:").green());
		Self
		{
			id: id,
			stream: stream,
			addr: addr,
			online: true,
		}
	}

	pub fn	disconnect(&mut self)
	{
		let id = self.id;
		println!("{} {}", format!("[INFO] Client [{id}] déconnecté:").green(), self.addr);
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
		self.disconnect();
	}
}
