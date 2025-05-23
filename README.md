# 🧌 Zappy

## 📋 Description

**Zappy** est un jeu en réseau dans lequel les joueurs contrôlent une population sur un monde plat et sans relief, appelé **Trantor**.

Le plateau de jeu représente la totalité de la surface de Trantor, comme une carte.

Un **Trantorians** (joueur) sortant par la droite réapparaît à gauche, et vice-versa.

### 🌍 Univers

Trantor regorge de ressources naturelles, à la fois **alimentaires** et **minières**.

Les **Trantorians** peuvent se déplacer pour explorer, collecter de la nourriture et découvrir divers types de pierres.

- **Nourriture** : nécessaire pour survivre.
- **Pierres** : il en existe six types — `linemate`, `deraumere`, `sibur`, `mendiane`, `phiras`, et `thystame`.

Ces ressources sont générées de manière **aléatoire**, selon des règles définies, par le **serveur**.

Les habitants de Trantor ont deux objectifs majeurs :

- Se nourrir pour survivre
- Chercher, collecter et utiliser les pierres pour créer des **totems** et réaliser des **rituels d'élévation**, permettant de monter en niveau

---

## 🎯 Objectif

- Le jeu se joue en **équipes**
- Une partie se termine lorsqu’une équipe parvient à faire monter **6 de ses joueurs au niveau maximum**

---

## 💿 Installation des dépendences

🐧 Linux & MacOS 🍎:

    make install

✅ Vérification:

    rustc --version
    cargo --version


## 🗄️ Serveur

```
make ou make server
```

```
./bin/server -p <port> -x <width> -y <height> -n <team> [<team>] [<team>] ... -c <nb> -t <t>

    -p port number
    -x world width
    -y world height
    -n team\_name\_1 team\_name\_2 ...
    -c number of clients authorized at the beginning of the game
    -t time unit divider (the greater t is, the faster the game will go)
```

## 🎮 Client

```
make ou make client
```

```
./bin/client -n <team> -p <port> [-h <hostname>]

    -n team\_name
    -p port
    -h name of the host, by default it'll be localhost
```
