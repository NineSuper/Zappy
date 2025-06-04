/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   server.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/01 22:12:16 by tde-los-          #+#    #+#             */
/*   Updated: 2025/06/04 17:50:54 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use colored::*;
use std::collections::HashMap;
use std::net::TcpListener;
use std::process::exit;

use crate::client::client::Client;
use crate::game::core::state::GameState;
use crate::game::entities::team::add_client_team;
use crate::game_log;

#[derive(Debug, Clone)]
pub struct ServerSettings {
    pub port: u32,
    pub width: u32,
    pub height: u32,
    pub connexion_max: u32,
    pub time_unit: f64,
    pub teams_name: Vec<String>,
}

#[derive(Debug)]
pub struct ServerState {
    pub clients: HashMap<i32, Client>,
    pub listener: TcpListener,
    pub next_id: i32,
    pub connexion_max: u32,
}

fn setup_listener(addr: &String) -> TcpListener {
    let listener = TcpListener::bind(addr);

    match listener {
        Ok(listener) => {
            listener
                .set_nonblocking(true)
                .expect("Cannot set non-blocking");
            game_log!("🌍 Serveur ouvert sur: {}\n", addr);
            return listener;
        }
        Err(e) => {
            println!("❌ Erreur lors de l'écoute sur {}: {}", addr, e);
            exit(-1);
        }
    }
}

fn accept_new_client(server: &mut ServerState) {
    if let Ok((stream, addr)) = server.listener.accept() {
        let mut client = Client::new(stream, addr, server.next_id);

        client.send_message("BIENVENUE\n".to_string());
        server.clients.insert(server.next_id, client);
        server.next_id += 1;
    }
}

fn disconnect_client(clients: &mut HashMap<i32, Client>, game_state: &mut GameState, id: i32) {
    if let Some(client) = clients.get(&id) {
        if client.team_id != 0 {
            for team in &mut game_state.teams {
                if team.id == client.team_id {
                    if let Some(pos) = team.players.iter().position(|p| p.client_id == Some(id)) {
                        game_log!(
                            "{} Joueur {} est mort",
                            "[DEATH]".red().bold(),
                            team.players[pos].id
                        );
                        team.players.remove(pos);
                    }
                    break;
                }
            }
        }
    }
    clients.remove(&id);
}

fn player_exists(game_state: &GameState, client_id: i32) -> bool {
    for team in &game_state.teams {
        for player in &team.players {
            if player.client_id == Some(client_id) {
                return true;
            }
        }
    }
    false
}

fn handle_first_command(client: &mut Client, game_state: &mut GameState) -> bool {
    let command = match client.update_commands() {
        Some(cmd) => cmd,
        None => return false,
    };

    let team_name = command.trim();
    // game_log!("{} Tentative de connexion à l'équipe: {}", "[DEBUG]".yellow().bold(), team_name);

    let team_exists = game_state.teams.iter().any(|team| team.name == team_name);
    if !team_exists {
        game_log!(
            "{} Équipe {} n'existe pas",
            "[ERROR]".red().bold(),
            team_name
        );
        client.send_message("ko\n".to_string());
        client.remove_command();
        return false;
    }

    let player_id = match add_client_team(team_name.to_string(), &mut game_state.teams, client.id) {
        Some(id) => id,
        None => {
            game_log!(
                "{} Impossible de rejoindre l'équipe {}",
                "[ERROR]".red().bold(),
                team_name
            );
            client.send_message("ko\n".to_string());
            client.remove_command();
            return false;
        }
    };

    let team_id = game_state
        .teams
        .iter()
        .find(|team| team.name == team_name)
        .map(|team| team.id)
        .unwrap_or(0);

    let connect_nbr = game_state
        .teams
        .iter()
        .find(|team| team.name == team_name)
        .map(|team| team.get_connect_nbr())
        .unwrap_or(0);

    let map_height = game_state.map.len();
    let map_width = game_state.map[0].len();

    client.player_id = Some(player_id);
    client.team_id = team_id;
    client.send_message(format!("{}\n{} {}\n", connect_nbr, map_width, map_height));
    client.remove_command();

    game_log!(
        "{} Client #{} a rejoint l'équipe {}",
        "[SUCCESS]".green().bold(),
        client.id,
        team_name
    );

    return true;
}

pub fn handle_client(client: &mut Client, game_state: &mut GameState) {
    if client.team_id == 0 {
        if !handle_first_command(client, game_state) {
            return;
        }
    }
    if let Some(command) = client.update_commands() {
        let mut parts = command.trim().splitn(2, ' ');
        let action: &str = parts.next().unwrap_or("");
        let args: Option<&str> = parts.next();

        if !player_exists(game_state, client.id) {
            client.send_message("Vous n'avez pas de joueur associé !\n".to_string());
            return;
        }
        match action {
            "avance" => {
                game_state.move_player_forward(client.id);
                client.send_message("ok\n".to_string());
            }
            "droite" => {
                game_state.turn_player_right(client.id);
                client.send_message("ok\n".to_string());
            }
            "gauche" => {
                game_state.turn_player_left(client.id);
                client.send_message("ok\n".to_string());
            }
            "voir" => {
                let vision = game_state.get_player_vision(client.id);
                client.send_message(vision);
            }
            "inventaire" => {
                let inventory = game_state.get_player_inventory(client.id);
                client.send_message(inventory);
            }
            "prend" => {
                if let Some(object) = args {
                    if game_state.player_take_object(client.id, object) {
                        client.send_message("ok\n".to_string());
                    } else {
                        client.send_message("ko\n".to_string());
                    }
                } else {
                    client.send_message("ko\n".to_string());
                }
            }
            "pose" => {
                if let Some(object) = args {
                    if game_state.player_drop_object(client.id, object) {
                        client.send_message("ok\n".to_string());
                    } else {
                        client.send_message("ko\n".to_string());
                    }
                } else {
                    client.send_message("ko\n".to_string());
                }
            }
            "expulse" => {
                // game_state.player_expulse(client.id);
                client.send_message("ok\n".to_string());
            }
            "broadcast" => {
                if let Some(message) = args {
                    // game_state.player_broadcast(client.id, message);
                    client.send_message("ok\n".to_string());
                } else {
                    client.send_message("ko\n".to_string());
                }
            }
            "incantation" => {
                // let level = game_state.get_player_level(client.id);
                client.send_message("elevation en cours\nniveau actuel: K\n".to_string());
            }
            "fork" => {
                // game_state.player_fork(client.id);
                client.send_message("ok\n".to_string());
            }
            "connect_nbr" => {
                // let connections = game_state.get_team_free_connections(client.id);
                client.send_message("0\n".to_string());
            }
            _ => {
                client.send_message("Commande Inconnue\n".to_string());
            }
        }
    }
}

pub fn server_loop(server: &mut ServerState, game_state: &mut GameState) {
    let mut to_remove = vec![];

    if server.clients.len() < server.connexion_max.try_into().unwrap() {
        accept_new_client(server);
    }
    for (id, client) in server.clients.iter_mut() {
        if !client.read_from_stream() {
            to_remove.push(*id);
        }
    }
    for id in to_remove {
        disconnect_client(&mut server.clients, game_state, id);
    }
}

pub fn init_server(config: &ServerSettings) -> TcpListener {
    let addr: String = format!("127.0.0.1:{}", config.port);
    let listener: TcpListener = setup_listener(&addr);

    return listener;
}
