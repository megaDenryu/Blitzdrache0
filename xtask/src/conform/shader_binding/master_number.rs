//! CPU正本の定数の宣言から束縛番号を読み取る工程。受け取るのは正本のパスと宣言の前置き、
//! 返すのはその宣言が持つ番号か、読めなかった理由である。
//!
//! 正本は値オブジェクトの生成を通して書くため、前置きの後ろに数がそのまま並ばず丸括弧の中へ入る。
//! 写しの側の読み取りとは規則が別物であり、どの組を突き合わせるかを持つ親からも分けてここへ閉じる。

use std::path::Path;

use crate::conform::error::規約検査の破れ;

pub(super) fn 正本の束縛番号を読む(パス: &'static str, 前置き: &'static str) -> Result<u32, 規約検査の破れ> {
    let 内容 =
        std::fs::read_to_string(Path::new(パス)).map_err(|誤り| 規約検査の破れ::ファイルを読めなかった(Path::new(パス), 誤り))?;
    番号を取り出す(&内容, 前置き).ok_or(規約検査の破れ::束縛番号を読めない { パス, 前置き })
}

/// 前置きで始まる行を1本見つけ、前置きを取り除いた残りの最初の丸括弧の中身を数として読む。
/// 見つからない場合を違反でなく失敗として扱うのは、宣言の消失を「一致した」と読み替えないためである。
fn 番号を取り出す(内容: &str, 前置き: &str) -> Option<u32> {
    let 行 = 内容.lines().find(|行| 行.trim_start().starts_with(前置き))?;
    let 残り = 行.trim_start().trim_start_matches(前置き);
    let 開き括弧 = 残り.find('(')?;
    let 閉じ括弧 = 残り.find(')')?;
    残り.get(開き括弧 + 1..閉じ括弧)?.trim().parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    const 前置き: &str = "pub(crate) const 局所可視度の束縛番号: 束縛番号 = 束縛番号::生成する";

    #[test]
    fn 生成の丸括弧の中の番号を読む() {
        let 内容 = format!("use x;\n{前置き}(7);\n");
        assert_eq!(番号を取り出す(&内容, 前置き), Some(7));
    }

    #[test]
    fn 前置きの行が無ければ読めない() {
        assert_eq!(番号を取り出す("use x;\n", 前置き), None);
    }

    #[test]
    fn 丸括弧が無ければ読めない() {
        let 内容 = format!("{前置き} = 7;\n");
        assert_eq!(番号を取り出す(&内容, 前置き), None);
    }
}
