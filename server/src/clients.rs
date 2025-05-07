/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   clients.rs                                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/07 15:11:11 by tde-los-          #+#    #+#             */
/*   Updated: 2025/05/07 15:12:13 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::net::TcpStream;

#[derive(Debug, Clone)]
pub struct	Client
{
	stream: TcpStream,
	id: i32,
}

impl	Client
{
	pub fn new(stream: TcpStream, id: i32) -> Self
	{
		Client {stream, id}
	}
}
