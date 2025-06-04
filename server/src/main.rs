/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/04/29 15:23:47 by tde-los-          #+#    #+#             */
/*   Updated: 2025/06/03 17:12:01 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

/*
	* --------------------------------------------------------------------
	* SERVER
	* --------------------------------------------------------------------
	* Le serveur est le cœur du projet Zappy.
	* Il héberge et gère l’ensemble du monde : la carte, les ressources,
	* les équipes, les joueurs, le temps, les règles du jeu.
	*
	* Il écoute les connexions entrantes des clients IA et du client graphique,
	* traite leurs commandes selon les règles du jeu, et renvoie les réponses
	* appropriées.
	*
	* --------------------------------------------------------------------
	* TODO LIST SERVER
	*
	* [x] Gérer les arguments du serveur :
	*     -p <port> : port d’écoute
	*     -x <width> -y <height> : dimensions de la carte
	*     -n <team1> [team2...] : noms des équipes
	*     -c <nb_clients> : nb de joueurs autorisés par équipe
	*     -t <time_unit> : unité de temps (plus t est grand, plus c’est rapide)
	*
	* [x] Initialiser le monde :
	*     - Générer une carte torique de dimensions x * y
	*     - Placer aléatoirement des ressources sur chaque case (avec règles)
	*     - Initialiser les équipes (nom, slots, œufs)
	*
	* [x] Gérer les connexions clients :
	*     - Associer chaque client à une structure `Client`
	*     - Lire/écrire dans les sockets
	*     - Accepter les sockets clients (IA ou graphique)
	*
	* [ ] Implémenter le système de joueurs :
	*     - Chaque client est lié à un joueur (position, orientation, inventaire, etc.)
	*     - Gérer le temps de vie (nourriture consommée avec le temps)
	*     - Gérer le fork / naissance / mort
	*
	* [x] Implémenter le moteur de commandes :
	*     - Lecture de la file de commandes du client
	*     - Exécution dans l’ordre avec le bon délai
	*     - Réponses envoyées via socket
	*     - Commandes à gérer : avance, voir, inventaire, prend, pose, incantation, etc.
	*
	* [x] Implémenter le protocole réseau :
	*     - Format des messages (`BIENVENUE\n`, `message <k>,<txt>`, etc.)
	*     - Gestion du buffer (max 10 commandes par client en attente)
	*
	* [x] Gérer le temps :
	*     - Boucle serveur basée sur un "tick" système
	*     - Exécution différée des commandes selon leur coût (ex : avance = 7/t)
	*     - Pas de `sleep()` bloquant → système non-bloquant
	*
	* [ ] Implémenter la gestion d’équipe :
	*     - Suivre le nombre de joueurs actifs par équipe
	*     - Gérer les œufs et `connect_nbr`
	*     - Vérifier la victoire (6 joueurs niveau max dans une même équipe)
	*
	* [x] (Optionnel) Logs/debug :
	*     - Messages serveur
	*     - Connexions, exécutions, erreurs
	*
*/

mod app;
mod client;
mod config;
mod game;
mod gui;
mod server;
mod utils;

use app::AppState;
use game::core::state::{game_init, game_loop};
use gui::display::{display_cleanup, display_init};
use server::{init_server, ServerSettings, ServerState};

use std::collections::HashMap;

fn main()
{
	let mut config: ServerSettings = config::env::init_env();

	let mut app_state: AppState = AppState {
		game: game_init(&mut config),
		server: ServerState {
			clients: HashMap::new(),
			listener: init_server(&config),
			connexion_max: config.connexion_max,
			next_id: 0,
		},
		settings: config,
	};
	display_init();
	game_loop(&mut app_state);
	display_cleanup();
}
