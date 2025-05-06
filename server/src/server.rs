/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   server.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/01 22:12:16 by tde-los-          #+#    #+#             */
/*   Updated: 2025/05/06 18:51:19 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::net::{TcpListener, TcpStream};
use std::process::exit;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ServerConfig
{
    pub _port: u32,
    pub width: u32,
    pub height: u32,
    pub _clients: u32,
    pub _time_unit: u32,
    pub teams: Vec<String>,
}

fn	setup_listener(addr: &String) -> TcpListener
{
    let	listener = TcpListener::bind(addr);

    match listener
    {
        Ok(listener) => {
			// listener.set_nonblocking(true);
            println!("✅ Serveur ouvert sur: {}", addr);
            return listener;
        }
        Err(e) => {
            eprintln!("❌ Erreur lors de l'écoute sur {}: {}", addr, e);
			exit(-1);
        }
    }
}

fn	handle_client()
{

}

fn	accept_new_clients(listener: &TcpListener)
{
	match listener.accept()
	{
		Ok((_socket, addr)) => println!("[DEBUG] Nouveau client: {addr:?}"),
		Err(e) => println!("couldn't get client: {e:?}"),
	}
}

pub fn	init_server(config: ServerConfig)
{
	let	addr = format!("127.0.0.1:{}", config._port);
	let	listener = setup_listener(&addr);
	let mut	clients: HashMap<i32, TcpStream> = HashMap::new();

	loop
	{
		accept_new_clients(&listener);
		handle_client();
	}
}
