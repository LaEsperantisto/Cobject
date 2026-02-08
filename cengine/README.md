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
```

---

## Project Structure

```text
.
├── src/
│   ├── main.rs            # Application example
│   ├── cwindow.rs         # Window, input, framebuffer, buttons
│   ├── cinput.rs          # Cached input state (mouse, keyboard)
│   ├── cdrawable.rs       # Drawable trait
│   ├── cobject.rs         # Rectangular drawable object
│   ├── cpoint.rs          # 2D point abstraction
│   ├── cbutton.rs         # Button trait
│   ├── cbasic_button.rs   # Simple button implementation
│   ├── ctitle_bar.rs      # Custom title bar UI
│   ├── ckey.rs            # Keyboard abstraction
│   └── cmouse.rs          # Mouse abstraction
├── target/
│   └── debug/
│       └── cobject        # The example's executable
```

---

## Main Game Loop

The engine uses a classic game loop:

```rust
fn main() {
    while window.is_open() {
        window.poll_input(); // Cache mouse & keyboard input
        window.update(); // Handle UI logic & events
        window.clear(); // Clear framebuffer
        window.draw(&scene); // Render objects
        window.draw_window(); // Display frame
    }
}
```

---

## Core Concepts

### `CDrawable`

Anything that can be rendered implements:

```rust
trait CDrawable {
    fn draw(&self, pixels: &mut Vec<u32>, width: usize, height: usize);
}
```

---

### `CObject`

A simple rectangular drawable used for:

- UI backgrounds

- Buttons

- Hitboxes

```rust
CObject::new(x, y, width, height, color)
```

---

### `Buttons`

Buttons are logic-only:

- They define a hitbox
- They execute an action on click
- They do NOT draw themselves

```rust
CBasicButton::new(| | {
// action
}, hitbox)
```

---

### Input Handling

Input is cached once per frame using CInput:

- Mouse position
- Mouse button state

This avoids querying the OS multiple times per frame.

---

### Title Bar

The engine includes a custom title bar with:

* Background rendering
* A thick `❌` close button
* Safe shutdown (no process::exit)

The title bar registers its button during initialization:

```rust
title_bar.init( & mut window);
```

---

### Window Closing (Safe)

UI elements do not directly close the window.

Instead:

* Buttons emit a close request
* `CWindow` decides when to shut down

This avoids lifetime issues and keeps the architecture clean.

```rust
CLOSE_REQUESTED.store(true, Ordering::Relaxed);
```

---

### Running the Project

```bash
cargo run
```

---

### Design Goals

* Minimal dependencies
* Clear ownership rules
* No Rc<RefCell<...>>
* Easy to read & extend
* Educational first, performance second

### Images

Images have a custom file extension: `foo.imace`. They can be opened using CImage::new


---

### Roadmap

Planned features:

* Text rendering (bitmap font)
* Hover & pressed button states
* Draggable windows
* UI theming
* Simple scene system
* Event queue

---

### License

MIT License — do whatever you want, just don’t blame the engine!

---

### Credits

Built with ❤️ using Rust and minifb.

Inspired by classic software-rendered UI systems and early game engines.