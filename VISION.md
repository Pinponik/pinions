# Pinions vision

\_______________

| src/main.rs           |

```rust
use pinions::prelude::*;

fn main() {
    type App = i32;
    impl PinionsApp for App {}
    let app = App::new();
    let plus = easy::Button::label(|_| "+".to_string()).on_click(|&mut app| app += 1);
    let display = easy::Label::label(|app| app.to_string());
    let minus = easy::Button::label(|_| "-".to_string()).on_click(|&mut app| app -= 1);

    let win = easy::Window::label(|_| "Counter");
    win.widgets().push(vec![plus, display, minus]);
    win.run(app);    
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
