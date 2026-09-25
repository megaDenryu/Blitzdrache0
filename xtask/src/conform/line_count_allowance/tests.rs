//! 何行まで許すかの判定と、台帳の陳腐化の検査を固定する試験。

use std::path::Path;

use super::*;

const 台帳に載ったパス: &str = "xtask/src/main.rs";
const 台帳に無いパス: &str = "xtask/src/conform/line_count.rs";

#[test]
fn 台帳に無いファイルは百行を超えたら違反になる() {
    assert!(数えた行数を台帳と突き合わせる(Path::new(台帳に無いパス), 上限行数, "").is_empty());
    assert_eq!(数えた行数を台帳と突き合わせる(Path::new(台帳に無いパス), 上限行数 + 1, "").len(), 1);
}

#[test]
fn 上限を置かない区分は百行も百五十行も超えて違反にならない() {
    assert!(数えた行数を台帳と突き合わせる(Path::new(台帳に載ったパス), 上限行数 + 1, "").is_empty());
    assert!(数えた行数を台帳と突き合わせる(Path::new(台帳に載ったパス), 統合した結果の上限行数 + 1, "").is_empty());
}

#[test]
fn 台帳に載ったファイルが百行以内へ収まったら台帳からの削除を強制する() {
    assert_eq!(数えた行数を台帳と突き合わせる(Path::new(台帳に載ったパス), 上限行数, "").len(), 1);
}

#[test]
fn 統合した結果の区分は百五十行を超えたら違反になる() {
    let 許容 = 超過を許すファイル {
        パス: "crates/x/src/y.rs",
        超過を許した理由: "試験のための理由",
    };
    let 上限 = Some(統合した結果の上限行数);
    assert!(許容.行数を判定する(Path::new("crates/x/src/y.rs"), 統合した結果の上限行数, 上限).is_empty());
    assert_eq!(許容.行数を判定する(Path::new("crates/x/src/y.rs"), 統合した結果の上限行数 + 1, 上限).len(), 1);
}

#[test]
fn 区切り文字が逆斜線でも斜線表記へ揃う() {
    assert_eq!(台帳の表記へ揃える(Path::new(r"xtask\src\main.rs")), 台帳に載ったパス);
}

#[test]
fn graphiteの宣言の区分は上限を置かない() {
    let 区分 = &超過を許す区分一覧[2];
    assert!(区分.許す上限行数.is_none());
    let 許容 = 超過を許すファイル {
        パス: "crates/x/src/経路網.rs",
        超過を許した理由: "試験のための理由",
    };
    assert!(許容.行数を判定する(Path::new("crates/x/src/経路網.rs"), 統合した結果の上限行数 * 4, 区分.許す上限行数).is_empty());
}

#[test]
fn graphiteの宣言の区分へ宣言を含まないファイルを載せたら違反になる() {
    let 条件 = 原文に求める条件::Graphiteの宣言を含む;
    let パス = Path::new("crates/x/src/経路網.rs");
    assert!(
        条件
            .破れの違反(
                パス,
                "graphite::dynamic_graph_schema! { generated = \"generated/網.rs\"; }
"
            )
            .is_none()
    );
    assert!(
        条件
            .破れの違反(
                パス,
                "// dynamic_graph_schema! と書いただけの手書きのコード
pub struct 手書き;
"
            )
            .is_some()
    );
    assert!(
        原文に求める条件::無い
            .破れの違反(
                パス,
                "pub struct 手書き;
"
            )
            .is_none()
    );
}
