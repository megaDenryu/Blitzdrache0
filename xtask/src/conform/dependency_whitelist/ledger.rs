//! クレートごとに許される依存の白リスト。担当するのは、どのクレートがどの依存を持ってよいかの正本を持つことだけである。
//!
//! 検査の工程から分けているのは、クレートや依存を足すときに触るのがこの表だけであり、工程の側は触らないためである。
//! 触れる対象と変わる理由が別であることが分ける根拠である。
//!
//! 表の並びを分断しないため、行末に収まらない採用の経緯はここへ集める。blitz_collision は衝突数学層であり、
//! 世界もチャンクもアセットもGPUも知らない純粋な数学であることを、文書でなくこの表で守る
//! (参照: `_doc/設計/世界の形と衝突基盤.md`)。blitz_assembly は部品の接合と組み立ての層であり、
//! glTFもファイルシステムも知らない純粋計算である(参照: `_doc/設計/部品カタログと接合点.md`「機械強制の手段」)。
//! blitz_game はゲームロジック層であり、設計正本が許すのは blitz_engine と blitz_math の2つで、白リストへは
//! 実依存になった時点で足す(参照: `_doc/設計/ゲーム制作アーキテクチャ.md`「第1段階の定義」)。winit・egui・ashへは依存させない。
//! xtask の blitz_asset_compiler は置き場とファイル名の綴りの正本を読むためだけの依存であり、検収が写しを
//! 持たないための唯一の例外である。crossterm は`cargo xtask menu`の端末の生モード制御専用である。
//! editor_server はゲーム開発用エディター段1で新設した、ブラウザからの静的配信と生存確認の口を持つ独立サーバーである。
//! 判断5(ソースアセットの書き出し)で blitz_asset_compiler への依存を追加したのは、高さ格子のバイト書式の正本(書き手)を
//! 再利用し、editor_server 側では書式を写さないためである。blitz_engine はその関数(`高さ格子を切り出す`)の引数型
//! (チャンク座標)を組み立てるために直接要る。blitz_asset_compiler が既に blitz_engine(→blitz_render→ash)を連鎖依存に
//! 持つため、この2つの追加は依存木の到達範囲を広げない(参照: `_doc/設計/ゲーム開発用エディター基盤.md`「判断5」)。

pub(super) const 白リスト: [(&str, &[&str]); 11] = [
    ("blitz_math", &["glam"]),
    ("blitz_collision", &["blitz_math", "thiserror"]), // 衝突数学層。世界もチャンクもアセットもGPUも知らない
    ("blitz_engine", &["blitz_collision", "blitz_math", "blitz_render", "thiserror"]),
    ("blitz_assembly", &["blitz_engine", "blitz_math", "thiserror"]), // 部品の接合と組み立ての層。glTFもファイルシステムも知らない
    (
        "blitz_asset_compiler",
        &[
            "blitz_assembly",
            "blitz_collision",
            "blitz_engine",
            "blitz_math",
            "gltf",
            "image",
            "rayon",
            "serde",
            "serde_json",
            "thiserror",
        ],
    ),
    (
        "blitz_render",
        &["ash", "ash-window", "raw-window-handle", "glam", "thiserror", "blitz_math"],
    ),
    ("blitz_sim", &["blitz_math", "thiserror"]), // 判断51: シミュレーション基盤層。手法の数学のみでashもblitz_renderも知らない
    ("blitz_game", &["blitz_math"]),             // ゲームロジック層。許すのは blitz_engine と blitz_math だけ
    (
        "blitz_app",
        &[
            "blitz_engine",
            "blitz_game",
            "blitz_math",
            "blitz_render",
            "blitz_sim",
            "winit",
            "raw-window-handle",
            "thiserror",
            "egui",
            "egui-winit",
        ],
    ),
    ("xtask", &["blitz_asset_compiler", "crossterm"]), // 検収が綴りの写しを持たないための唯一の例外
    (
        "editor_server",
        &[
            "serde",
            "serde_json",
            "thiserror",
            "axum",
            "tokio",
            "tower",
            "tower-http",
            "ts-rs",
            "blitz_asset_compiler",
            "blitz_engine",
        ],
    ),
];
