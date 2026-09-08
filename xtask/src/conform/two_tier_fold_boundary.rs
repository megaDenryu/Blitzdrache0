//! 二段の位置の和を単精度へ畳む口が、畳んでよい境界の外で呼ばれていないかの検査(剛体の状態と接触の判断3の2026-09-08の実装の細目の機械強制)。
//! 畳んでよい境界は、GPU境界(`gpu_layout/rigid/`のバイト列化)と、`配置`が持つ描画の変換の生成(`rigid_body/placement.rs`)の2つである。
//! それ以外のファイルにこの口の呼び出しが現れたら、CPUの物理の途中で2段を1本の単精度へ畳む経路が入ったことになる。
//! 衝突数学層へ形を渡す境界は畳まず、基準原点を選ぶための粗い位置との差で組む。その読み口の綴りはこの検査が拾わない。
//! Rustの可視性は先祖のモジュールへしか絞れず、2つの境界が別のモジュールの木にあるため、可視性の代わりにこの検査が限定を課す。
//! 注意: 検出パターンの綴りをこのファイルに連続して書くと自分自身を違反として検出するため、分割リテラルの連結で回避する。

use std::path::Path;

use super::source_lexing::コードだけの行一覧;
use super::violation::違反;

const 畳む口の綴り: &str = concat!("境界用に", "畳んだ位置");

// この口を呼んでよいファイル(リポジトリルートからの相対パス)。型の定義と、その型の試験と、2つの境界である。
const 許可するファイル一覧: [&str; 5] = [
    "crates/blitz_math/src/frame/two_tier_position.rs",
    "crates/blitz_math/src/frame/two_tier_position_tests.rs",
    "crates/blitz_sim/src/rigid_body/placement.rs",
    "crates/blitz_sim/src/gpu_layout/rigid/motion_state_bytes.rs",
    "crates/blitz_sim/src/gpu_layout/rigid/previous_state_bytes.rs",
];

fn 許可するファイルか(パス: &Path) -> bool {
    let 綴り = パス.to_string_lossy().replace('\\', "/");
    許可するファイル一覧.iter().any(|許可| 綴り.ends_with(許可))
}

pub fn 検査する(パス: &Path, 内容: &str) -> Vec<違反> {
    if 許可するファイルか(パス) {
        return Vec::new();
    }
    コードだけの行一覧(内容)
        .iter()
        .enumerate()
        .filter(|(_, 行)| 行.contains(畳む口の綴り))
        .map(|(添字, _)| {
            違反::行単位(
                パス.to_path_buf(),
                添字 + 1,
                "二段の位置を単精度へ畳む口を境界の外で呼んでいる(2段の意味を保つ演算を使う)".to_string(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn 呼び出しの原文() -> String {
        let 綴り = 畳む口の綴り;
        format!("let 位置 = 配置.重心の位置().{綴り}();\n")
    }

    #[test]
    fn 境界の外の呼び出しだけを違反にする() {
        let 原文 = 呼び出しの原文();
        assert_eq!(
            検査する(Path::new("crates/blitz_sim/src/contact/pipeline/substep_predict.rs"), &原文).len(),
            1
        );
        assert_eq!(検査する(Path::new("crates/blitz_sim/src/rigid_xpbd/previous_state.rs"), &原文).len(), 1);
        assert!(検査する(Path::new("crates/blitz_sim/src/rigid_body/placement.rs"), &原文).is_empty());
        assert!(検査する(Path::new("crates/blitz_sim/src/gpu_layout/rigid/motion_state_bytes.rs"), &原文).is_empty());
        assert!(検査する(Path::new("crates\\blitz_math\\src\\frame\\two_tier_position.rs"), &原文).is_empty());
    }

    #[test]
    fn 基準原点を選ぶための粗い位置の読み口は畳む口として数えない() {
        let 原文 = "let 原点 = 配置.重心の位置().基準原点を選ぶための粗い位置();\n";
        assert!(検査する(Path::new("crates/blitz_sim/src/contact/query_origin/mod.rs"), 原文).is_empty());
    }

    #[test]
    fn コメントの中の綴りは数えない() {
        let 綴り = 畳む口の綴り;
        let 原文 = format!("// {綴り} は境界だけが読む\nlet a = 1;\n");
        assert!(検査する(Path::new("crates/blitz_sim/src/contact/pipeline/substep_predict.rs"), &原文).is_empty());
    }
}
