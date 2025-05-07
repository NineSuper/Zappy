/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   clients.rs                                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/07 15:11:11 by tde-los-          #+#    #+#             */
/*   Updated: 2025/05/08 00:56:40 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::net::{TcpStream, SocketAddr, IpAddr};

#[derive(Debug)]
pub struct Client
{
	pub id: i32,
	pub stream: TcpStream,
	pub addr: SocketAddr,
	pub online: bool,
}

impl	Client
{
	pub fn	new(stream: TcpStream, id: i32) -> Self
	{
		let addr = stream
			.peer_addr()
			.expect("[ERROR] Impossible de récupérer l'adresse du client");
		Self
		{
			id,
			stream,
			addr,
			online: true,
		}
	}

	pub fn get_addr(&self) -> SocketAddr
	{
		self.addr
	}

	pub fn get_ip(&self) -> IpAddr
	{
		self.addr.ip()
	}

	pub fn get_port(&self) -> u16
	{
		self.addr.port()
	}
}

