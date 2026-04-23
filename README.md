# Background Game

Lehká 2D desktop hra v Rustu a Bevy pro krátké arcade session.

## O projektu

**Background Game** je malá top-down akční hra postavená nad ECS architekturou v Bevy. Projekt je rozdělený na samostatné moduly pro gameplay, enemy logiku, UI a plugin wiring, aby se dal postupně rozšiřovat bez velkého tření.

Aktuální verze obsahuje hráčovu loď s pohybem a dash mechanikou, střelbu levým tlačítkem, health systém pro hráče i enemy, UI panel, FPS panel a první fungující enemy typ `DroneSwarm`.

## Ovládání

### Klávesnice

- `W` pohyb vpřed ve směru natočení lodi
- `S` pohyb vzad
- `A` rotace doleva
- `D` rotace doprava
- `C` dash vpřed s krátkým cooldownem
- `Escape` ukončí hru

### Myš

- `Levé tlačítko` střelba ve směru natočení hráče

### UI panel

- `2D Arcade`, `2D Side-Scrolling` přepínají herní režim
- `New Game` vrátí běh hry do pauzy a resetuje combat stav
- `Start` spustí gameplay; po spuštění se změní na `Pause`
- `Pause` zastaví běh hry
- `X` zavře aplikaci

## Herní režimy

- **2D Arcade** je aktuálně hlavní režim. Zobrazuje hvězdné pozadí, hráče, enemy vlnu a combat UI.
- **2D Side-Scrolling** je připravený v UI, ale gameplay pro něj zatím není implementovaný.

## Combat

- hráč střílí automaticky při držení levého tlačítka
- hráč má health bar vlevo dole
- každý dron má vlastní health bar pod entitou
- zásah střely ubírá enemy část života
- kontakt dronu s hráčem ubírá život hráči

## Enemy systém

Enemy systém je rozdělený do samostatného modulu a aktuálně obsahuje první reálně fungující typ `DroneSwarm`. Ostatní enemy typy jsou zatím připravené v datech pro další iterace.

### Drone Swarm

- spawnne se jako jedna vlna o `3` až `6` dronech
- přilétá těsně mimo viditelnou plochu
- používá 3 sprite varianty:
  - `drone-swarm-left.png`
  - `drone-swarm-center.png`
  - `drone-swarm-right.png`
- všechny drony automaticky letí směrem k hráči
- swarm používá seek, cohesion, alignment a separaci, aby držel roj a co nejméně se srážel

### Připravené další typy

- `KamikazeDrone`
- `ShieldCarrier`
- `TurretDrone`
- `PhaseJumper`
- `GravityDrone`
- `Splitter`
- `FlameChaser`
- `SniperEye`
- `OverlordCore`

## Tech Stack

- **Language:** Rust 2021
- **Game engine:** Bevy `0.14`
- **Architecture:** ECS přes Bevy pluginy, komponenty a systémy

Aktuální Cargo dependencies:

```toml
[dependencies]
bevy = { version = "0.14", default-features = true }
```

## Build & Run

```powershell
cargo build
cargo run
cargo check
```

## Struktura projektu

```text
src/
  main.rs          # App entrypoint
  game/            # Gameplay logika, hráč, pozadí, combat, enemies
  ui/              # Bevy UI panel a UI stav hry
  plugins/         # Hlavní plugin wiring

assets/
  sprites/         # Sprite assety včetně Drone Swarm variant
```

## Roadmapa

- [x] Inicializace Rust + Bevy projektu
- [x] Ovládací UI panel a FPS panel
- [x] Hráčova loď s rotací, akcelerací a dashem
- [x] Základní enemy systém
- [x] První implementovaný enemy `DroneSwarm`
- [x] Kolize, poškození a health systém
- [ ] Další enemy AI a odlišná chování
- [ ] Více vln, spawn pravidla a progres obtížnosti
- [ ] Audio, efekty a finální assety

## Licence

Licence zatím není specifikovaná.
