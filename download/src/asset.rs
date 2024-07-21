use std::fs;
use model::asset::*;
use parse::Parse;
use crate::Download;
use crate::get;

impl Download for AssetIndex {
    fn download(&self, game_dir: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
        println!("Downloading asset index: {}", self.id);

        let indexes_dir = &game_dir.join("assets").join("indexes");
        if !indexes_dir.exists() {
            std::fs::create_dir_all(indexes_dir).unwrap();
        }

        let path = &indexes_dir.join(&format!("{}.json", self.id));

        std::fs::File::create(path).unwrap();

        let url = &self.url;
        let text = &get(url).unwrap().text().unwrap();

        std::fs::write(path, text).unwrap();

        let index = Index::parse(text).unwrap();
        
        let object_dir = &game_dir.join("assets").join("objects");
        if !object_dir.exists() {
            std::fs::create_dir_all(object_dir).unwrap();
        }

        for (_, value) in index.objects {
            let hash = &value.hash;
            let hash_first_two = &hash[0..2];
            let first_two_dir = &object_dir.join(hash_first_two);

            if !first_two_dir.exists() {
                std::fs::create_dir_all(first_two_dir).unwrap();
            }

            let path = &first_two_dir.join(hash);
            if path.exists() {
                if crate::sha1(path)?.eq(hash) {
                    continue;
                } else {
                    std::fs::remove_file(path).unwrap();
                }
            }

            std::fs::File::create(path).unwrap();
            let url = format!(
                "https://resources.download.minecraft.net/{}/{}",
                hash_first_two, hash
            );

            println!("Downloading asset: {}", url);

            let bytes = get(&url).unwrap().bytes().unwrap();
            fs::write(path, bytes)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::Download;

    #[test]
    fn test_download_asset() {
        let asset_index = model::asset::AssetIndex {
            id: "17".to_string(),
            sha1: "fab15439bdef669e389e25e815eee8f1b2aa915e".to_string(),
            size: 447033,
            total_size: 799252591,
            url: "https://piston-meta.mojang.com/v1/packages/fab15439bdef669e389e25e815eee8f1b2aa915e/17.json".to_string(),
        };

        let download_path = &std::env::temp_dir().join("rust-minecraft-client-launch");
        std::fs::create_dir_all(download_path).unwrap_or_else(|err| panic!("{:?}", err));

        if let Err(err) = asset_index.download(download_path) {
            panic!("{:?}", err);
        }
    }
}
