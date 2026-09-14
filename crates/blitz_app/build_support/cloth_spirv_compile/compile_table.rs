//! 布シェーダーのソース、エントリポイント、出力ファイル名の対応表。
//! 表の1行が1ソースファイルであり、エントリ名とステージの組み合わせは3通りしかないため、生成関数で組み立てる。

use super::super::slangc_entry_compile::エントリ指定;

/// 出力ファイル名は`embedded_cloth_shaders`と一致させる。
pub(super) const コンパイル表: [(&str, &[エントリ指定]); 11] = [
    (
        "cloth_step.slang",
        &[
            コンピュートのエントリを組み立てる("interventionMain", "cloth_intervention.spv"),
            コンピュートのエントリを組み立てる("integrateMain", "cloth_integrate.spv"),
        ],
    ),
    (
        "cloth_target.slang",
        &[
            コンピュートのエントリを組み立てる("targetUpdateMain", "cloth_target_update.spv"),
            コンピュートのエントリを組み立てる("targetConstraintMain", "cloth_target_constraint.spv"),
        ],
    ),
    (
        "cloth_constraint.slang",
        &[
            コンピュートのエントリを組み立てる("lambdaClearMain", "cloth_lambda_clear.spv"),
            コンピュートのエントリを組み立てる("constraintMain", "cloth_constraint.spv"),
        ],
    ),
    ("cloth_bending.slang", &[コンピュートのエントリを組み立てる("bendingConstraintMain", "cloth_bending_constraint.spv")]),
    (
        "cloth_hash.slang",
        &[
            コンピュートのエントリを組み立てる("hashClearMain", "cloth_hash_clear.spv"),
            コンピュートのエントリを組み立てる("hashStoreMain", "cloth_hash_store.spv"),
        ],
    ),
    ("cloth_separate.slang", &[コンピュートのエントリを組み立てる("separateMain", "cloth_separate.spv")]),
    (
        "cloth_finish.slang",
        &[
            コンピュートのエントリを組み立てる("collisionPushOutMain", "cloth_collision_push_out.spv"),
            コンピュートのエントリを組み立てる("finishMain", "cloth_finish.spv"),
        ],
    ),
    ("cloth_vertex.slang", &[コンピュートのエントリを組み立てる("vertexGenMain", "cloth_vertex_gen.spv")]),
    ("cloth_draw.slang", &[頂点のエントリを組み立てる("cloth_draw_vertex.spv"), 画素段のエントリを組み立てる("cloth_draw_fragment.spv")]),
    ("cloth_draw_distant_environment.slang", &[画素段のエントリを組み立てる("cloth_draw_distant_environment_fragment.spv")]),
    ("cloth_shadow.slang", &[頂点のエントリを組み立てる("cloth_shadow_vertex.spv"), 画素段のエントリを組み立てる("cloth_shadow_fragment.spv")]),
];

const fn コンピュートのエントリを組み立てる(エントリ名: &'static str, 出力ファイル名: &'static str) -> エントリ指定 {
    エントリ指定 {
        エントリ名,
        ステージ: "compute",
        出力ファイル名,
    }
}

const fn 頂点のエントリを組み立てる(出力ファイル名: &'static str) -> エントリ指定 {
    エントリ指定 {
        エントリ名: "vertexMain",
        ステージ: "vertex",
        出力ファイル名,
    }
}

const fn 画素段のエントリを組み立てる(出力ファイル名: &'static str) -> エントリ指定 {
    エントリ指定 {
        エントリ名: "fragmentMain",
        ステージ: "fragment",
        出力ファイル名,
    }
}
