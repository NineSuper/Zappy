# 🦀 Rust — Bases pratiques et claires

Un condensé des notions clés de Rust expliqué avec simplicité et exemples concrets.

---

## ✅ 1. `Option<T>` vs `Result<T, E>`

| Type          | Usage                              | Exemples                        |
|---------------|------------------------------------|----------------------------------|
| `Option<T>`   | Une valeur présente ou absente     | `Some(42)`, `None`               |
| `Result<T, E>`| Une opération réussie ou échouée   | `Ok("ok")`, `Err("erreur")`      |

```rust
let maybe = Some("Hello");
let result = Ok("World");
```

---

## 🔍 2. `if let` vs `match`

### if let — pour un seul cas
```rust
if let Some(val) = maybe {
    println!("Valeur : {}", val);
}
```

### match (équivalent de switch) — pour tous les cas
```rust
match maybe {
    Some(val) => println!("Valeur : {}", val),
    None => println!("Aucune valeur"),
}
```

---

## ⚠️ 3. `unwrap()` vs `expect()`

### ⚡ Risqué :
```rust
let name = Some("Tom").unwrap(); // Panic si None
```

### ✅ Préférable :
```rust
let name = Some("Tom").expect("Le nom est requis");
```

---

## 🔁 4. `iter()` vs `iter_mut()` vs `into_iter()`

| Méthode        | Description                          | Exemple                        |
|----------------|--------------------------------------|--------------------------------|
| `.iter()`      | Lecture seule (`&T`)                 | `for x in vec.iter()`          |
| `.iter_mut()`  | Lecture/écriture (`&mut T`)          | `for x in vec.iter_mut()`      |
| `.into_iter()` | Prend possession (`T`)               | `for x in vec.into_iter()`     |

---

## ❓ 5. L’opérateur `?` — Propagation d'erreurs

```rust
fn lire_fichier() -> Result<String, std::io::Error> {
    let contenu = std::fs::read_to_string("fichier.txt")?;
    Ok(contenu)
}
```

Le `?` renvoie automatiquement l’erreur si elle existe.

---

## 🔄 6. `.map()` vs `.and_then()` (`Option` / `Result`)

### Avec `Option` :
```rust
Some(2).map(|x| x + 1);             // Some(3)
Some(2).and_then(|x| Some(x * 2));  // Some(4)
```

### Avec `Result` :
```rust
Ok(2).map(|x| x + 1);               // Ok(3)
Ok(2).and_then(|x| Ok(x * 2));      // Ok(4)
```

- `map` : transforme la valeur
- `and_then` : transforme et retourne un `Option` ou `Result`

---

## 🧹 7. `#[derive(Debug)]` + `println!("{:?}", ...)`

```rust
#[derive(Debug)]
struct Player {
    name: String,
}

let p = Player { name: "lmas".to_string() };
println!("{:?}", p); // → Player { name: "lmas" }
```

---

## 📦 8. `Copy` vs `Clone`

### `Copy` : types simples copiés automatiquement

```rust
let x = 5;
let y = x; // OK, Copy
```

### `Clone` : nécessaire pour les types complexes

```rust
let a = String::from("hello");
let b = a.clone(); // a est toujours utilisable
```

---

## 🧠 9. Réutiliser une variable dans d'autres fonctions

Rust empêche la copie implicite de valeurs non triviales (comme `String`) pour éviter les erreurs mémoire. Tu as plusieurs options :

### 1. **Passer une référence** (`&T` ou `&mut T`)
```rust
fn afficher(msg: &String) {
    println!("{}", msg);
}

fn changer(msg: &mut String) {
    msg.push_str(" !");
}

let mut texte = String::from("Salut");
afficher(&texte);
changer(&mut texte);
```

### 2. **Retourner la valeur modifiée**
```rust
fn transformer(mut s: String) -> String {
    s.push_str(" monde");
    s
}

let s = String::from("Hello");
let s = transformer(s);
```

### 3. **Utiliser un `struct` et manipuler ses champs**
```rust
struct Data {
    valeur: String,
}

fn maj(data: &mut Data) {
    data.valeur = "Nouvelle valeur".into();
}

let mut d = Data { valeur: "Ancienne".into() };
maj(&mut d);
```

---

## 🛠 10. Fonctions anonymes (closures)

Les **closures** sont des fonctions anonymes que l'on peut stocker dans des variables. Elles peuvent capturer des variables de l'environnement.

```rust
let add = |a: i32, b: i32| a + b;
println!("{}", add(2, 3)); // 5
```

### Closures avec environnement capturé

```rust
let facteur = 10;
let multiplier = |x: i32| x * facteur;
println!("{}", multiplier(3)); // 30
```

Par défaut :
- elles empruntent les variables (`&facteur`) si possible,
- sinon elles les mutent (`&mut facteur`),
- ou les prennent par valeur (`facteur`) si nécessaire.

### Closures mutables

```rust
let mut compteur = 0;
let mut incr = || {
    compteur += 1;
    println!("compteur = {}", compteur);
};
incr();
incr();
```

---

## ⏳ 11. Lifetimes — durées de vie des références

Les **lifetimes** permettent à Rust de s'assurer qu'aucune référence ne devient invalide (dangling pointer). Le compilateur les infère souvent, mais on peut aussi les expliciter.

### Exemple simple :

```rust
fn plus_long<'a>(a: &'a str, b: &'a str) -> &'a str
{
    if a.len() > b.len()
    {
        a
    }
    else
    {
        b
    }
}
```

- `'a` est une durée de vie générique.
- Ici, le retour vivra au moins aussi longtemps que `a` **et** `b`.

### Sans lifetime explicite (erreur) :

```rust
// erreur : durée de vie du retour inconnue
fn mauvais(a: &str, b: &str) -> &str
{
    if a.len() > b.len() { a } else { b }
}
```

---

## 🧬 12. Traits — comportements génériques

Un **trait** est comme une interface. Il définit un comportement que d'autres types peuvent implémenter.

### Définir un trait

```rust
trait Parler
{
    fn parler(&self);
}
```

### Implémenter un trait

```rust
struct Chien;

impl Parler for Chien
{
    fn parler(&self)
    {
        println!("Wouf !");
    }
}

let rex = Chien;
rex.parler();
```

### Trait générique

```rust
fn faire_parler<T: Parler>(animal: T)
{
    animal.parler();
}
```

### Trait dérivable : `PartialEq`, `Clone`, `Debug`...

```rust
#[derive(Debug, PartialEq, Clone)]
struct Point
{
    x: i32,
    y: i32,
}
```

Ces traits permettent :
- `==` / `!=` (`PartialEq`)
- `clone()` (`Clone`)
- `println!("{:?}", ...)` (`Debug`)
---
