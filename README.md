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
libsts = "0.3"
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
        if let Ok(modified_save) = save.to_b64_string() {
            // Attempt to overwrite the current save file
            // with our modified cheaty save file
            fs::write("IRONCLAD.autosave", modified_save); 
        }
        
        // Or if you're using the BETA branch of STS:
        if let Ok(modified_save) = save.to_string() {
            // ...
            fs::write("IRONCLAD.autosaveBETA", modified_save);
        }
    }
}
```
### License
Licensed under either of
 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.