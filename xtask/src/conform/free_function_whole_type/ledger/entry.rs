//! 台帳の1項目と、1つの区画分の一覧。区画とは、同じモジュールの根を共有する項目のまとまりのことである。
//!
//! 項目がパスを根からの相対で持つのは、綴りを短くして1項目を1行へ収めるためである。全体のパスを毎行に
//! 書くと整形が項目を4行へ折り返し、1つの表が1ファイル100行の原則を超える。根は区画の側が1箇所で持つ。

use std::path::PathBuf;

pub struct 未是正の自由関数 {
    根からのパス: &'static str,
    関数名: &'static str,
    型名: &'static str,
}

impl 未是正の自由関数 {
    pub const fn 生成する(根からのパス: &'static str, 関数名: &'static str, 型名: &'static str) -> Self {
        Self {
            根からのパス, 関数名, 型名
        }
    }
}

pub struct 区画の一覧 {
    モジュールの根: &'static str,
    項目一覧: &'static [未是正の自由関数],
}

impl 区画の一覧 {
    pub const fn 生成する(モジュールの根: &'static str, 項目一覧: &'static [未是正の自由関数]) -> Self {
        Self {
            モジュールの根, 項目一覧
        }
    }

    pub fn 位置一覧(&self) -> Vec<super::自由関数の位置> {
        self.項目一覧
            .iter()
            .map(|項目| super::自由関数の位置 {
                パス: PathBuf::from(self.モジュールの根)
                    .join(項目.根からのパス)
                    .to_string_lossy()
                    .replace('\\', "/"),
                関数名: 項目.関数名.to_string(),
                型名: 項目.型名.to_string(),
            })
            .collect()
    }
}
