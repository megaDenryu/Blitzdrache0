//! 布シェーダーのソース、エントリポイント、出力ファイル名の対応表。

use super::super::slangc_entry_compile::エントリ指定;

/// 出力ファイル名は`embedded_cloth_shaders`と一致させる。
pub(super) const コンパイル表: [(&str, &[エントリ指定]); 8] = [
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
