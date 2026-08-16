//! 部品のglTFから接合点を読み取る工程。受け取るのは開いた文書、返すのは宣言ごとの読み取りの成否の列である。
//!
//! ローダーの中に置くのは、glTFをどう読むかの正本がローダーだからである。読んだ結果は`blitz_assembly`の型であり、
//! gltfの型はこの境界を越えない。組み立ての側は読み方を知らない。
//!
//! 宣言が1件も無いことを破れにしないのは、接合点を持たない部品でないアセット(単体で置く小物)が正当に成立し、
//! それを違反とするかは契約の側が決めることだからである。ここは読むだけで、何が違反かを持たない。
//! 参照: `_doc/設計/部品カタログと接合点.md`「置き場所はglTFの`extras`である」

mod catalog_loader;
mod declaration;
mod error;
mod extras_scan;
mod face_read;
mod outcome;
mod part_file;
mod value_read;

#[cfg(test)]
mod declaration_fixture;
#[cfg(test)]
mod face_read_rejection_tests;
#[cfg(test)]
mod joint_rejection_tests;
#[cfg(test)]
mod joint_tests;

use blitz_assembly::接合点;
use serde_json::Value;

use declaration::接合点の宣言;
use extras_scan::接合点の宣言の並びを取り出す;

pub use catalog_loader::部品カタログの読み込み係;
pub use error::{接合点読み取りエラー, 部品の読み取りエラー, 部品カタログの読み込みエラー};
pub use outcome::接合点の読み取りの成否;
pub use part_file::部品のglTFのファイル;

pub struct 部品の接合点の読み取り {
    宣言ごとの結果: Vec<接合点の読み取りの成否>,
}

impl 部品の接合点の読み取り {
    /// 先頭メッシュを参照するノードのextrasから読む。部品は1ファイル1メッシュであり、接合点はそのノードに付く。
    pub fn 先頭メッシュのノードから読み取る(文書: &gltf::Document) -> Result<Self, 接合点読み取りエラー> {
        let ノード = 文書
            .nodes()
            .find(|ノード| ノード.mesh().is_some_and(|メッシュ| メッシュ.index() == 0))
            .ok_or(接合点読み取りエラー::先頭メッシュのノードが無い)?;
        Self::ノードから読み取る(&ノード)
    }

    pub fn ノードから読み取る(ノード: &gltf::Node<'_>) -> Result<Self, 接合点読み取りエラー> {
        let 宣言一覧 = 接合点の宣言の並びを取り出す(ノード)?;
        let mut 宣言ごとの結果 = Vec::with_capacity(宣言一覧.len());
        for (添字, 宣言) in 宣言一覧.into_iter().enumerate() {
            宣言ごとの結果.push(宣言1件を読み取る(添字, 宣言));
        }
        Ok(Self { 宣言ごとの結果 })
    }

    pub fn 宣言ごとの結果(&self) -> &[接合点の読み取りの成否] {
        &self.宣言ごとの結果
    }

    pub fn 宣言の件数(&self) -> usize {
        self.宣言ごとの結果.len()
    }

    /// 全件が読めていれば接合点の列を返す。1件でも読めなければ最初の破れを返す。
    /// カタログを組む側は半端な列を受け取ってはならないため、こちらの口を使う。
    pub fn 全件の接合点(&self) -> Result<Vec<接合点>, 接合点読み取りエラー> {
        let mut 接合点一覧 = Vec::with_capacity(self.宣言ごとの結果.len());
        for 成否 in &self.宣言ごとの結果 {
            match 成否 {
                接合点の読み取りの成否::読めた(接合点) => 接合点一覧.push(接合点.clone()),
                接合点の読み取りの成否::読めなかった { 誤り, .. } => return Err(誤り.clone()),
            }
        }
        Ok(接合点一覧)
    }
}

fn 宣言1件を読み取る(添字: usize, 宣言: Value) -> 接合点の読み取りの成否 {
    if !宣言.is_object() {
        return 接合点の読み取りの成否::結果から生成する(添字, Err(接合点読み取りエラー::宣言が表でない { 添字 }));
    }
    接合点の読み取りの成否::結果から生成する(添字, 接合点の宣言::表から生成する(宣言).接合点へ組み上げる())
}
