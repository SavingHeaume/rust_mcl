use std::io::{Bytes, Read};

use model::{library, version::Libraries};

use crate::{Download, LibaryAllowed};

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
    fn download(&self, game_dir: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
        println!("Downloading Libraries");

        let libraries_dir = &game_dir.join("libraries");
        if !libraries_dir.exists() {
            let _ = std::fs::create_dir_all(libraries_dir);
        }

        for library in self {
            if !library.allowed() {
                continue;
            }

            let library_file = &library.downloads.artifact.path;
            let library_path = &libraries_dir.join(library_file);
            if !library_path.parent().unwrap().exists() {
                std::fs::create_dir_all(library_path)?;
            }

            if library_path.exists() {
                if crate::sha1(library_path)?.eq(&library.downloads.artifact.sha1) {
                    continue;
                } else {
                    std::fs::remove_dir(library_path)?;
                }
            }

            let url = &library.downloads.artifact.url;
            println!("Downloading library: {}", url);

            let bytes = crate::get(url)?.bytes()?;
            std::fs::write(library_path, bytes)?;
        }

        Ok(())
    }
}
