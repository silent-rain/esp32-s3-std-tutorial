//! 静态资源文件
#![allow(unused)]
use rust_embed::RustEmbed;

/// WEB 静态资源
#[derive(Debug, Default, RustEmbed)]
#[folder = "./web/dist/"]
pub struct AssetWebDist;

impl AssetWebDist {
    pub fn to_bytes(path: String) -> Option<Vec<u8>> {
        let asset = Self::get(&path)?;
        Some(asset.data.to_vec())
    }

    /// 获取文件类型
    pub fn mimetype(path: String) -> Option<String> {
        let asset = Self::get(&path)?;
        Some(asset.metadata.mimetype().to_string())
    }
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    use super::*;

    #[test]
    fn it_file() {
        let body = match AssetWebDist::to_bytes("/assets/index-dcH5h-_h.js".to_string()) {
            Some(v) => v,
            None => "not found".as_bytes().to_vec(),
        };
        log::error!("===== {:#?}", body.bytes());
        assert_ne!(body.len(), 0);
    }
}
