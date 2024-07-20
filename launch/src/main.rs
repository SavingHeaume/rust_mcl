use clap::{arg, Command};
use download::get;

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
        _ => unreachable!(),
    }
}

fn get_version_manifest() -> model::version_manifest::VersionManifest {
    return get("https://launchermeta.mojang.com/mc/game/version_manifest.json")
        .unwrap()
        .json::<model::version_manifest::VersionManifest>()
        .unwrap();
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
