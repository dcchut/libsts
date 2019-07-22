mod common;
mod run;
mod save;

use crate::run::Run;
use crate::save::Save;

use std::fs;
use std::path::{Path, PathBuf};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let f =
        r"C:\Program Files (x86)\Steam\steamapps\common\SlayTheSpire\runs\IRONCLAD\1563755407.run";

    let run_file = fs::read_to_string(Path::new(f));

    if let Ok(run_file) = run_file {
        if let Ok(_r) = Run::new(&run_file) {
            //
        }

        // use serde_json::Value;
        //let v : Value = serde_json::from_str(&run_file).unwrap();
        //println!("{}", serde_json::to_string_pretty(&v).unwrap());
        // decode with serde
    }

    // In the future we can get all of this from user input - for now,
    // we just stick to some common defaults
    let base_directory = "C:\\Program Files (x86)\\Steam\\steamapps\\common\\SlayTheSpire\\saves\\";
    let update_delay = Duration::from_secs(12);
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

    loop {
        // Check whether any of our savefiles currently exist
        for savefile in savefiles.iter() {
            if savefile.exists() {
                //..println!("Reading {:?}...", savefile);

                let contents = fs::read_to_string(savefile);

                if let Ok(contents) = contents {
                    let savefile = Save::new(&contents);

                    if let Ok(_savefile) = savefile {
                        //dbg!(savefile);
                        //println!("Success!");
                        //println!("Successfully parsed!");
                        //savefile.hand_size = 7;

                        //if let Ok(raw_savefile) = savefile.as_b64() {
                        //    println!("{}", raw_savefile);
                        //}
                    } else {
                        dbg!(savefile.err());
                    }
                }
            }
        }

        sleep(update_delay);
    }
}
