//! エンジン材質名: エディターが地表材質へ付けた、エンジン側の材質を指す名前。担当するのは、
//! 名前として成立しない綴りを持てなくすることと、名前を運ぶ経路から生の文字列を消すことである。
//!
//! 裸の`String`で運ばないのは、材質名・ファイル名・表示名が同じ型になり、取り違えても型が通るためである。
//! 生の綴りへ戻るのはファイル名の組み立てと直列化の境界だけであり、その口を`綴り`に限る。

use super::error::地表層テクスチャ集エラー;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct エンジン材質名(String);

impl エンジン材質名 {
    pub fn 生成する(綴り: &str) -> Result<Self, 地表層テクスチャ集エラー> {
        if 綴り.trim().is_empty() {
            return Err(地表層テクスチャ集エラー::材質名が空);
        }
        Ok(Self(綴り.to_string()))
    }

    /// 境界: ファイル名の組み立てと直列化だけがこの口を通る。
    pub fn 綴り(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for エンジン材質名 {
    fn fmt(&self, 出力: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        出力.write_str(&self.0)
    }
}
