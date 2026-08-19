//! クレートごとに許される依存の白リスト。担当するのは、どのクレートがどの依存を持ってよいかの正本を持つことだけである。
//!
//! 検査の工程から分けているのは、クレートや依存を足すときに触るのがこの表だけであり、工程の側は触らないためである。
//! 触れる対象と変わる理由が別であることが分ける根拠である。

pub(super) const 白リスト: [(&str, &[&str]); 10] = [
    ("blitz_math", &["glam"]),
    ("blitz_engine", &["blitz_math", "blitz_render", "thiserror"]),
    // 部品の接合と組み立ての層。glTFもファイルシステムも知らない純粋計算であることを、
    // 文書でなくこの表で守る(参照: `_doc/設計/部品カタログと接合点.md`「機械強制の手段」)
    ("blitz_assembly", &["blitz_engine", "blitz_math", "thiserror"]),
    (
        "blitz_asset_compiler",
        &[
            "blitz_assembly",
            "blitz_engine",
            "blitz_math",
            "gltf",
            "image",
            "rayon",
            "serde_json",
            "thiserror",
        ],
    ),
    (
        "blitz_render",
        &["ash", "ash-window", "raw-window-handle", "glam", "thiserror", "blitz_math"],
    ),
    // 判断51: シミュレーション基盤層。手法の数学のみでashもblitz_renderも知らない
    ("blitz_sim", &["blitz_math", "thiserror"]),
    // ゲームロジック層。設計正本が許すのはblitz_engineとblitz_mathの2つであり、白リストへは実依存になった時点で足す
    // (参照: `_doc/設計/ゲーム制作アーキテクチャ.md`「第1段階の定義」)。winit・egui・ashへは依存させない
    ("blitz_game", &["blitz_math"]),
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
    // blitz_asset_compilerは置き場とファイル名の綴りの正本を読むためだけの依存であり、検収が写しを
    // 持たないための唯一の例外である。crosstermは`cargo xtask menu`の端末の生モード制御専用
    ("xtask", &["blitz_asset_compiler", "crossterm"]),
    // ゲーム開発用エディター段1で新設。ブラウザからの静的配信と生存確認の口を持つ独立サーバー。
    // 判断5(ソースアセットの書き出し)でblitz_asset_compilerへの依存を追加した。高さ格子の
    // バイト書式の正本(書き手)を再利用するためであり、editor_server側では書式を写さない。
    // blitz_engineはその関数(`高さ格子を切り出す`)の引数型(チャンク座標)を組み立てるために直接要る。
    // blitz_asset_compilerが既にblitz_engine(→blitz_render→ash)を連鎖依存に持つため、この2つの追加は
    // 依存木の到達範囲を広げない(参照: `_doc/設計/ゲーム開発用エディター基盤.md`「判断5」)
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
