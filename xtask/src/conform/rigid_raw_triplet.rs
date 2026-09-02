//! 剛体のモジュール群の中で単精度の3つ組を宣言していないかの検査(剛体の状態と接触の判断5の機械強制)。
//! 力学の量は座標系と単位を持つ型で持ち、生の3つ組へ戻すのはGPUのバイト列化の1箇所だけである。剛体のモジュール群(`rigid_body/`・`rigid_xpbd/`)に
//! 3つ組の宣言が現れたら、型を経ずに生値を運ぶ経路が入ったことになる。
//! 注意: 検出パターンの綴りをこのファイルに連続して書くと自分自身を違反として検出するため、分割リテラルの連結で回避する。

use std::path::Path;

use super::source_lexing::コードだけの行一覧;
use super::violation::違反;

const 対象ディレクトリ一覧: [&str; 2] = ["rigid_body", "rigid_xpbd"];
const 三つ組の綴り: &str = concat!("[f32", "; 3]");

fn 剛体のモジュール群か(パス: &Path) -> bool {
    パス
        .components()
        .any(|部品| 対象ディレクトリ一覧.iter().any(|名前| 部品.as_os_str() == *名前))
}

pub fn 検査する(パス: &Path, 内容: &str) -> Vec<違反> {
    if !剛体のモジュール群か(パス) {
        return Vec::new();
    }
    コードだけの行一覧(内容)
        .iter()
        .enumerate()
        .filter(|(_, 行)| 行.contains(三つ組の綴り))
        .map(|(添字, _)| {
            違反::行単位(
                パス.to_path_buf(),
                添字 + 1,
                "剛体のモジュール群で単精度の3つ組を宣言している(座標系と単位を持つ型で持つ)".to_string(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 剛体のモジュール群の3つ組だけを違反にする() {
        let 原文 = concat!("let 成分: [f32", "; 3] = [0.0; 3];\n");
        assert_eq!(検査する(Path::new("crates/blitz_sim/src/rigid_xpbd/predictor.rs"), 原文).len(), 1);
        assert!(検査する(Path::new("crates/blitz_sim/src/gpu_layout/rigid/mod.rs"), 原文).is_empty());
        assert!(
            検査する(
                Path::new("crates/blitz_sim/src/rigid_body/body.rs"),
                "let 位置 = 位置::生成する(x, y, z);\n"
            )
            .is_empty()
        );
    }

    #[test]
    fn コメントの中の綴りは数えない() {
        let 原文 = concat!("// [f32", "; 3] は使わない\nlet a = 1;\n");
        assert!(検査する(Path::new("crates/blitz_sim/src/rigid_body/body.rs"), 原文).is_empty());
    }
}
