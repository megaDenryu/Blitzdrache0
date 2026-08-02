//! 段5で廃止した、ビュー定数に照明を仮置きしていた経路が書き戻されていないことの検査。受け取るのは無し(対象の語の表がここにある)、
//! 返すのは廃止した語を持つファイルの違反一覧である。
//!
//! 廃止したのは、176バイトのビュー・シーンパス定数と、その中の方向光・点光源・環境光・ライティング有効の5フィールド、
//! および多段影定数へ「方向光が影を落とすか」を持たせる形である。どれもコンパイルの通る形で書き戻せてしまい、
//! 書き戻しても絵は同じに見えるため、名前の不在を機械的に守る
//! (参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「段階導入」の段5)。
//!
//! 走査対象をcratesとshadersに限るのは、この検査自身のソースが対象の語を持つためである。

use std::path::{Path, PathBuf};

use super::violation::違反;
use crate::file_scan;

const 走査対象ディレクトリ一覧: [&str; 2] = ["crates", "shaders"];
const 走査対象拡張子一覧: [&str; 2] = ["rs", "slang"];

/// 書き戻しを禁じる語。GPU側の構造体名と変数名とフィールド名、CPU側のモジュール名と型名である。
const 廃止語一覧: [&str; 11] = [
    "ViewPassUniform",
    "viewPassUniform",
    "view_pass_uniform",
    "view_pass_bytes",
    "view_pass_content",
    "directionalLightDirection",
    "directionalLightColor",
    "pointLightPosition",
    "pointLightColor",
    "ambientAndLightingEnabled",
    "shadowFlagsAndTexelSize",
];

pub fn 全ファイルを検査する() -> Result<Vec<違反>, String> {
    let ファイル一覧 = file_scan::対象ファイル一覧を集める(&走査対象ディレクトリ一覧, &走査対象拡張子一覧)
        .map_err(|誤り| format!("旧ビュー定数の照明フィールドの検査のファイル走査に失敗した: {誤り}"))?;
    let mut 違反一覧 = Vec::new();
    for パス in &ファイル一覧 {
        let 内容 = std::fs::read_to_string(パス).map_err(|誤り| format!("{}の読み取りに失敗した: {誤り}", パス.display()))?;
        違反一覧.extend(ファイル1つを検査する(パス, &内容));
    }
    Ok(違反一覧)
}

fn ファイル1つを検査する(パス: &Path, 内容: &str) -> Vec<違反> {
    let mut 違反一覧 = Vec::new();
    for (行番号, 行) in 内容.lines().enumerate() {
        for 語 in 廃止語一覧 {
            if 行.contains(語) {
                違反一覧.push(違反::行単位(
                    PathBuf::from(パス),
                    行番号 + 1,
                    format!("段5で廃止したビュー定数への照明の仮置きの語({語})を書き戻している"),
                ));
            }
        }
    }
    違反一覧
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 廃止した構造体名を書き戻した行を違反にする() {
        assert_eq!(
            ファイル1つを検査する(Path::new("shaders/scene.slang"), "struct ViewPassUniform\n").len(),
            1
        );
    }

    #[test]
    fn 廃止した照明フィールドを書き戻した行を違反にする() {
        let 内容 = "    float4 directionalLightColor;\n";
        assert_eq!(ファイル1つを検査する(Path::new("shaders/view_uniform.slang"), 内容).len(), 1);
    }

    #[test]
    fn 現行の名前は違反にしない() {
        let 内容 = "struct ViewUniform\nfloat4 directionToLightAndIntensity;\n";
        assert!(ファイル1つを検査する(Path::new("shaders/view_uniform.slang"), 内容).is_empty());
    }
}
