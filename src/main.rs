use anyhow::{Context, Result};
use regex::Regex;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use hemtt_pbo::create;

lazy_static::lazy_static! {
    static ref MISSION_SQM: Regex = Regex::new(r#"class Mission\s*\{\s*"#).unwrap();
}

fn validate_mission(dir: &Path) -> Result<()> {
    let mut has_mission_sqm = false;
    
    for entry in WalkDir::new(dir) {
        let entry = entry?;
        if entry.file_name() == "mission.sqm" {
            let content = std::fs::read_to_string(entry.path())?;
            if MISSION_SQM.is_match(&content) {
                has_mission_sqm = true;
            }
        }
    }
    
    if !has_mission_sqm {
        anyhow::bail!("Invalid mission structure: Missing valid mission.sqm");
    }
    
    Ok(())
}

fn pack_mission(input_dir: &Path, output_path: &Path) -> Result<()> {
    let pbo = create(input_dir, hemtt_pbo::Prefix::Keep, true)?;
    std::fs::write(output_path, pbo)?;
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        anyhow::bail!("Usage: arma3-packer <input_dir> <output_path>");
    }
    
    let input_dir = PathBuf::from(&args[1]);
    let output_path = PathBuf::from(&args[2]);
    
    validate_mission(&input_dir)
        .context("Mission validation failed")?;
    pack_mission(&input_dir, &output_path)
        .context("Packing failed")?;
    
    Ok(())
}
