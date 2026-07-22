//! 布シミュレーション(判断54)のシェーダー群をSPIR-Vへコンパイルする。
//! コンピュート9エントリ(7ファイル)+描画1組。布の有無は実行時のCLIで切り替わるため、常時ビルドする。

use std::path::Path;

use super::slangc_entry_compile::{エントリ一覧をコンパイルする, エントリ指定};
use super::slangc_locate::スランガー位置;

/// (ソースファイル名, エントリ一覧)の表。出力名は`embedded_cloth_shaders`と一致させる。
const コンパイル表: [(&str, &[エントリ指定]); 8] = [
    (
        "cloth_step.slang",
        &[
            エントリ指定 {
                エントリ名: "interventionMain",
                ステージ: "compute",
                出力ファイル名: "cloth_intervention.spv",
            },
            エントリ指定 {
                エントリ名: "integrateMain",
                ステージ: "compute",
                出力ファイル名: "cloth_integrate.spv",
            },
        ],
    ),
    (
        "cloth_attach.slang",
        &[エントリ指定 {
            エントリ名: "attachMain",
            ステージ: "compute",
            出力ファイル名: "cloth_attach.spv",
        }],
    ),
    (
        "cloth_constraint.slang",
        &[エントリ指定 {
            エントリ名: "constraintMain",
            ステージ: "compute",
            出力ファイル名: "cloth_constraint.spv",
        }],
    ),
    (
        "cloth_hash.slang",
        &[
            エントリ指定 {
                エントリ名: "hashClearMain",
                ステージ: "compute",
                出力ファイル名: "cloth_hash_clear.spv",
            },
            エントリ指定 {
                エントリ名: "hashStoreMain",
                ステージ: "compute",
                出力ファイル名: "cloth_hash_store.spv",
            },
        ],
    ),
    (
        "cloth_separate.slang",
        &[エントリ指定 {
            エントリ名: "separateMain",
            ステージ: "compute",
            出力ファイル名: "cloth_separate.spv",
        }],
    ),
    (
        "cloth_finish.slang",
        &[エントリ指定 {
            エントリ名: "finishMain",
            ステージ: "compute",
            出力ファイル名: "cloth_finish.spv",
        }],
    ),
    (
        "cloth_vertex.slang",
        &[エントリ指定 {
            エントリ名: "vertexGenMain",
            ステージ: "compute",
            出力ファイル名: "cloth_vertex_gen.spv",
        }],
    ),
    (
        "cloth_draw.slang",
        &[
            エントリ指定 {
                エントリ名: "vertexMain",
                ステージ: "vertex",
                出力ファイル名: "cloth_draw_vertex.spv",
            },
            エントリ指定 {
                エントリ名: "fragmentMain",
                ステージ: "fragment",
                出力ファイル名: "cloth_draw_fragment.spv",
            },
        ],
    ),
];

pub(super) fn 全部をコンパイルする(
    slangc: &スランガー位置,
    シェーダーディレクトリ: &Path,
    出力先ディレクトリ: &Path,
) -> Result<(), String> {
    for (ファイル名, エントリ一覧) in コンパイル表 {
        let ソース = シェーダーディレクトリ.join(ファイル名);
        エントリ一覧をコンパイルする(slangc, &ソース, 出力先ディレクトリ, エントリ一覧)?;
    }
    Ok(())
}
