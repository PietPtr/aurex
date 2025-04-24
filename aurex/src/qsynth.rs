use std::{thread, time::Duration};

pub fn ensure_qsynth() -> Result<(), Box<dyn std::error::Error>> {
    let output = std::process::Command::new("pgrep")
        .arg("-f")
        .arg("/usr/bin/qsynth")
        .output()?;

    if !output.status.success() {
        println!("qsynth is not running, starting it.");
        let _ = std::process::Command::new("/usr/bin/qsynth").spawn()?;
        thread::sleep(Duration::from_secs(3));
    }

    Ok(())
}
