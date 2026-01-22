# Cobject Game Engine

A lightweight, experimental **UI & 2D game engine** built in **Rust** on top of  
[`minifb`](https://crates.io/crates/minifb).

This project is designed as a learning-oriented engine with:

- A simple framebuffer renderer
- Basic UI elements (buttons, title bar)
- Clean input handling
- A clear game loop structure

> ⚠️ This is a **work in progress** and not intended for production use (yet).

---

## Features

- Software rendering (pixel buffer)
- Custom window title bar
- Clickable close button
- Mouse input handling
- Keyboard input abstraction
- Extensible UI system
- Clean update / draw separation

---

## Dependencies

```toml
[dependencies]
minifb = "0.25"
