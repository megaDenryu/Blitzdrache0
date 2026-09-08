//! 二段の位置の2つの口が、それぞれ呼んでよい境界の外で呼ばれていないかの検査(剛体の状態と接触の判断3の2026-09-08と2026-09-09の実装の細目の機械強制)。
//! 畳む口(和を単精度へ畳む)を呼んでよい境界は、GPU境界(`gpu_layout/rigid/`のバイト列化)と、`配置`が持つ描画の変換の生成(`rigid_body/placement.rs`)の2つである。
//! 読み口(基準原点を選ぶための粗い位置)を呼んでよいのは、衝突の問い合わせの基準原点のモジュール(`contact/query_origin/`)だけである。
//! それ以外のファイルにどちらかの口の呼び出しが現れたら、CPUの物理の途中で2段を1本の単精度へ落とす経路が入ったことになる。
//! Rustの可視性は先祖のモジュールへしか絞れず、境界が別のモジュールの木にあるため、可視性の代わりにこの検査が限定を課す。
//! 注意: 検出パターンの綴りをこのファイルに連続して書くと自分自身を違反として検出するため、分割リテラルの連結で回避する。

use std::path::Path;

use super::source_lexing::コードだけの行一覧;
use super::violation::違反;

// 口1つの綴りと、その口を呼んでよいファイル(リポジトリルートからの相対パス。型の定義と試験を含む)と、違反の文言。
struct 限定する口 {
    綴り: &'static str,
    許可するファイル一覧: &'static [&'static str],
    違反の文言: &'static str,
}

const 畳む口: 限定する口 = 限定する口 {
    綴り: concat!("境界用に", "畳んだ位置"),
    許可するファイル一覧: &[
        "crates/blitz_math/src/frame/two_tier_position.rs",
        "crates/blitz_math/src/frame/two_tier_position_tests.rs",
        "crates/blitz_sim/src/rigid_body/placement.rs",
        "crates/blitz_sim/src/gpu_layout/rigid/motion_state_bytes.rs",
        "crates/blitz_sim/src/gpu_layout/rigid/previous_state_bytes.rs",
    ],
    違反の文言: "二段の位置を単精度へ畳む口を境界の外で呼んでいる(2段の意味を保つ演算を使う)",
};

const 読み口: 限定する口 = 限定する口 {
    綴り: concat!("基準原点を選ぶ", "ための粗い位置"),
    許可するファイル一覧: &[
        "crates/blitz_math/src/frame/two_tier_position_algebra.rs",
        "crates/blitz_math/src/frame/two_tier_position_tests.rs",
        "crates/blitz_sim/src/contact/query_origin/",
    ],
    違反の文言: "二段の位置の粗い位置の読み口を衝突の問い合わせの基準原点の外で呼んでいる(基準原点の型を通す)",
};

impl 限定する口 {
    fn 許可するファイルか(&self, パス: &Path) -> bool {
        let 綴り = パス.to_string_lossy().replace('\\', "/");
        self.許可するファイル一覧.iter().any(|許可| {
            if 許可.ends_with('/') {
                綴り.contains(許可)
            } else {
                綴り.ends_with(許可)
            }
        })
    }

    fn 検査する(&self, パス: &Path, 内容: &str) -> Vec<違反> {
        if self.許可するファイルか(パス) {
            return Vec::new();
        }
        コードだけの行一覧(内容)
            .iter()
            .enumerate()
            .filter(|(_, 行)| 行.contains(self.綴り))
            .map(|(添字, _)| 違反::行単位(パス.to_path_buf(), 添字 + 1, self.違反の文言.to_string()))
            .collect()
    }
}

pub fn 検査する(パス: &Path, 内容: &str) -> Vec<違反> {
    let mut 違反一覧 = 畳む口.検査する(パス, 内容);
    違反一覧.extend(読み口.検査する(パス, 内容));
    違反一覧
}

#[cfg(test)]
mod tests {
    use super::*;

    fn 呼び出しの原文(口: &限定する口) -> String {
        format!("let 位置 = 配置.重心の位置().{}();\n", 口.綴り)
    }

    #[test]
    fn 畳む口は境界の外の呼び出しだけを違反にする() {
        let 原文 = 呼び出しの原文(&畳む口);
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
    fn 読み口は基準原点のモジュールと型の定義の外の呼び出しだけを違反にする() {
        let 原文 = 呼び出しの原文(&読み口);
        assert_eq!(
            検査する(Path::new("crates/blitz_sim/src/contact/pipeline/substep_predict.rs"), &原文).len(),
            1
        );
        assert_eq!(検査する(Path::new("crates/blitz_sim/src/rigid_body/placement.rs"), &原文).len(), 1);
        assert!(検査する(Path::new("crates/blitz_sim/src/contact/query_origin/mod.rs"), &原文).is_empty());
        assert!(検査する(Path::new("crates\\blitz_sim\\src\\contact\\query_origin\\pair_tests.rs"), &原文).is_empty());
        assert!(検査する(Path::new("crates/blitz_math/src/frame/two_tier_position_algebra.rs"), &原文).is_empty());
    }

    #[test]
    fn コメントの中の綴りは数えない() {
        let 原文 = format!("// {} と {} は境界だけが読む\nlet a = 1;\n", 畳む口.綴り, 読み口.綴り);
        assert!(検査する(Path::new("crates/blitz_sim/src/contact/pipeline/substep_predict.rs"), &原文).is_empty());
    }
}
