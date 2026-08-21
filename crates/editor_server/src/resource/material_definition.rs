//! マテリアル定義の型契約。エディターが持つのはエンジン材質名との対応と識別用カラーの
//! 定義までであり、実テクスチャは持たない(参照: `_doc/設計/ゲーム開発用エディター基盤.md`「判断9」)。

use serde::{Deserialize, Serialize};

use super::validation_error::資源検証エラー;

/// マテリアル定義とは、地表材質をエンジン側のどの材質名へ対応させるかと、三次元ビューで
/// その材質をどの識別色(#+16進6桁)で表示するかを1件ぶん持つ値のことである。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct マテリアル定義 {
    pub エンジン材質名: String,
    pub 識別色: String,
}

impl マテリアル定義 {
    pub fn 検証する(&self) -> Result<(), 資源検証エラー> {
        if self.エンジン材質名.trim().is_empty() {
            return Err(資源検証エラー::識別子が空);
        }
        識別色の書式であることを確かめる(&self.識別色)
    }
}

/// "#"に16進6桁が続く書式(例: "#2d5a27")であることを確かめる。純粋関数のため自由関数として
/// 置く(CLAUDE.md「自由関数の許容2条件」(a))。
fn 識別色の書式であることを確かめる(値: &str) -> Result<(), 資源検証エラー> {
    let 桁部分 = 値.strip_prefix('#');
    let 書式が正しいか = 桁部分.is_some_and(|桁部分| 桁部分.len() == 6 && 桁部分.chars().all(|文字| 文字.is_ascii_hexdigit()));
    if 書式が正しいか {
        Ok(())
    } else {
        Err(資源検証エラー::色書式が不正 {
            フィールド名: "マテリアル定義.識別色",
            値: 値.to_string(),
        })
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn 例() -> マテリアル定義 {
        マテリアル定義 {
            エンジン材質名: "grass_a".to_string(),
            識別色: "#2d5a27".to_string(),
        }
    }

    #[test]
    fn 正しい形は検証を通る() {
        例().検証する().unwrap();
    }

    #[test]
    fn 材質名が空なら拒否する() {
        let mut 対象 = 例();
        対象.エンジン材質名 = "  ".to_string();
        assert!(対象.検証する().is_err());
    }

    #[test]
    fn 識別色の桁数が足りないと拒否する() {
        let mut 対象 = 例();
        対象.識別色 = "#2d5a2".to_string();
        assert!(対象.検証する().is_err());
    }

    #[test]
    fn 識別色の先頭にシャープが無いと拒否する() {
        let mut 対象 = 例();
        対象.識別色 = "2d5a27".to_string();
        assert!(対象.検証する().is_err());
    }

    #[test]
    fn 識別色に16進数でない文字があると拒否する() {
        let mut 対象 = 例();
        対象.識別色 = "#2d5a2g".to_string();
        assert!(対象.検証する().is_err());
    }
}
