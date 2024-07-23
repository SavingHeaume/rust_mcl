use crate::{Download, LibaryAllowed};
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use model::{library, version::Libraries};
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::path::Path;
use std::sync::Arc;

impl LibaryAllowed for library::Library {
    fn allowed(&self) -> bool {
        let mut allowed = true;

        if self.rules.is_some() {
            for rule in self.rules.as_ref().unwrap() {
                if rule.os.name == "osx" && !cfg!(target_os = "macos") {
                    allowed = false;
                    break;
                } else if rule.os.name == "linux" && !cfg!(target_os = "linux") {
                    allowed = false;
                    break;
                } else if rule.os.name == "windows" && !cfg!(target_os = "windows") {
                    allowed = false;
                    break;
                }
            }
        }

        if self.name.contains("natives") {
            if self.name.contains("x86") && !cfg!(target_arch = "x86") {
                allowed = false;
            } else if self.name.contains("arm64") && !cfg!(target_arch = "aarch64") {
                allowed = false;
            } else if !cfg!(target_arch = "x86_64") {
                allowed = false;
            }
        }

        allowed
    }
}

impl Download for Libraries {
    fn download(&self, game_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        println!("Downloading Libraries");

        let libraries_dir = &game_dir.join("libraries");
        if !libraries_dir.exists() {
            std::fs::create_dir_all(libraries_dir).unwrap();
        }

        let libraries: Vec<_> = self.iter().filter(|lib| lib.allowed()).collect();
        let libraries_count = libraries.len();

        let pb = ProgressBar::new(libraries_count as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
                .progress_chars("##-"),
        );

        let game_dir = Arc::new(game_dir.to_path_buf());

        libraries.par_iter().for_each(|library| {
            let result: Result<(), Box<dyn std::error::Error + Send + Sync>> = (|| {
                let library_path = game_dir
                    .join("libraries")
                    .join(&library.downloads.artifact.path);

                if !library_path.parent().unwrap().exists() {
                    std::fs::create_dir_all(library_path.parent().unwrap()).unwrap();
                }

                if library_path.exists() {
                    if crate::sha1(&library_path)
                        .unwrap()
                        .eq(&library.downloads.artifact.sha1)
                    {
                        pb.inc(1);
                        return Ok(());
                    } else {
                        std::fs::remove_dir(&library_path).unwrap();
                    }
                }

                let url = &library.downloads.artifact.url;
                let bytes = crate::get(url).unwrap().bytes().unwrap();
                std::fs::write(&library_path, bytes).unwrap();
                Ok(())
            })();

            pb.inc(1);
        });

        pb.finish_with_message("库文件下载完成");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use model::version::Version;

    #[test]
    fn test_download_library() {
        let game = reqwest::blocking::get("https://piston-meta.mojang.com/v1/packages/177e49d3233cb6eac42f0495c0a48e719870c2ae/1.21.json")
            .unwrap()
            .json::<Version>()
            .unwrap();

        let download_path = &std::env::temp_dir().join("rust-minecraft-client-launch");
        std::fs::create_dir_all(download_path).unwrap_or_else(|err| panic!("{:?}", err));

        if let Err(err) = game.libraries.download(download_path) {
            panic!("{:?}", err);
        }
    }
}
