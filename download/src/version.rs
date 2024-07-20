use crate::get;
use crate::Download;
use model::version_manifest::Version;

impl Download for Version {
    fn download(&self, game_dir: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
        // 获取游戏版本的配置
        let game = get(&self.url)?.json::<model::version::Version>()?;

        // 创建一个version目录
        let version_dir = &game_dir.join("versions").join(&game.id);
        if !version_dir.exists() {
            std::fs::create_dir_all(version_dir)?;
        }

        // 下载资源
        game.libraries.download(game_dir)?;
        game.libraries.download(game_dir)?;

        // 创建版本的json配置
        let version_config = &game_dir
            .join("versions")
            .join(&game.id)
            .join(&format!("{}.json", &self.id));

        if version_config.exists() {
            std::fs::remove_file(version_config)?;
        }

        // 写入config内容
        std::fs::File::create(version_config)?;
        std::fs::write(version_config, get(&self.url)?.bytes()?)?;

        // 创建jar
        let path = &game_dir
            .join("versions")
            .join(&game.id)
            .join(&format!("{}.jar", &game.id));
        if path.exists() {
            if crate::sha1(path)?.eq(&game.downloads.client.sha1) {
                return Ok(());
            } else {
                std::fs::remove_file(path)?;
            }
        }

        // 下载
        std::fs::File::create(path)?;
        let bytes = get(&game.downloads.client.url)?.bytes()?;
        std::fs::write(path, bytes)?;

        Ok(())
    }
}
