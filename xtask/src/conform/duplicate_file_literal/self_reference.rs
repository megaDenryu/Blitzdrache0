//! 自分自身が検査の対象を名指しする台帳ファイルの一覧と、その一覧の陳腐化の検査。
//!
//! 台帳はどのファイルとどのファイルを結ぶかを書く場所であり、そこに並ぶパスは検査の対象そのものである。
//! 正本を別に持てないうえ、綴りを誤ればその台帳を使う検査が「読み取りに失敗した」として止まるため、
//! この検査が重ねて見る必要がない。
//!
//! 置き場ごと外さずに1件ずつ挙げるのは、検査の実装が本番のファイル名を持ったときに見逃さないためである。
//! 一覧は減る方向にしか動かない。載っているのに他所と綴りを分け合っていないファイルは違反として報告する。

use std::path::{Path, PathBuf};

use crate::conform::violation::違反;

/// 注意: この一覧への追加は、そのファイルが検査の台帳であるとき以外は禁止する。
const 自分で名指しする台帳一覧: [&str; 14] = [
    "xtask/src/conform/drop_impl.rs",
    "xtask/src/conform/duplicate_file_literal/allowance.rs",
    "xtask/src/conform/lighting_query_declaration/table.rs",
    "xtask/src/conform/module_import_boundary/table.rs",
    "xtask/src/conform/shader_constant/table/atmosphere.rs",
    "xtask/src/conform/shader_constant/table/auto_exposure.rs",
    "xtask/src/conform/shader_constant/table/clustered_lighting.rs",
    "xtask/src/conform/shader_constant/table/distant_environment.rs",
    "xtask/src/conform/shader_constant/table/point_light_shadow.rs",
    "xtask/src/conform/shader_constant/table/scene.rs",
    "xtask/src/conform/shader_constant/table/temporal_reconstruction.rs",
    "xtask/src/conform/shader_constant/table/workgroup_threads.rs",
    "xtask/src/conform/shader_uniform_alias/table.rs",
    "xtask/src/conform/single_lighting_slot_write.rs",
];

/// 走査で得るパスの区切り文字は実行環境で変わるため、斜線へ揃えてから一覧と照合する。
fn 一覧の表記へ揃える(パス: &Path) -> String {
    パス.to_string_lossy().replace('\\', "/")
}

pub(super) fn 台帳のファイルか(パス: &Path) -> bool {
    自分で名指しする台帳一覧.contains(&一覧の表記へ揃える(パス).as_str())
}

/// 一覧に載っているのに、他所と綴りを分け合っていない台帳を違反として報告し、一覧からの削除を強制する。
/// 判定に使うのは、そのファイルが持つ綴りのうち他のファイルにも現れたものの数である。
pub(super) fn 一覧の陳腐化を検査する(他所とも分け合う台帳一覧: &[String]) -> Vec<違反> {
    自分で名指しする台帳一覧
        .iter()
        .filter(|台帳| !他所とも分け合う台帳一覧.iter().any(|分け合う| 分け合う == *台帳))
        .map(|台帳| {
            違反::ファイル単位(
                PathBuf::from(*台帳),
                "自分で名指しする台帳の一覧に載っているが、他所と分け合う綴りを1つも持たない(一覧から削除する)".to_string(),
            )
        })
        .collect()
}
