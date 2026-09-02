//! XPBDの並列方式の計測の起動指定の検査: 要求の引数が無ければ見分けないこと、方式とグラフが必須であること、
//! 数の既定と読み取りが働くこと、知らない綴りを拒むことを見る。

use blitz_render::xpbd_solver_bench_probe::XPBD並列方式;

use super::xpbd_solver_bench_request::{XPBD計測のグラフの種別, 計測の要求を見分ける};

fn 語一覧(語: &[&str]) -> Vec<String> {
    語.iter().map(|語| (*語).to_string()).collect()
}

#[test]
fn 要求の引数が無ければ見分けない() {
    assert!(計測の要求を見分ける(&語一覧(&["--xpbd-method", "atomic"])).is_none());
}

#[test]
fn 方式とグラフを読み数は既定を持つ() {
    let 指定 = match 計測の要求を見分ける(&語一覧(&[
        "--report-xpbd-solver-bench",
        "--xpbd-method",
        "coloring",
        "--xpbd-graph",
        "irregular",
    ])) {
        Some(Ok(指定)) => 指定,
        その他 => panic!("読めるはず: {その他:?}"),
    };
    assert_eq!(指定.方式, XPBD並列方式::グラフ彩色);
    assert_eq!(指定.グラフ, XPBD計測のグラフの種別::不規則);
    assert_eq!((指定.反復回数, 指定.刻み数, 指定.点の数, 指定.比較の刻み数), (4, 240, 1024, 10));
}

#[test]
fn 数を指定でき方式かグラフが無ければ失敗する() {
    let 指定 = match 計測の要求を見分ける(&語一覧(&[
        "--report-xpbd-solver-bench",
        "--xpbd-method",
        "two-stage",
        "--xpbd-graph",
        "grid",
        "--xpbd-iterations",
        "8",
        "--xpbd-steps",
        "10",
        "--xpbd-points",
        "256",
        "--xpbd-compare-steps",
        "3",
    ])) {
        Some(Ok(指定)) => 指定,
        その他 => panic!("読めるはず: {その他:?}"),
    };
    assert_eq!((指定.反復回数, 指定.刻み数, 指定.点の数, 指定.比較の刻み数), (8, 10, 256, 3));
    assert!(matches!(
        計測の要求を見分ける(&語一覧(&["--report-xpbd-solver-bench", "--xpbd-method", "atomic"])),
        Some(Err(_))
    ));
    assert!(matches!(
        計測の要求を見分ける(&語一覧(
            &["--report-xpbd-solver-bench", "--xpbd-method", "fast", "--xpbd-graph", "grid"]
        )),
        Some(Err(_))
    ));
    assert!(matches!(
        計測の要求を見分ける(&語一覧(&[
            "--report-xpbd-solver-bench",
            "--xpbd-method",
            "atomic",
            "--xpbd-graph",
            "grid",
            "--xpbd-steps",
            "0"
        ])),
        Some(Err(_))
    ));
}
