/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   player.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: tde-los- <tde-los-@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/02 08:33:16 by tde-los-          #+#    #+#             */
/*   Updated: 2025/06/04 14:08:19 by tde-los-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use super::inventory::Inventory;
use crate::game::world::map;
use crate::{game::core::gamestate::GameState, game_log};
use crate::game::entities::object::Objet;


use colored::*;

/*
	* Player._id: ID_TEAM + _ + ID_PLAYER
*/

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Player {
    pub id: String,
    pub pos_x: i32,
    pub pos_y: i32,
    pub inventory: Inventory,
    life_unit: i32,
    pub level: u32,
    direction: Direction,
    health_points: i32,
    pub client_id: Option<i32>,
}

impl Player {
    pub fn new(id: String) -> Self {
        let mut player = Self {
            id: id,
            pos_x: 1, // TODO
            pos_y: 1, // TODO
            inventory: Inventory::new(),
            life_unit: 10,
            level: 1,
            direction: Direction::North,
            health_points: 100,
            client_id: None,
        };
        player.inventory.add(Objet::Food, 10);
        return player;
    }

    pub fn take_object(&mut self, map: &mut Vec<Vec<map::Cell>>, obj: Objet) -> bool {
        if map::take_object(map, self.pos_x, self.pos_y, obj.clone()) {
            // DEBUG
            game_log!(
                "{} {}",
                format!("[DEBUG] Client #{} a récupéré:", self.id)
                    .yellow()
                    .italic(),
                obj.name()
            );

            self.inventory.add(obj, 1);
            return true;
        }
        return false;
    }

    pub fn drop_object(&mut self, map: &mut Vec<Vec<map::Cell>>, obj: Objet) -> bool {
        // if map::drop_object(map, self.pos_x, self.pos_y, obj.clone())
        if self.inventory.remove(obj.clone(), 1) {
            // DEBUG
            game_log!(
                "{} {}",
                format!("[DEBUG] Client #{} a laché:", self.id)
                    .yellow()
                    .italic(),
                obj.name()
            );

            // return self.inventory.remove(obj, 1);
            return map::drop_object(map, self.pos_x, self.pos_y, obj.clone());
        }
        return false;
    }

    /*
        TODO Ajouter du temps de vie
        One 'nourriture' unit allows him to survive 126 time units,
        therefore 126/t seconds.
    */
    pub fn _eat(&mut self) -> bool {
        if self.inventory.get(Objet::Food) > 0 {
            // DEBUG
            game_log!(
                "{}",
                format!("[DEBUG] Joueur: {} vient de manger !", self.id)
                    .yellow()
                    .italic()
            );

            return self.inventory.remove(Objet::Food, 1);
        }
        // DEBUG
        game_log!(
            "{}",
            format!(
                "[DEBUG] Joueur: {} n'a pas de nourriture dans son inventaire !",
                self.id
            )
            .yellow()
            .italic()
        );
        return false;
    }

    // Retourne un tuple de coordonées (x, y)
    pub fn get_position(&self) -> (i32, i32) {
        return (self.pos_x, self.pos_y);
    }

    pub fn move_forward(&mut self, map: &Vec<Vec<map::Cell>>) {
        let map_width = map[0].len() as i32;
        let map_height = map.len() as i32;

        match self.direction {
            Direction::North => self.pos_y = (self.pos_y - 1 + map_height) % map_height,
            Direction::South => self.pos_y = (self.pos_y + 1) % map_height,
            Direction::East => self.pos_x = (self.pos_x + 1) % map_width,
            Direction::West => self.pos_x = (self.pos_x - 1 + map_width) % map_width,
        }
    }

    pub fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub fn turn_left(&mut self) {
        self.direction = match self.direction {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }

    pub fn _get_direction(&self) -> &Direction {
        &self.direction
    }

    pub fn _get_health_points(&self) -> i32 {
        self.health_points
    }

    pub fn get_vision_range(&self) -> i32 {
        self.level as i32
    }

    pub fn get_vision(&self, game_state: &GameState) -> String {
        let range = self.get_vision_range();
        let (x, y) = self.get_position();
        let map_height = game_state.map.len() as i32;
        let map_width = game_state.map[0].len() as i32;

        let mut vision_cells: Vec<String> = Vec::new();

        for dist in 0..=range {
            for offset in -dist..=dist {
                let (view_x, view_y) = match self.direction {
                    Direction::North => (
                        (x + offset + map_width) % map_width,
                        (y - dist + map_height) % map_height,
                    ),
                    Direction::South => (
                        (x - offset + map_width) % map_width,
                        (y + dist + map_height) % map_height,
                    ),
                    Direction::East => (
                        (x + dist + map_width) % map_width,
                        (y + offset + map_height) % map_height,
                    ),
                    Direction::West => (
                        (x - dist + map_width) % map_width,
                        (y - offset + map_height) % map_height,
                    ),
                };

                let cell = &game_state.map[view_y as usize][view_x as usize];
                let mut cell_items: Vec<String> = Vec::new();

                for (obj, count) in &cell.content {
                    for _ in 0..*count {
                        cell_items.push(obj.name().to_lowercase());
                    }
                }

                for team in &game_state.teams {
                    for player in &team.players {
                        let (player_x, player_y) = player.get_position();
                        if player_x == view_x && player_y == view_y && player.id != self.id {
                            cell_items.push("player".to_string());
                        }
                    }
                }

                let cell_content = if cell_items.is_empty() {
                    if vision_cells.is_empty() {
                        " ".to_string()
                    } else {
                        "".to_string()
                    }
                } else {
                    cell_items.join(" ")
                };
                vision_cells.push(cell_content);
            }
        }
        let vision = format!("{{{}}}\n", vision_cells.join(", "));
        return vision;
    }

    pub fn get_inventory(&self) -> String {
        let mut response = String::from("{");
        let inventory = self.inventory.get_all_objects();

        for (i, (obj, count)) in inventory.iter().enumerate() {
            if i > 0 {
                response.push_str(", ");
            }
            response.push_str(&format!("{} {}", obj.name().to_lowercase(), count));
        }
        response.push_str("}\n");
        response
    }
}
