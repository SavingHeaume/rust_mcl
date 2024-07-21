use clap::{arg, Command};
use std::path::Path;
use download::{get, Download};

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

fn lunch(sub_matches: &clap::ArgMatches) {

}
