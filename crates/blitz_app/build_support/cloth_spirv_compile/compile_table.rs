//! 布シェーダーのソース、エントリポイント、出力ファイル名の対応表。
//! 表の1行が1ソースファイルであり、エントリ名とステージの組み合わせは3通りしかないため、生成関数で組み立てる。

use super::super::slangc_entry_compile::エントリ指定;

/// 出力ファイル名は`embedded_cloth_shaders`と一致させる。
pub(super) const コンパイル表: [(&str, &[エントリ指定]); 9] = [
    (
        "cloth_step.slang",
        &[
            コンピュート("interventionMain", "cloth_intervention.spv"),
            コンピュート("integrateMain", "cloth_integrate.spv"),
        ],
    ),
    ("cloth_attach.slang", &[コンピュート("attachMain", "cloth_attach.spv")]),
    ("cloth_constraint.slang", &[コンピュート("constraintMain", "cloth_constraint.spv")]),
    (
        "cloth_hash.slang",
        &[
            コンピュート("hashClearMain", "cloth_hash_clear.spv"),
            コンピュート("hashStoreMain", "cloth_hash_store.spv"),
        ],
    ),
    ("cloth_separate.slang", &[コンピュート("separateMain", "cloth_separate.spv")]),
    ("cloth_finish.slang", &[コンピュート("finishMain", "cloth_finish.spv")]),
    ("cloth_vertex.slang", &[コンピュート("vertexGenMain", "cloth_vertex_gen.spv")]),
    (
        "cloth_draw.slang",
        &[頂点("cloth_draw_vertex.spv"), フラグメント("cloth_draw_fragment.spv")],
    ),
    (
        "cloth_shadow.slang",
        &[頂点("cloth_shadow_vertex.spv"), フラグメント("cloth_shadow_fragment.spv")],
    ),
];

const fn コンピュート(エントリ名: &'static str, 出力ファイル名: &'static str) -> エントリ指定 {
    エントリ指定 {
        エントリ名,
        ステージ: "compute",
        出力ファイル名,
    }
}

const fn 頂点(出力ファイル名: &'static str) -> エントリ指定 {
    エントリ指定 {
        エントリ名: "vertexMain",
        ステージ: "vertex",
        出力ファイル名,
    }
}

const fn フラグメント(出力ファイル名: &'static str) -> エントリ指定 {
    エントリ指定 {
        エントリ名: "fragmentMain",
        ステージ: "fragment",
        出力ファイル名,
    }
}
