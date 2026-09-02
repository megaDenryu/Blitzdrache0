//! 参照比較の条件の台帳。実行名と、アプリへ渡す布の引数の組を並べる。条件を足すときに触るのはこのファイルだけである。

/// 有限のコンプライアンス(メートル毎ニュートン)。刻み依存量α̃=0.01×3600=36毎キログラムであり、有効逆質量512の約7%として乗数の項が式へ効く。
const 有限のコンプライアンス: &str = "0.01";
/// 水平に敷いた布の曲げの2条件の曲げのコンプライアンス(毎ニュートンメートル)。0は硬い曲げ、1000はほぼ曲げ無しである。
const 硬い曲げ: &str = "0";
const 柔らかい曲げ: &str = "1000";
/// 参照比較の7条件(実行名・アプリの引数)。
pub(super) const 参照比較の条件一覧: [(&str, &[&str]); 7] = [
    ("hard", &["--cloth-xpbd-reference", "0"]),
    ("soft", &["--cloth-xpbd-reference", 有限のコンプライアンス]),
    ("floor", &["--cloth-xpbd-reference-below-floor", "0"]),
    (
        "bend_stiff",
        &[
            "--cloth-xpbd-reference",
            "0",
            "--cloth-xpbd-reference-shape",
            "horizontal-top-row",
            "--cloth-xpbd-reference-bending",
            硬い曲げ,
        ],
    ),
    (
        "bend_soft",
        &[
            "--cloth-xpbd-reference",
            "0",
            "--cloth-xpbd-reference-shape",
            "horizontal-top-row",
            "--cloth-xpbd-reference-bending",
            柔らかい曲げ,
        ],
    ),
    (
        "two_edges",
        &["--cloth-xpbd-reference", "0", "--cloth-xpbd-reference-shape", "horizontal-two-edges"],
    ),
    (
        "one_point",
        &["--cloth-xpbd-reference", "0", "--cloth-xpbd-reference-shape", "horizontal-one-point"],
    ),
];
