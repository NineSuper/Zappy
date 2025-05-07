/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   server.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/01 22:12:16 by tde-los-          #+#    #+#             */
/*   Updated: 2025/05/07 15:12:18 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::process::exit;
use std::collections::HashMap;
use colored::*;

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

fn	accept_new_client(clients: &mut HashMap<i32, TcpStream>, listener: &TcpListener, next_id: &mut i32)
{
	if let Ok((stream, addr)) = listener.accept()
	{
		println!("{} {addr}", format!("[INFO] Client [{next_id}] connecté:").green());
		stream.set_nonblocking(true).expect("Cannot set non-blocking");
		clients.insert(*next_id, stream);
		*next_id += 1;
	}
}

fn	disconnect_client(clients: &mut HashMap<i32, TcpStream>, id: i32)
{
	if let Some(stream) = clients.remove(&id)
	{
		if let Ok(addr) = stream.peer_addr()
		{
			println!("{} {addr}", format!("[INFO] Client [{id}] déconnecté:").green());
		}
		else
		{
			println!("{} addresse inconnue", format!("[INFO] Client [{id}] déconnecté:").green());
		}
	}
	else
	{
		print!("{}", "[ERROR] Client inconnu déconnecté !".red())
	}
}

fn	handle_client(clients: &mut HashMap<i32, TcpStream>)
{
	let mut to_remove: Vec<i32> = vec![];

	for (id, stream) in clients.iter_mut()
	{
		let mut buf = [0; 1024];

		match stream.read(&mut buf)
		{
			Ok(0) => {to_remove.push(*id);}
			Ok(received) => {
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

pub fn	server_loop()
{

}

pub fn	init_server(port :u32)
{
	let	addr: String = format!("127.0.0.1:{}", port);
	let	listener: TcpListener = setup_listener(&addr);
	let mut	clients: HashMap<i32, TcpStream> = HashMap::new();
	let mut next_id: i32 = 0;

	loop
	{
		accept_new_client(&mut clients, &listener, &mut next_id);
		handle_client(&mut clients);
	}
}
