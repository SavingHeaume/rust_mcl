use clap::{arg, Command};
use download::{get, Download, LibaryAllowed};
use model::version::Version;
use parse::Parse;
use std::path::Path;

fn cli() -> Command {
    Command::new("rmcl")
        .about("A Minecraft launcher written in Rust")
        .version("0.1.0")
        .author("meng")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("search")
                .about("Search game")
                .arg(arg!([VERSION] "game version"))
                .arg(
                    arg!(-t --type <TYPE> "game type")
                        .value_parser(["release", "snapshot", "old_bate", "old_alpha"])
                        .require_equals(true)
                        .default_value("release")
                        .default_missing_value("release"),
                ),
        )
        .subcommand(
            Command::new("download")
                .about("Download game")
                .arg(arg!(<VERSION> "game version"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("launch")
                .about("Launch game")
                .arg(arg!(<VERSION> "game version"))
                .arg_required_else_help(true),
        )
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("search", sub_matches)) => search(sub_matches),
        Some(("download", sub_matches)) => download(sub_matches),
        Some(("launch", sub_matches)) => launch(sub_matches),
        _ => unreachable!(),
    }
}

fn get_version_manifest() -> model::version_manifest::VersionManifest {
    get("https://launchermeta.mojang.com/mc/game/version_manifest.json")
        .expect("Failed to send GET request")
        .json::<model::version_manifest::VersionManifest>()
        .expect("Failed to parse JSON")
}

fn search(sub_matches: &clap::ArgMatches) {
    let version = sub_matches.get_one::<String>("VERSION");
    let type_ = sub_matches.get_one::<String>("type").unwrap();
    let versions = get_version_manifest().versions;

    let versions = versions.iter().filter(|v| {
        (if version.is_some() {
            v.id.contains(version.unwrap())
        } else {
            true
        }) && v.type_.eq(type_)
    });

    for version in versions {
        println!("Version: {}, {}", version.id, version.type_);
    }
}

fn download(sub_matches: &clap::ArgMatches) {
    // 获取游戏路径
    let game_dir = std::env::current_dir().unwrap().join(".minecraft");

    let version = sub_matches.get_one::<String>("VERSION").unwrap();
    let versions = get_version_manifest().versions;

    if let Some(version) = versions.iter().find(|v| v.id.eq(version)) {
        version.download(&game_dir).unwrap_or_else(|err| {
            eprintln!("download error: {}", err);
        });
    } else {
        eprintln!("Version: {} not found", version);
    }
}

fn extract_jar(jar_path: &Path, dir: &Path) {
    // 使用zip读取jar文件
    let mut archive = zip::ZipArchive::new(std::fs::File::open(jar_path).unwrap()).unwrap();

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i).unwrap();
        if entry.is_file() && !entry.name().contains("META_TNF") {
            let mut name = entry.name();
            if name.contains("/") {
                name = &name[entry.name().rfind('/').unwrap() + 1..];
            }

            let path = dir.join(name);
            if path.exists() {
                std::fs::remove_file(&path).unwrap();
            }

            let mut file = std::fs::File::create(&path).unwrap();
            std::io::copy(&mut entry, &mut file).unwrap();
        }
    }
}

fn launch(sub_matches: &clap::ArgMatches) {
    let game_dir = std::env::current_dir().unwrap().join(".minecraft");
    let libraries_dir = game_dir.join("libraries");
    let assets_dir = game_dir.join("assets");

    let version = sub_matches.get_one::<String>("VERSION").unwrap();
    let version_dir = game_dir.join("versions").join(version);
    let natives_dir = version_dir.join("natives");
    let config_path = version_dir.join(format!("{}.json", version));
    let jar_path = version_dir.join(format!("{}.jar", version));

    if !jar_path.exists() || !config_path.exists() {
        eprintln!("version: {} net found", version);
        return;
    }

    let version = &Version::parse(&std::fs::read_to_string(&config_path).unwrap()).unwrap();
    if !natives_dir.exists() {
        std::fs::create_dir_all(&natives_dir).unwrap();
    }

    for library in &version.libraries {
        if library.allowed() && library.name.contains("natives") {
            extract_jar(
                &libraries_dir.join(&library.downloads.artifact.path),
                &natives_dir,
            );
        }
    }

    let classpath = format!(
        "{}{}",
        &version
            .libraries
            .iter()
            .map(|library| {
                format!(
                    "{}{}",
                    libraries_dir
                        .join(&library.downloads.artifact.path)
                        .display(),
                    if cfg!(windows) { ";" } else { ":" }
                )
            })
            .collect::<String>(),
        jar_path.display()
    );

    std::process::Command::new("jave")
        .current_dir(&game_dir)
        .arg(format!("-Djava.library.path={}", natives_dir.display()))
        .arg("-cp")
        .arg(classpath)
        .arg(&version.main_class)
        .arg("--username")
        .arg("meng")
        .arg(&version.id)
        .arg("--gameDir")
        .arg(game_dir)
        .arg("--assetDir")
        .arg(assets_dir)
        .arg("--assetIndex")
        .arg(&version.asset_index.id)
        .arg("--accessToken")
        .arg("0")
        .arg("--versionType")
        .arg("RMCL 0.1.0")
        .status()
        .unwrap();
}
