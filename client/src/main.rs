/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/14 09:50:16 by tde-los-          #+#    #+#             */
/*   Updated: 2025/05/16 11:25:59 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

/*
	* --------------------------------------------------------------------
	* CLIENT
	* --------------------------------------------------------------------
	* Ce programme est un client autonome qui se connecte au serveur Zappy.
	* Il incarne un joueur (Trantorien) contrôlé par une IA. Une fois lancé,
	* il gère la connexion, reçoit les messages du serveur, interprète l'état
	* du jeu, prend des décisions, et envoie des commandes de manière autonome.
	*
	* Le client fonctionne en boucle :
	*   1. Se connecter au serveur via TCP (avec paramètres hostname, port, team).
	*   2. Recevoir le message de bienvenue + infos de la map.
	*   3. Répondre avec le nom de l'équipe.
	*   4. Boucle principale : lire les réponses du serveur, analyser la situation,
	*      décider de la prochaine commande, et l’envoyer.
	*
	* --------------------------------------------------------------------
	* TODO LIST CLIENT (IA) :
	*
	* [ ] Gérer la connexion TCP avec le serveur :
	*     - Ouverture de socket
	*     - Envoi du nom d'équipe
	*     - Lecture du message BIENVENUE et des infos map
	*
	* [ ] Implémenter la boucle principale de l’IA :
	*     - Lire les messages du serveur
	*     - Réagir à l’environnement (voir, inventaire, nourriture, etc.)
	*     - Prendre des décisions logiques (déplacement, ramassage, incantation)
	*
	* [ ] Implémenter les commandes de base :
	*     - voir / avance / gauche / droite / prend / pose / incantation / inventaire / fork / broadcast...
	*     - Respecter les délais d’exécution selon le temps serveur
	*
	* [ ] Gérer les priorités (stratégie simple) :
	*     - Se nourrir (prioritaire)
	*     - Explorer la carte
	*     - Ramasser des ressources
	*     - Coopérer à une incantation si possible
	*
	* [ ] Permettre l'envoi manuel de commandes :
	*     - Entrée utilisateur envoyée directement au serveur
*/

fn	main()
{
	println!("Client");
}
