# Background Game

Desktop overlay hra s transparentním pozadím pro krátké herní mikro-session během práce.

## O projektu

**Background Game** je lehká 2D hra běžící jako overlay nad pracovní plochou. Projekt staví na Rustu a Bevy, aby bylo možné postupně rozšiřovat gameplay logiku, UI i Windows overlay vrstvu bez velkého tření.

Aktuální verze obsahuje transparentní always-on-top okno, ovládací UI panel, FPS panel, hráčovu loď a první implementovaný enemy typ `DroneSwarm`.

## Ovládání

### Klávesnice

- `W` pohyb vpřed ve směru natočení lodi
- `S` pohyb vzad
- `A` rotace doleva
- `D` rotace doprava
- `C` dash vpřed s krátkým cooldownem
- `Escape` ukončí hru

### UI panel

- `2D Arcade`, `2D Side-Scrolling`, `Idle Overlay` přepínají herní režim
- `New Game` vrátí běh hry do pauzy
- `Start` spustí gameplay; po spuštění se změní na `Pause`
- `Pause` zastaví běh hry
- `X` zavře aplikaci

## Herní režimy

- **2D Arcade** je aktuálně hlavní režim. Zobrazuje hvězdné pozadí, hráče a po stisknutí `Start` aktivuje enemy vlnu.
- **2D Side-Scrolling** je připravený v UI, ale gameplay pro něj zatím není implementovaný.
- **Idle Overlay** je připravený v UI pro budoucí background režim.

## Enemy systém

Enemy systém je rozdělený do samostatného modulu a aktuálně obsahuje první reálně fungující typ `DroneSwarm`. Ostatní enemy typy jsou zatím jen připravené v datech pro další iterace.

### Drone Swarm

- spawnne se jako jedna vlna o `3` až `6` dronech
- přilétá těsně mimo viditelnou plochu
- používá 3 sprite varianty:
  - `drone-swarm-left.png`
  - `drone-swarm-center.png`
  - `drone-swarm-right.png`
- každá entita ve swarmu má základní velikost `30x30 px`, varianty se od ní mírně škálují
- všechny drony automaticky letí směrem k hráči
- swarm používá seek, cohesion, alignment a separaci, aby držel roj a co nejméně se srážel
- pravá varianta má permanentní rotaci po směru hodinových ručiček

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
- **Windowing:** `bevy_winit`
- **Platform handles:** `raw-window-handle`
- **Windows API bindings:** `windows-sys`
- **Architecture:** ECS přes Bevy pluginy, komponenty a systémy

Aktuální Cargo dependencies:

```toml
[dependencies]
bevy = { version = "0.14", default-features = true }
bevy_winit = "0.14"
raw-window-handle = "0.6"
windows-sys = { version = "0.52", features = ["Win32_Foundation", "Win32_UI_WindowsAndMessaging"] }
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
  game/            # Gameplay logika, hráč, pozadí, enemies
  ui/              # Bevy UI panel a UI stav hry
  overlay/         # Window setup a Win32 overlay úpravy
  plugins/         # Hlavní plugin wiring

assets/
  sprites/         # Sprite assety včetně Drone Swarm variant
```

## Poznámky k overlay režimu

- okno je borderless, transparentní a always-on-top
- transparentní pozadí používá Win32 layered-window color key
- overlay nemusí fungovat stejně nad aplikacemi v exclusive fullscreen režimu
- projekt cílí primárně na Windows

## Roadmapa

- [x] Inicializace Rust + Bevy projektu
- [x] Transparentní overlay okno pro Windows
- [x] Ovládací UI panel a FPS panel
- [x] Hráčova loď s rotací, akcelerací a dashem
- [x] Základní enemy systém
- [x] První implementovaný enemy `DroneSwarm`
- [ ] Kolize, poškození a health systém
- [ ] Další enemy AI a odlišná chování
- [ ] Více vln, spawn pravidla a progres obtížnosti
- [ ] Audio, efekty a finální assety

## Licence

Licence zatím není specifikovaná.
