# libsts

libsts is a Rust library for working with Slay the Spire save and run files.

* [API Documentation](https://docs.rs/libsts/)
* Cargo package: [libsts](https://crates.io/crates/libsts)
* [Mega Crit](https://www.megacrit.com/), the creators of Slay the Spire

---
## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
libsts = "0.1.0"
```

## Basic Usage

```rust
use libsts::{Save, SaveError};
use std::fs;

fn main() {
    // Load the Ironclad save file
    let contents = fs::read_to_string("IRONCLAD.autosave").unwrap();
    
    // Attempt to parse the save file
    if let Ok(mut save) = Save::new(&contents) {
        // Increase the player's hand size and gold
        save.hand_size += 2;
        save.gold += 999;
        
        // Get the base64 string representation of our modified savefile
        if let Ok(modified_save) = save.as_b64() {
            // Attempt to overwrite the current save file
            // with our modified cheaty save file
            fs::write("IRONCLAD.autosave", modified_save);
        }
    }
}
```
## License

This project is licensed under the Apache License, Version 2.0.