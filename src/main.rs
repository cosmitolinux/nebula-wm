use std::env;
use std::fs;
use std::process::{self, Command, Stdio};

fn ensure_dir(path: &str) {
    if fs::metadata(path).is_err() {
        fs::create_dir_all(path).expect("failed to create directory");
    }
}

fn run(cmd: &str, args: &[&str]) {
    let _ = Command::new(cmd).args(args).status();
}

fn timestamp() -> String {
    let output = Command::new("date")
        .arg("+%Y%m%d_%H%M%S")
        .output()
        .expect("failed to get timestamp");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn main() {
    let home = env::var("HOME").expect("HOME not set");
    let save_dir = format!("{}/Screenshots", home);
    ensure_dir(&save_dir);

    let args: Vec<String> = env::args().collect();

    // no subcommand = init mode (original main.cpp behavior)
    if args.len() < 2 {
        println!("--- n3bu1a shell v1.0 ---");
        run("pkill", &["waybar"]);
        run("pkill", &["swww"]);
        match Command::new("waybar").spawn() {
            Ok(_) => {
                println!("[n3bu1a] firing up waybar...");
                println!("[n3bu1a] we vibin. shell is ready.");
            }
            Err(e) => eprintln!("[n3bu1a] waybar failed to launch: {}", e),
        }
        return;
    }

    // subcommand mode (original n3bu1a-ctl behavior)
    let action = args[1].as_str();
    match action {
        "screenshot-area" => {
            let ts = timestamp();
            let path = format!("{}/n3bu1a_{}.png", save_dir, ts);

            let slurp = Command::new("slurp").output().expect("slurp failed");
            let geometry = String::from_utf8_lossy(&slurp.stdout);
            let geometry = geometry.trim();

            run("grim", &["-g", geometry, &path]);

            Command::new("wl-copy")
                .stdin(Stdio::from(
                    fs::File::open(&path).expect("failed to open screenshot"),
                ))
                .status()
                .expect("wl-copy failed");

            run("notify-send", &["snagged that area for u"]);
        }
        "screenshot-full" => {
            let ts = timestamp();
            let path = format!("{}/n3bu1a_{}.png", save_dir, ts);
            run("grim", &[&path]);
            run("notify-send", &["saved the full screen"]);
        }
        "reload" => {
            run("pkill", &["waybar"]);
            match Command::new("waybar").spawn() {
                Ok(_) => println!("[n3bu1a] waybar reloaded."),
                Err(e) => eprintln!("[n3bu1a] waybar failed to launch: {}", e),
            }
        }
        _ => {
            eprintln!("n3bu1a: {} is not a thing", action);
            process::exit(1);
        }
    }
}
