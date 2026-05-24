# Pachinko Game on GitHub

Welcome to the **Pachinko Game** repository.

## What this is
A browser-based Pachinko game built with Rust + WebAssembly.

## Repository layout
- `src/` – Rust game logic
- `pkg/` – generated wasm-bindgen package
- `demo/` – static site for GitHub Pages deployment
- `docs/` – project docs

## Quick start
```bash
wasm-pack build --target web
python3 -m http.server 8080
```
Then open `http://localhost:8080`.

## GitHub Pages demo
After Pages is enabled, the demo is served from the `demo/` directory.
