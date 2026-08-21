//! 地表層のタイル画像が並ぶ置き場という役割を持つパスの型。担当するのは、材質名から画像ファイルのパスを
//! 導くことと、その画像のバイト列を読むことである。
//!
//! 綴り(ディレクトリ名と拡張子)をこの型の中へ閉じるのは、置き場を移したときに触る場所を1つにするためである。

use std::path::{Path, PathBuf};

use blitz_engine::surface_layer_textures::エンジン材質名;

use super::error::地表層テクスチャ集コンパイルエラー;
use crate::asset_layout::ソースルート;

/// ソースルートの直下に置くタイル画像のディレクトリ名。
const タイルのディレクトリ名: &str = "surface_layer_textures";

/// タイル画像の拡張子。段Dの実タイルが届いても、この置き場の中身を差し替えるだけで済むよう1つに固定する。
const タイル画像の拡張子: &str = "png";

pub struct 地表層タイルの置き場(PathBuf);

impl 地表層タイルの置き場 {
    pub fn ソースルートの下に作る(ソースルート: &ソースルート) -> Self {
        Self(ソースルート.宣言の相対パスが指すソース(Path::new(タイルのディレクトリ名)))
    }

    pub fn 存在するか(&self) -> bool {
        self.0.is_dir()
    }

    pub fn 表示の綴り(&self) -> std::path::Display<'_> {
        self.0.display()
    }

    pub(super) fn 材質のタイル画像のパス(&self, 材質名: &エンジン材質名) -> PathBuf {
        self.0.join(format!("{}.{タイル画像の拡張子}", 材質名.綴り()))
    }

    /// 材質1つ分のタイル画像のバイト列と、依存一覧へ載せるパスを返す。
    pub(super) fn 材質のタイル画像を読む(
        &self,
        材質名: &エンジン材質名,
    ) -> Result<(Vec<u8>, PathBuf), 地表層テクスチャ集コンパイルエラー> {
        let パス = self.材質のタイル画像のパス(材質名);
        if !パス.is_file() {
            return Err(地表層テクスチャ集コンパイルエラー::タイル画像が無い {
                材質名: 材質名.to_string(),
                パス: パス.display().to_string(),
            });
        }
        let バイト列 =
            std::fs::read(&パス).map_err(|原因| 地表層テクスチャ集コンパイルエラー::タイル画像の読込に失敗 {
                材質名: 材質名.to_string(),
                原因: 原因.to_string(),
            })?;
        Ok((バイト列, パス))
    }
}
