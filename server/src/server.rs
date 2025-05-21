/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   server.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/01 22:12:16 by tde-los-          #+#    #+#             */
/*   Updated: 2025/05/21 16:18:41 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::net::TcpListener;
use std::process::exit;
use std::collections::HashMap;
use colored::*;

use crate::client::Client;
use crate::game::core::state::GameState;
use crate::game::entities::team::add_client_team;
use crate::game::entities::player::get_player_by_client_id;

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

fn	disconnect_client(clients: &mut HashMap<i32, Client>, game_state: &mut GameState, id: i32)
{
    if let Some(client) = clients.get(&id)
    {
        if client.team_id != 0
        {
            for team in &mut game_state.teams
            {
                if team.id == client.team_id
                {
                    if let Some(pos) = team.players.iter().position(|p| p.client_id == Some(id))
                    {
                        println!("{} Joueur {} est mort", "[DEATH]".red().bold(), team.players[pos].id);
                        team.players.remove(pos);
                    }
                    break;
                }
            }
        }
    }
    clients.remove(&id);
}

fn	handle_first_command(client: &mut Client, game_state: &mut GameState) -> bool
{
    let command = match client.update_commands()
    {
        Some(cmd) => cmd,
        None => return false,
    };

    let team_name = command.trim();
    println!("{} Tentative de connexion à l'équipe: {}", "[DEBUG]".yellow().bold(), team_name);

    let team_exists = game_state.teams.iter().any(|team| team.name == team_name);
    if !team_exists
    {
        println!("{} Équipe {} n'existe pas", "[ERROR]".red().bold(), team_name);
        client.send_message("ko\n".to_string());
        client.remove_command();
        return false;
    }

    let player_id = match add_client_team(team_name.to_string(), &mut game_state.teams, client.id)
    {
        Some(id) => id,
        None =>
        {
            println!("{} Impossible de rejoindre l'équipe {}", "[ERROR]".red().bold(), team_name);
            client.send_message("ko\n".to_string());
            client.remove_command();
            return false;
        }
    };

    let team_id = game_state.teams.iter()
        .find(|team| team.name == team_name)
        .map(|team| team.id)
        .unwrap_or(0);

    client.player_id = Some(player_id);
    client.team_id = team_id;
    client.send_message("ok\n".to_string());
    client.remove_command();

    println!("{} Client #{} a rejoint l'équipe {}", "[SUCCESS]".green().bold(), client.id, team_name);

    return true;
}

pub fn	handle_client(client: &mut Client, game_state: &mut GameState)
{
	if client.team_id == 0 {
        if !handle_first_command(client, game_state) {
            return;
        }
    }
    if let Some(command) = client.update_commands()
	{
        let mut parts = command.trim().splitn(2, ' ');
        let action = parts.next().unwrap_or("");
        let args = parts.next();
        let player = get_player_by_client_id(game_state, client.id);

        if player.is_none()
		{
            client.send_message("Pas de joueur associé à ce client\n".to_string());
            return;
        }
        let player = player.unwrap();
        match action
		{
            "avance" =>
			{
                player.move_forward();
                client.send_message("ok\n".to_string());
            }
            "droite" =>
			{
                player.turn_right();
                client.send_message("ok\n".to_string());
            }
            "gauche" =>
			{
                player.turn_left();
                client.send_message("ok\n".to_string());
            }
            "voir" =>
			{
                client.send_message("{case1, case2, ...}\n".to_string());
            }
            "inventaire" =>
			{
                client.send_message(player.get_inventory());
            }
            "prend" =>
			{
                if let Some(object) = args
				{
                    client.send_message("ok\n".to_string());
                }
                else
				{
                    client.send_message("ko\n".to_string());
                }
            }
            "pose" =>
			{
                if let Some(object) = args
				{
                    client.send_message("ok\n".to_string());
                }
                else
				{
                    client.send_message("ko\n".to_string());
                }
            }
            "expulse" =>
			{
                client.send_message("ok\n".to_string());
            }
            "broadcast" =>
			{
                if let Some(message) = args
				{
                    client.send_message("ok\n".to_string());
                }
                else
				{
                    client.send_message("ko\n".to_string());
                }
            }
            "incantation" =>
			{
                client.send_message("elevation en cours\nniveau actuel: K\n".to_string());
            }
            "fork" =>
			{
                client.send_message("ok\n".to_string());
            }
            "connect_nbr" =>
			{
                client.send_message("0\n".to_string());
            }
            _ =>
			{
                client.send_message("Commande Inconnue\n".to_string());
            }
        }
    }
}

pub fn	server_loop(server: &mut ServerState, game_state: &mut GameState)
{
    let mut to_remove = vec![];

    if server.clients.len() < server.connexion_max.try_into().unwrap()
    {
        accept_new_client(server);
    }
    for (id, client) in server.clients.iter_mut()
    {
        if !client.read_from_stream()
        {
            to_remove.push(*id);
        }
    }
    for id in to_remove
    {
        disconnect_client(&mut server.clients, game_state, id);
    }
}

pub fn	init_server(config: &ServerSettings) -> TcpListener
{
	let	addr: String = format!("127.0.0.1:{}", config.port);
	let	listener: TcpListener = setup_listener(&addr);

	return listener;
}
