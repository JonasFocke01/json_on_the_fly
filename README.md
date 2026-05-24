# Json-on-the-fly

Json-on-the-fly takes care of the boilerplate when you just want a quick way of saving a struct to a `.json` file. \
To be even quicker, a `derive` macro is provided to make every struct capable of saving and restoring. \

Json-on-the-fly is intended to make prototyping faster, not as a replacement for serde or other serializers. \
No special `serde` syntax is supported, although it might work in your case. \

## Examples

Add this to your `Cargo.toml`:

```toml
serde = { version = "1.0", features = ["serde_derive"] }
json-on-the-fly = "0.1"
```

### Derive

```rust
use json_on_the_fly::JsonOnTheFly;
use serde::{Deserialize, Serialize};

#[derive(JsonOnTheFly, Serialize, Deserialize, Default, Clone, Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

fn main() {
    let color = Color {
        red: 0,
        green: 0,
        blue: 255,
    };

    color.write().unwrap(); // Writing the file does not require any more boilerplate. We even take care of creating it, if it does not exist

    Color::backup_db_file().unwrap(); // Create a backup of the file associated with the struct

    let new_color = Color::load().unwrap(); // Initialize the struct from the .json file. The errors provided give more information about whether it was created beforehand.

    println!("{:?}", new_color);
}
```

### Manual impl

The manual implementation for your struct has the advantage, that you can define, where the .json file is stored and what it is called. \
Only the function returning that name has to be implemented.

```rust
use json_on_the_fly::JsonOnTheFly;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl JsonStore for Color {
    fn db_file_path() -> std::path::PathBuf {
        let mut path = std::env::current_dir().unwrap();
        path.push("MyColor");
        path.set_extension("my_json");
        path
    }
}

fn main() {
    let color = Color {
        red: 0,
        green: 0,
        blue: 255,
    };

    color.write().unwrap();

    Color::backup_db_file().unwrap();

    let new_color = Color::load().unwrap();

    println!("{:?}", new_color);
}
```
