#![warn(clippy::all, clippy::nursery)]

use std::{
    fs::File,
    path::PathBuf,
};

use walkdir::WalkDir;

fn main() {
    let source = PathBuf::from(std::env::args().nth(1).expect("no source directory provided"));
    let dest = PathBuf::from(std::env::args().nth(2).expect("no destination directory provided"));

    let mission_roots = find_mission_roots(&source);

    for mission_root in mission_roots {
        let mission_name = mission_root.file_name()
            .expect("Failed to get mission name")
            .to_str()
            .expect("Mission name contains invalid characters");
        
        println!("Packing mission: {}", mission_name);

        let mut pbo = hemtt_pbo::WritablePbo::<File>::new();

        // Walk through all files in the mission directory
        for entry in WalkDir::new(&mission_root)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file()) 
        {
            let file_path = entry.path();
            let relative_path = file_path.strip_prefix(&mission_root)
                .expect("Failed to strip mission root prefix");
            
            // Convert path separators to Arma's preferred format
            let pbo_path = relative_path.to_str()
                .expect("Invalid UTF-8 in file path")
                .replace(std::path::MAIN_SEPARATOR, "\\");

            pbo.add_file(pbo_path, File::open(file_path).unwrap())
                .expect("Failed to add file to PBO");
        }

        // Create output PBO file
        let pbo_filename = format!("{}.pbo", mission_name);
        let pbo_path = dest.join(pbo_filename);
        let mut pbo_file = File::create(&pbo_path)
            .expect("Failed to create PBO file");
        
        pbo.write(&mut pbo_file, false)
            .expect("Failed to write PBO");
        
        println!("Successfully created: {}", pbo_path.display());
    }
}

fn find_mission_roots(source: &PathBuf) -> Vec<PathBuf> {
    WalkDir::new(source)
        .into_iter()
        .filter_map(|entry| {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => return None,
            };

            // Look for mission.sqm files
            if entry.file_type().is_file() && entry.file_name() == "mission.sqm" {
                entry.path().parent().map(|p| p.to_path_buf())
            } else {
                None
            }
        })
        .collect()
}