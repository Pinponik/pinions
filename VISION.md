# Pinions vision

\_______________

| src/main.rs           |

```rust
use pinions::prelude::*;

struct CounterApp {
    counter: i32,
    buffer: Str<16>,
}

fn main() {
    let app = CounterApp {
        counter: 0,
        buffer: Str::new(),
    };

    let mut win = easy::Window::title("Counter");

    win.run(app, |app| {
        (
            easy::Button::label(|_|"+").on_click(|state| state.counter += 1),
            
            easy::Label::label(|state| {
                let mut txt = Str::new();
                let _ = core::fmt::write(&mut txt, format_args!("{}", state.counter));
                txt
            }),
            
            easy::Button::label(|_| "-").on_click(|state| state.counter -= 1),
        )
    });
}
```

\_______________

| Cargo.toml            |

```toml
[package]
name = "counter"
version = "0.1.0"
authors = [""]
edition = "2021"

[dependencies]
pinions = "*"
```
