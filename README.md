# WASM Pachinko Game

A browser pachinko game with physics in Rust compiled to WebAssembly.

## Prerequisites

- Rust (stable)
- `wasm-pack` (`cargo install wasm-pack`)

## Build

```bash
wasm-pack build --target web
```

This generates `pkg/`.

## Run locally

From this folder:

```bash
python3 -m http.server 8080
```

Open:

- http://localhost:8080

## Controls

- **Drop Ball**: drop from center
- **Auto**: continuously drops balls
- **Reset**: clear balls, bins, and score
- Click the board to drop at your cursor X position
