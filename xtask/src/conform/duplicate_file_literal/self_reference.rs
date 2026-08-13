//! 検査の対象の綴りを自分で列挙する台帳の一覧と、その一覧が台帳でなくなったことの検査。
//!
//! 台帳はどのファイルとどのファイルを結ぶかを書く場所であり、そこに並ぶ綴りは検査の対象そのものである。
//! 正本を別に持てないうえ、綴りを誤ればその台帳を使う検査が「読み取りに失敗した」として止まるか、
//! 対象を1つも見つけられずに止まるため、この検査が重ねて見る必要がない。
//!
//! 置き場ごと外さずに1件ずつ挙げるのは、検査の実装が本番のファイル名を持ったときに見逃さないためである。
//! 一覧へ載せてよいのは、検査の一部として対象の綴りを列挙する台帳だけである。新しい台帳を作ったら足す。
//! 判定に使うのは今の衝突の有無ではなく役割である。衝突で判定すると、対象の綴りを列挙しているのに
//! たまたま同名が他所に無い台帳を「載せてはいけない」ことになり、同名が現れた日に偽の違反へ変わる。

use std::path::{Path, PathBuf};

use super::extract;
use crate::conform::error::規約検査の破れ;
use crate::conform::source_lexing;
use crate::conform::violation::違反;

/// 注意: 載せてよいのは、対象の綴りを列挙する検査の台帳だけである。
const 対象の綴りを列挙する台帳一覧: [&str; 25] = [
    "xtask/src/conform/depth_contract/table/camera.rs",
    "xtask/src/conform/depth_contract/table/camera_compare.rs",
    "xtask/src/conform/depth_contract/table/shadow.rs",
    "xtask/src/conform/drop_impl.rs",
    "xtask/src/conform/duplicate_file_literal/allowance/table.rs",
    "xtask/src/conform/lighting_query_declaration/table.rs",
    "xtask/src/conform/module_import_boundary/table.rs",
    "xtask/src/conform/shader_binding/table/geometry_set.rs",
    "xtask/src/conform/shader_binding/table/lighting_set.rs",
    "xtask/src/conform/shader_binding/table/material_set.rs",
    "xtask/src/conform/shader_binding/table/view_pass_set.rs",
    "xtask/src/conform/shader_constant/table/atmosphere.rs",
    "xtask/src/conform/shader_constant/table/auto_exposure.rs",
    "xtask/src/conform/shader_constant/table/clustered_lighting.rs",
    "xtask/src/conform/shader_constant/table/distant_environment.rs",
    "xtask/src/conform/shader_constant/table/instance_transform.rs",
    "xtask/src/conform/shader_constant/table/point_light_shadow.rs",
    "xtask/src/conform/shader_constant/table/scene.rs",
    "xtask/src/conform/shader_constant/table/temporal_reconstruction.rs",
    "xtask/src/conform/shader_constant/table/workgroup_threads.rs",
    "xtask/src/conform/shader_uniform_alias/table.rs",
    "xtask/src/conform/single_lighting_slot_write.rs",
    "xtask/src/conform/wording_contract/table/atmosphere_readback.rs",
    "xtask/src/conform/wording_contract/table/exit_report.rs",
    "xtask/src/conform/wording_contract/table/shader_reload.rs",
];

/// 走査で得るパスの区切り文字は実行環境で変わるため、斜線へ揃えてから一覧と照合する。
fn 一覧の表記へ揃える(パス: &Path) -> String {
    パス.to_string_lossy().replace('\\', "/")
}

pub(super) fn 台帳のファイルか(パス: &Path) -> bool {
    対象の綴りを列挙する台帳一覧.contains(&一覧の表記へ揃える(パス).as_str())
}

/// 一覧に載っているのに台帳でなくなったファイルを違反として報告し、一覧からの削除を強制する。
/// 台帳であることの印は、対象の綴り(ファイル名らしい文字列リテラル)を1つ以上列挙していることである。
pub(super) fn 一覧の陳腐化を検査する() -> Result<Vec<違反>, 規約検査の破れ> {
    let mut 違反一覧 = Vec::new();
    for 台帳 in 対象の綴りを列挙する台帳一覧 {
        let 内容 = std::fs::read_to_string(Path::new(台帳))
            .map_err(|誤り| 規約検査の破れ::ファイルを読めなかった(Path::new(台帳), 誤り))?;
        if !対象の綴りを列挙しているか(&内容) {
            違反一覧.push(違反::ファイル単位(
                PathBuf::from(台帳),
                "台帳の一覧に載っているが、対象の綴りを1つも列挙していない(台帳でなくなったので一覧から削除する)".to_string(),
            ));
        }
    }
    Ok(違反一覧)
}

fn 対象の綴りを列挙しているか(内容: &str) -> bool {
    source_lexing::文字列リテラル一覧(内容)
        .iter()
        .any(|断片| extract::拡張子を含むか(&断片.中身))
}
