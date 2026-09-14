//! XPBDの並列方式の計測(Issue #35)のコンピュート7エントリ(4ファイル)をSPIR-Vへコンパイルする。
//! 計測の報告だけが使うが、実行時のCLIで選ぶため常時ビルドする(布と同じ扱い)。出力ファイル名は`embedded_xpbd_shaders`と一致させる。

use std::path::Path;

use super::slangc_entry_compile::{エントリ一覧をコンパイルする, エントリ指定};
use super::slangc_locate::スラングコンパイラの所在;

const コンパイル表: [(&str, &[エントリ指定]); 4] = [
    (
        "xpbd_step.slang",
        &[
            コンピュートのエントリを組み立てる("integrateMain", "xpbd_integrate.spv"),
            コンピュートのエントリを組み立てる("lambdaClearMain", "xpbd_lambda_clear.spv"),
        ],
    ),
    (
        "xpbd_atomic.slang",
        &[
            コンピュートのエントリを組み立てる("constraintAtomicMain", "xpbd_atomic_constraint.spv"),
            コンピュートのエントリを組み立てる("applyAtomicMain", "xpbd_atomic_apply.spv"),
        ],
    ),
    ("xpbd_coloring.slang", &[コンピュートのエントリを組み立てる("constraintColoredMain", "xpbd_coloring_constraint.spv")]),
    (
        "xpbd_two_stage.slang",
        &[
            コンピュートのエントリを組み立てる("constraintCandidateMain", "xpbd_two_stage_constraint.spv"),
            コンピュートのエントリを組み立てる("gatherMain", "xpbd_two_stage_gather.spv"),
        ],
    ),
];

const fn コンピュートのエントリを組み立てる(エントリ名: &'static str, 出力ファイル名: &'static str) -> エントリ指定 {
    エントリ指定 {
        エントリ名,
        ステージ: "compute",
        出力ファイル名,
    }
}

pub(super) fn 全部をコンパイルする(slangc: &スラングコンパイラの所在, シェーダーディレクトリ: &Path, 出力先ディレクトリ: &Path) -> Result<(), String> {
    for (ファイル名, エントリ一覧) in コンパイル表 {
        エントリ一覧をコンパイルする(slangc, &シェーダーディレクトリ.join(ファイル名), 出力先ディレクトリ, エントリ一覧)?;
    }
    Ok(())
}
