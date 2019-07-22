use libsts::save::Save;

use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    // In the future we can get all of this from user input - for now,
    // we just stick to some common defaults
    let base_directory = "C:\\Program Files (x86)\\Steam\\steamapps\\common\\SlayTheSpire\\saves\\";
    let savefile_names = vec![
        "IRONCLAD.autosave",
        "DEFECT.autosave",
        "THE_SILENT.autosave",
    ];

    let save_path = Path::new(base_directory);

    if !save_path.exists() {
        panic!("Save directory not found");
    }

    // Construct a vector containing the save files we're on the lookout for
    let mut savefiles = Vec::new();

    for savefile_name in savefile_names {
        let mut tmp_save_path = PathBuf::from(&save_path);
        tmp_save_path.push(savefile_name);

        savefiles.push(tmp_save_path);
    }

    // Check whether any savefiles currently exist
    for savefile_path in savefiles.iter() {
        if savefile_path.exists() {
            let contents = fs::read_to_string(savefile_path);

            if let Ok(contents) = contents {
                if let Ok(mut save) = Save::new(&contents) {
                    // Increase player hand size by 2
                    save.hand_size += 2;
                    // Add 999 gold
                    save.gold += 999;

                    if let Ok(modified_save) = save.as_b64() {
                        println!("{}", modified_save);
                    }
                }
            }
        }
    }
}
