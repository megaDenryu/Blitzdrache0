//! フレームの定数の宣言を取り込む側の台帳。1行が1つのシェーダーであり、そのシェーダーが読む定数の正本を並べる。
//! 台帳を検査の手順と分けるのは、シェーダーが増えたときに触るのがこの並びだけになるようにするためである。

/// 宣言を持ってはならないシェーダーと、代わりに取り込むべきモジュール。
pub(super) struct 取り込む側 {
    pub(super) パス: &'static str,
    pub(super) 取り込むモジュール一覧: &'static [&'static str],
}

pub(super) const 検査対象一覧: [取り込む側; 14] = [
    取り込む側 {
        パス: "shaders/shadow.slang",
        取り込むモジュール一覧: &["cascade_shadow_uniform"],
    },
    取り込む側 {
        パス: "shaders/cloth_shadow.slang",
        取り込むモジュール一覧: &["cascade_shadow_uniform"],
    },
    取り込む側 {
        パス: "shaders/cloth_draw.slang",
        取り込むモジュール一覧: &["lighting_query", "view_uniform"],
    },
    取り込む側 {
        パス: "shaders/cloth_draw_distant_environment.slang",
        取り込むモジュール一覧: &["lighting_query"],
    },
    取り込む側 {
        パス: "shaders/cloth_shading.slang",
        取り込むモジュール一覧: &["cascade_shadow_uniform", "lighting_query", "view_uniform"],
    },
    取り込む側 {
        パス: "shaders/motion_vector.slang",
        取り込むモジュール一覧: &["view_uniform"],
    },
    取り込む側 {
        パス: "shaders/scene.slang",
        取り込むモジュール一覧: &["lighting_query", "view_uniform"],
    },
    取り込む側 {
        パス: "shaders/scene_distant_environment.slang",
        取り込むモジュール一覧: &["lighting_query"],
    },
    取り込む側 {
        パス: "shaders/scene_diagnostic.slang",
        取り込むモジュール一覧: &["cascade_shadow_uniform", "lighting_query", "view_uniform"],
    },
    取り込む側 {
        パス: "shaders/scene_surface.slang",
        取り込むモジュール一覧: &["view_uniform"],
    },
    取り込む側 {
        パス: "shaders/sky_frame.slang",
        取り込むモジュール一覧: &["sky_pass_uniform", "view_uniform"],
    },
    取り込む側 {
        パス: "shaders/particle.slang",
        取り込むモジュール一覧: &["view_uniform"],
    },
    取り込む側 {
        パス: "shaders/sph.slang",
        取り込むモジュール一覧: &["view_uniform"],
    },
    取り込む側 {
        パス: "shaders/surface_flow.slang",
        取り込むモジュール一覧: &["view_uniform"],
    },
];
