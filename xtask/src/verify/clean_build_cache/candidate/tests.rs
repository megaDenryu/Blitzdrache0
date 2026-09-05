//! 候補の振り分けの検査。いま使っている出力の木と同じ場所の候補を、候補の種類に関わらず残す枝にすること。
//!
//! 試験が`CARGO_TARGET_DIR`を書き換えないのは、環境変数がプロセス全体のものであり、並列に走る他の試験と
//! 干渉するためである。いま使っている木は引数で受け取る形にしてあり、試験はパスを直に渡す。
//! 実在するディレクトリを作るのは、同じ場所かの判定が実体の解決を通すためである。

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, Ordering};

use super::掃除の候補;
use crate::verify::clean_build_cache::role::掃除の対象の役割;
use crate::verify::clean_build_cache::subject::掃除の対象;

/// 同じ試験の実行の中で置き場の名前がぶつからないようにする番号。
static 使い捨ての置き場の通し番号: AtomicU32 = AtomicU32::new(0);

fn 使い捨ての出力の木を作る() -> PathBuf {
    let 番号 = 使い捨ての置き場の通し番号.fetch_add(1, Ordering::Relaxed);
    let 木 = std::env::temp_dir().join(format!("blitzdrache0-掃除の候補の検査-{}-{番号}", std::process::id()));
    std::fs::create_dir_all(&木).unwrap();
    木
}

fn 残す枝か(役割: 掃除の対象の役割, 置き場: &Path, いま使っている木: &Path) -> bool {
    let 対象 = 掃除の対象::生成する(役割, 置き場.to_path_buf());
    matches!(
        掃除の候補::対象から生成する(対象, いま使っている木),
        掃除の候補::いま使っているため残す(_)
    )
}

#[test]
fn codexレビューの出力先をいま使っているならその候補を残す() {
    let 木 = 使い捨ての出力の木を作る();
    let codexレビューの出力先 = 木.join("codex-review");
    std::fs::create_dir_all(&codexレビューの出力先).unwrap();
    let 残すか = 残す枝か(
        掃除の対象の役割::Codexレビューのビルドの出力先,
        &codexレビューの出力先,
        &codexレビューの出力先,
    );
    std::fs::remove_dir_all(&木).unwrap();
    assert!(残すか, "いまのビルドが使っている出力の木そのものを消す枝へ積んでいる");
}

#[test]
fn 木の一部分である差分ビルドの中間データはその木を使っていても消す() {
    let 木 = 使い捨ての出力の木を作る();
    let 中間データの置き場 = 木.join("debug").join("incremental");
    std::fs::create_dir_all(&中間データの置き場).unwrap();
    let 残すか = 残す枝か(掃除の対象の役割::差分ビルドの中間データ, &中間データの置き場, &木);
    std::fs::remove_dir_all(&木).unwrap();
    assert!(!残すか, "木の一部分を意図して掃除する候補まで守っている");
}
