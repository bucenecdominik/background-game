# Background Game

Desktop overlay hra s transparentním pozadím pro krátké herní „mikro-session“ během čekání na dokončení práce agentů.

## O projektu
**Background Game** je lehká desktopová hra běžící jako overlay nad pracovní plochou. Není to browserová hra — cíleně staví na Rustu a Bevy, aby byla rychlá, modulární a dlouhodobě udržitelná.

Projekt je určený pro vývojáře a kreativce, kteří chtějí mít při práci puštěnou nenáročnou hru na pozadí (IDE, browser i nástroje pro agenty zůstávají viditelné).

## K čemu projekt slouží
- Zábavný „idle / micro gameplay“ zážitek během čekání.
- Overlay režim nad běžným workflow bez výrazného rušení.
- Experimentální platforma pro kombinaci ECS gameplay logiky a Win32 integrace.
- Základ pro postupné rozšiřování mechanik, UI i pluginů.

## 🧠 Tech Stack

### 🦀 Core
- **Language:** Rust
- **Game Engine:** Bevy
- **Architecture:** ECS (Entity Component System)

### 🪟 Window & Overlay Layer
- **Windowing (internal):** winit (přes Bevy)
- **Windows API bindings:** `windows` crate
- **Overlay features:**
  - transparentní okno (alpha background)
  - borderless okno
  - always-on-top režim
  - volitelný click-through (mouse passthrough)
  - no-focus režim (hra nekrade focus z IDE)

### 🎮 Rendering & Game Layer
- **Renderer:** wgpu (přes Bevy)
- **Graphics:** 2D (sprite-based)
- **UI:** Bevy UI systém
- **Camera:** Orthographic (2D overlay)

## 📦 Dependencies (Cargo)
```toml
[dependencies]
bevy = "0.13"         # game engine
windows = "0.56"      # Win32 API (overlay tweaks)
```

Volitelné (podle potřeby):

```toml
bevy_tweening = "0.10"   # animace
bevy_kira_audio = "0.18" # audio
rand = "0.8"             # náhoda
```

## ⚙️ Build & Tooling
- **Package manager:** Cargo
- **Build / Run:** `cargo build`, `cargo run`
- **Hot reload (assets):** Bevy asset systém
- **Formatting:** rustfmt
- **Linting:** clippy

## 🧩 Platform Target
- **OS:** Windows
- **Graphics API:** DirectX 12 / Vulkan (přes wgpu)
- **Window type:** Layered / transparent desktop overlay

## 🧠 Key Features
- Běh jako desktop overlay hra.
- Plně transparentní pozadí.
- Neblokuje workflow (agenti, IDE, browser zůstávají viditelné).
- Nízká zátěž CPU/GPU.
- Design pro idle/micro gameplay smyčky.

## 🏗️ Project Structure
```text
src/
 ├── main.rs          # app entrypoint
 ├── game/            # gameplay logika (systems, components)
 ├── ui/              # UI systémy
 ├── overlay/         # window + Win32 úpravy
 └── plugins/         # Bevy pluginy

assets/
 ├── sprites/
 ├── audio/
 └── fonts/
```

## 🚀 Dev Philosophy
- Minimal friction (rychlá iterace).
- „Always running“ background experience.
- Design pro vibe coding + AI-assisted development.
- Striktní separace zodpovědností:
  - rendering (Bevy)
  - OS integrace (`windows` crate)
  - gameplay logika (ECS systems)

## ⚠️ Poznámky
- Transparentní okno není automaticky click-through — řeší se přes Win32.
- Overlay nemusí fungovat nad aplikacemi v exclusive fullscreen režimu.
- Výkon je laděný pro background usage (nižší FPS může být akceptovatelné).

## Roadmapa
- [ ] Inicializace Rust + Bevy projektu
- [ ] Základní transparentní overlay okno pro Windows
- [ ] První hratelná idle mechanika
- [ ] Nízkopříkonový update loop a optimalizace
- [ ] Rozšíření UI a plugin architektury

## Licence
Prozatím není specifikována. Doporučeno doplnit před veřejným vydáním.

## Autor
Doplňte jméno/autorský tým projektu.
