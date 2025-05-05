use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_valid_mission() -> Result<(), Box<dyn std::error::Error>> {
    let dir = TempDir::new()?;
    std::fs::create_dir(dir.path().join("mission"))?;
    std::fs::write(
        dir.path().join("mission/mission.sqm"),
        r#"class Mission {
            class Entities {};
        }"#,
    )?;

    let mut cmd = Command::cargo_bin("pbo-packer")?;
    cmd.arg(dir.path().join("mission"))
        .arg(dir.path().join("mission.pbo"));
    cmd.assert().success();
    
    Ok(())
}
