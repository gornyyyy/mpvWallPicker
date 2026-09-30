use anyhow::Context;
use anyhow::Result;
use std::path::Path;
use std::process::Command;

pub fn stop() {
    let status = Command::new("pkill")
        .args(["-x", "mpvpaper"])
        .status();

    match status {
        Ok(s) if s.success() => println!("старый mpvpaper остановлен"),
        Ok(s) if s.code() == Some(1) => {}
        Ok(s) => eprintln!("pkill завершился с кодом {:?}", s.code()),
        Err(e) => eprintln!("Не удалось запустить pkill: {e}"),
    }
}

pub fn start(file: &Path) -> Result<()> {
    let inner = format!(
        "mpvpaper -o 'no-audio --loop-playlist --hwdec=vaapi --vo=gpu --panscan=1.0' ALL '{}' > /dev/null 2>&1",
        file.display()
    );

    Command::new("sh")
        .args(["-c", &inner])
        .spawn()
        .context("Не удалось запустить mpvpaper через sh -c")?;

    Ok(())
}