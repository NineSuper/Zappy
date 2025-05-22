import tkinter as tk
from tkinter import ttk
import socket
import threading
import json
from typing import List, Dict, Tuple, Set
import queue
import time
from dataclasses import dataclass
from enum import Enum

class Direction(Enum):
    NORTH = 0
    EAST = 1
    SOUTH = 2
    WEST = 3

class ItemType(Enum):
    FOOD = "nourriture"
    PLAYER = "player"
    LINEMATE = "linemate"
    DERAUMERE = "deraumere"
    SIBUR = "sibur"
    MENDIANE = "mendiane"
    PHIRAS = "phiras"
    THYSTAME = "thystame"

@dataclass
class Tile:
    items: Set[str]
    is_visible: bool
    canvas_id: int = None
    item_ids: List[int] = None

    def __post_init__(self):
        if self.item_ids is None:
            self.item_ids = []

class ZappyGUI:
    def __init__(self, root: tk.Tk):
        self.root = root
        self.root.title("Zappy Client")

        # Configuration du socket
        self.socket = None
        self.message_queue = queue.Queue()
        self.running = True
        self.last_command = None
        self.waiting_for_response = False

        # État du joueur
        self.player_pos = (1, 1)
        self.player_direction = Direction.NORTH
        self.visible_items = {}

        # Configuration des items
        self.item_colors = {
            ItemType.FOOD: "#90EE90",      # Vert clair
            ItemType.PLAYER: "#FF0000",    # Rouge
            ItemType.LINEMATE: "#0000FF",  # Bleu
            ItemType.DERAUMERE: "#FF00FF", # Magenta
            ItemType.SIBUR: "#00FFFF",     # Cyan
            ItemType.MENDIANE: "#FFA500",  # Orange
            ItemType.PHIRAS: "#800080",    # Violet
            ItemType.THYSTAME: "#FFD700"   # Or
        }

        # Grille de jeu
        self.grid: Dict[Tuple[int, int], Tile] = {}
        self.cell_size = 25

        self.setup_gui()
        self.setup_network()

        # Démarrer le thread de mise à jour de la vue
        self.view_thread = threading.Thread(target=self.update_view_loop)
        self.view_thread.daemon = True
        self.view_thread.start()

    def setup_gui(self):
        # Frame principale
        main_frame = ttk.Frame(self.root, padding="10")
        main_frame.grid(row=0, column=0, sticky=(tk.W, tk.E, tk.N, tk.S))

        # Grille de jeu (carte complète 32x32)
        self.map_canvas = tk.Canvas(main_frame, width=800, height=800, bg='white')
        self.map_canvas.grid(row=0, column=0, columnspan=3, padx=5, pady=5)

        # Contrôles
        controls_frame = ttk.Frame(main_frame)
        controls_frame.grid(row=1, column=0, columnspan=3, pady=10)

        ttk.Button(controls_frame, text="Tourner à gauche", command=self.turn_left).grid(row=0, column=0, padx=5)
        ttk.Button(controls_frame, text="Avancer", command=self.move_forward).grid(row=0, column=1, padx=5)
        ttk.Button(controls_frame, text="Tourner à droite", command=self.turn_right).grid(row=0, column=2, padx=5)

        # Légende des items
        legend_frame = ttk.LabelFrame(main_frame, text="Légende", padding="5")
        legend_frame.grid(row=2, column=0, columnspan=3, pady=5, sticky=(tk.W, tk.E))

        # Créer un canvas pour la légende
        legend_canvas = tk.Canvas(legend_frame, height=40, bg='white')
        legend_canvas.pack(fill=tk.X, expand=True)

        # Calculer la largeur disponible pour chaque élément
        total_width = legend_canvas.winfo_reqwidth()
        item_width = total_width / len(self.item_colors)

        # Ajouter les éléments de la légende
        for i, (item_type, color) in enumerate(self.item_colors.items()):
            x = (i * (item_width + item_width + 30)/2) + 30
            # Dessiner le cercle de couleur
            legend_canvas.create_oval(
                x - 15, 10, x + 15, 40,
                fill=color, outline='black'
            )
            # Ajouter le texte
            legend_canvas.create_text(
                x, 25,
                text=item_type.value,
                fill='black'
            )

        self.initialize_grid()

    def initialize_grid(self):
        """Initialise la grille de jeu avec des tuiles vides."""
        for i in range(32):
            for j in range(32):
                x1 = j * self.cell_size
                y1 = i * self.cell_size
                x2 = x1 + self.cell_size
                y2 = y1 + self.cell_size

                # Créer la tuile
                canvas_id = self.map_canvas.create_rectangle(
                    x1, y1, x2, y2,
                    fill='#D3D3D3', outline='gray', tags="tile"
                )

                self.grid[(i+1, j+1)] = Tile(items=set(), is_visible=False, canvas_id=canvas_id)

    def wrap_coordinates(self, x: int, y: int) -> Tuple[int, int]:
        """Applique le wrapping toroidal aux coordonnées."""
        return ((x - 1) % 32 + 1, (y - 1) % 32 + 1)

    def get_visible_tiles(self) -> List[Tuple[int, int]]:
        """Retourne les coordonnées des tuiles visibles selon la direction du joueur."""
        x, y = self.player_pos
        tiles = []

        # Niveau 0 (position du joueur)
        tiles.append(self.wrap_coordinates(x, y))

        # Niveau 1
        if self.player_direction == Direction.NORTH:
            tiles.extend([self.wrap_coordinates(x-1, y-1), self.wrap_coordinates(x-1, y), self.wrap_coordinates(x-1, y+1)])
        elif self.player_direction == Direction.EAST:
            tiles.extend([self.wrap_coordinates(x-1, y+1), self.wrap_coordinates(x, y+1), self.wrap_coordinates(x+1, y+1)])
        elif self.player_direction == Direction.SOUTH:
            tiles.extend([self.wrap_coordinates(x+1, y-1), self.wrap_coordinates(x+1, y), self.wrap_coordinates(x+1, y+1)])
        else:  # WEST
            tiles.extend([self.wrap_coordinates(x-1, y-1), self.wrap_coordinates(x, y-1), self.wrap_coordinates(x+1, y-1)])

        # Niveau 2
        if self.player_direction == Direction.NORTH:
            tiles.extend([self.wrap_coordinates(x-2, y-2), self.wrap_coordinates(x-2, y-1), self.wrap_coordinates(x-2, y),
                         self.wrap_coordinates(x-2, y+1), self.wrap_coordinates(x-2, y+2)])
        elif self.player_direction == Direction.EAST:
            tiles.extend([self.wrap_coordinates(x-2, y+2), self.wrap_coordinates(x-1, y+2), self.wrap_coordinates(x, y+2),
                         self.wrap_coordinates(x+1, y+2), self.wrap_coordinates(x+2, y+2)])
        elif self.player_direction == Direction.SOUTH:
            tiles.extend([self.wrap_coordinates(x+2, y-2), self.wrap_coordinates(x+2, y-1), self.wrap_coordinates(x+2, y),
                         self.wrap_coordinates(x+2, y+1), self.wrap_coordinates(x+2, y+2)])
        else:  # WEST
            tiles.extend([self.wrap_coordinates(x-2, y-2), self.wrap_coordinates(x-1, y-2), self.wrap_coordinates(x, y-2),
                         self.wrap_coordinates(x+1, y-2), self.wrap_coordinates(x+2, y-2)])

        # Niveau 3
        if self.player_direction == Direction.NORTH:
            tiles.extend([self.wrap_coordinates(x-3, y-3), self.wrap_coordinates(x-3, y-2), self.wrap_coordinates(x-3, y-1),
                         self.wrap_coordinates(x-3, y), self.wrap_coordinates(x-3, y+1), self.wrap_coordinates(x-3, y+2),
                         self.wrap_coordinates(x-3, y+3)])
        elif self.player_direction == Direction.EAST:
            tiles.extend([self.wrap_coordinates(x-3, y+3), self.wrap_coordinates(x-2, y+3), self.wrap_coordinates(x-1, y+3),
                         self.wrap_coordinates(x, y+3), self.wrap_coordinates(x+1, y+3), self.wrap_coordinates(x+2, y+3),
                         self.wrap_coordinates(x+3, y+3)])
        elif self.player_direction == Direction.SOUTH:
            tiles.extend([self.wrap_coordinates(x+3, y-3), self.wrap_coordinates(x+3, y-2), self.wrap_coordinates(x+3, y-1),
                         self.wrap_coordinates(x+3, y), self.wrap_coordinates(x+3, y+1), self.wrap_coordinates(x+3, y+2),
                         self.wrap_coordinates(x+3, y+3)])
        else:  # WEST
            tiles.extend([self.wrap_coordinates(x-3, y-3), self.wrap_coordinates(x-2, y-3), self.wrap_coordinates(x-1, y-3),
                         self.wrap_coordinates(x, y-3), self.wrap_coordinates(x+1, y-3), self.wrap_coordinates(x+2, y-3),
                         self.wrap_coordinates(x+3, y-3)])

        return tiles

    def update_tile(self, pos: Tuple[int, int], items: Set[str], is_visible: bool):
        """Met à jour une tuile spécifique."""
        if pos not in self.grid:
            return

        tile = self.grid[pos]
        i, j = pos
        x1 = (j-1) * self.cell_size
        y1 = (i-1) * self.cell_size
        x2 = x1 + self.cell_size
        y2 = y1 + self.cell_size

        # Mettre à jour la visibilité
        if tile.is_visible != is_visible:
            tile.is_visible = is_visible
            fill_color = '#F0F0F0' if is_visible else '#D3D3D3'
            self.map_canvas.itemconfig(tile.canvas_id, fill=fill_color)

        # Supprimer les anciens items
        for item_id in tile.item_ids:
            self.map_canvas.delete(item_id)
        tile.item_ids.clear()

        # Ajouter les nouveaux items
        if items:
            for k, item in enumerate(items):
                try:
                    item_type = ItemType(item)
                    color = self.item_colors[item_type]
                    item_x = x1 + (k + 1) * (self.cell_size / (len(items) + 1))
                    item_y = y1 + self.cell_size / 2
                    item_id = self.map_canvas.create_oval(
                        item_x - 5, item_y - 5,
                        item_x + 5, item_y + 5,
                        fill=color, outline='black', tags="item"
                    )
                    tile.item_ids.append(item_id)
                except ValueError:
                    continue

        tile.items = items

    def update_player_position(self):
        """Met à jour la position et l'orientation du joueur."""
        # Supprimer l'ancienne flèche de direction
        self.map_canvas.delete("direction")

        # Mettre à jour la tuile du joueur
        i, j = self.player_pos
        x1 = (j-1) * self.cell_size
        y1 = (i-1) * self.cell_size
        x2 = x1 + self.cell_size
        y2 = y1 + self.cell_size

        # Dessiner la flèche de direction
        if self.player_direction == Direction.NORTH:
            points = [x1 + self.cell_size/2, y1, x1 + self.cell_size/4, y1 + self.cell_size/2, x1 + 3*self.cell_size/4, y1 + self.cell_size/2]
        elif self.player_direction == Direction.EAST:
            points = [x2, y1 + self.cell_size/2, x1 + self.cell_size/2, y1 + self.cell_size/4, x1 + self.cell_size/2, y1 + 3*self.cell_size/4]
        elif self.player_direction == Direction.SOUTH:
            points = [x1 + self.cell_size/2, y2, x1 + self.cell_size/4, y1 + self.cell_size/2, x1 + 3*self.cell_size/4, y1 + self.cell_size/2]
        else:  # WEST
            points = [x1, y1 + self.cell_size/2, x1 + self.cell_size/2, y1 + self.cell_size/4, x1 + self.cell_size/2, y1 + 3*self.cell_size/4]

        self.map_canvas.create_polygon(points, fill='blue', outline='blue', tags="direction")

    def setup_network(self):
        self.socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        try:
            self.socket.connect(('localhost', 8080))
            self.receive_thread = threading.Thread(target=self.receive_messages)
            self.receive_thread.daemon = True
            self.receive_thread.start()
            self.send_command("tde-los-")
        except Exception as e:
            print(f"Erreur de connexion: {e}")

    def send_command(self, command: str):
        if self.socket and not self.waiting_for_response:
            try:
                self.socket.send(f"{command}\n".encode())
                self.last_command = command
                self.waiting_for_response = True
            except Exception as e:
                print(f"Erreur d'envoi: {e}")

    def turn_left(self):
        self.send_command("gauche")

    def turn_right(self):
        self.send_command("droite")

    def move_forward(self):
        self.send_command("avance")

    def update_view_loop(self):
        while self.running:
            if not self.waiting_for_response:
                self.send_command("voir")
            time.sleep(1)

    def receive_messages(self):
        buffer = ""
        while self.running:
            try:
                data = self.socket.recv(1024).decode()
                if not data:
                    break

                buffer += data
                while '\n' in buffer:
                    message, buffer = buffer.split('\n', 1)
                    self.message_queue.put(message)
                    self.process_message(message)
            except Exception as e:
                print(f"Erreur de réception: {e}")
                break

    def process_message(self, message: str):
        self.waiting_for_response = False

        if message == "ok":
            if self.last_command == "gauche":
                self.player_direction = Direction((self.player_direction.value - 1) % 4)
                self.update_player_position()
            elif self.last_command == "droite":
                self.player_direction = Direction((self.player_direction.value + 1) % 4)
                self.update_player_position()
            elif self.last_command == "avance":
                x, y = self.player_pos
                if self.player_direction == Direction.NORTH:
                    self.player_pos = self.wrap_coordinates(x - 1, y)
                elif self.player_direction == Direction.EAST:
                    self.player_pos = self.wrap_coordinates(x, y + 1)
                elif self.player_direction == Direction.SOUTH:
                    self.player_pos = self.wrap_coordinates(x + 1, y)
                else:  # WEST
                    self.player_pos = self.wrap_coordinates(x, y - 1)
                self.update_player_position()
        elif message.startswith("{"):
            try:
                # Traiter la vue du joueur
                items_str = message.strip("{}").split(", ")
                visible_tiles = self.get_visible_tiles()

                # Mettre à jour toutes les tuiles
                for pos in self.grid:
                    self.update_tile(pos, set(), False)

                # Mettre à jour les tuiles visibles
                for i, items in enumerate(items_str):
                    if i < len(visible_tiles):
                        tile = visible_tiles[i]
                        if items:  # Si la tuile n'est pas vide
                            self.update_tile(tile, set(items.split()), True)
                        else:
                            self.update_tile(tile, set(), True)
            except Exception as e:
                print(f"Erreur de traitement de la vue: {e}")

def main():
    root = tk.Tk()
    app = ZappyGUI(root)
    root.mainloop()

if __name__ == "__main__":
    main()
