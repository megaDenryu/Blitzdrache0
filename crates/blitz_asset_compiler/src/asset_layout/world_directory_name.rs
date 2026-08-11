//! 世界のディレクトリ名: ソースルートの直下に置く1つの世界のディレクトリの名前を表す値オブジェクト。
//!
//! 裸の綴りで受け取らないのは、置き場を組む口へ任意の綴りが流れると、この型が名乗る「ソースルートの直下の1つの
//! 通常のディレクトリ」という不変条件が崩れるためである。検証は対象環境(Windows)で有効な1つのディレクトリ名で
//! あることまで見る。条件ごとに拒む理由が違うため、理由は`validation`の条件ごとの関数に書いた。
//!
//! 検証を構築時に行うのは、綴りが不正でも置き場を組むところまでは成功し、ファイルを開く段になって初めて
//! 「見つからない」で失敗するためである。失敗を生成の奥まで遅らせない。

mod validation;
#[cfg(test)]
mod world_directory_name_tests;

use super::error::アセット配置エラー;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct 世界のディレクトリ名(&'static str);

impl 世界のディレクトリ名 {
    pub fn 生成する(名前: &'static str) -> Result<Self, アセット配置エラー> {
        validation::一つの通常のディレクトリ名であることを確かめる(名前)?;
        Ok(Self(名前))
    }

    pub(super) fn 綴り(self) -> &'static str {
        self.0
    }

    /// 生成台帳の見出しへ焼き方の指定として載せるための綴り。パスを組む口ではない。
    pub fn 綴りを見せる(self) -> &'static str {
        self.0
    }
}
