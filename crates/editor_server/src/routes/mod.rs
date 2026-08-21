//! ルーターの組み立て。経路とハンドラの対応付けをこの1箇所へ集約する。

mod chunk_heightmap_get;
mod chunk_heightmap_put;
mod chunk_splat_weights_get;
mod chunk_splat_weights_put;
mod chunk_structure_get;
mod chunk_structure_put;
mod health_get;
mod material_board_get;
mod material_board_put;
mod project_info_get;
mod source_asset_export_post;
mod static_serve;
mod world_heightmap_get;
mod world_heightmap_put;
mod world_layout_lookup;
mod world_structure_get;
mod world_structure_put;

use axum::{Router, routing::get, routing::post};

use crate::{normalized_app::経路正規化アプリ, server_state::サーバー状態};

pub fn ルーターを組み立てる(状態: サーバー状態) -> 経路正規化アプリ {
    let 静的配信ディレクトリ = 状態.静的配信ディレクトリ().to_path_buf();
    let ルーター = Router::new()
        .route("/api/生存確認", get(health_get::生存確認を返す))
        .route("/api/プロジェクト情報", get(project_info_get::プロジェクト情報を返す))
        .route(
            "/api/大域世界/構造",
            get(world_structure_get::大域世界構造を返す).put(world_structure_put::大域世界構造を保存する),
        )
        .route(
            "/api/大域世界/高さ格子",
            get(world_heightmap_get::大域世界高さ格子を返す).put(world_heightmap_put::大域世界高さ格子を保存する),
        )
        .route(
            "/api/マテリアル台帳",
            get(material_board_get::マテリアル台帳を返す).put(material_board_put::マテリアル台帳を保存する),
        )
        .route(
            "/api/チャンク/{x}/{z}/構造",
            get(chunk_structure_get::チャンク構造を返す).put(chunk_structure_put::チャンク構造を保存する),
        )
        .route(
            "/api/チャンク/{x}/{z}/高さ格子",
            get(chunk_heightmap_get::チャンク高さ格子を返す).put(chunk_heightmap_put::チャンク高さ格子を保存する),
        )
        .route(
            "/api/チャンク/{x}/{z}/材質重み",
            get(chunk_splat_weights_get::チャンク材質重みを返す).put(chunk_splat_weights_put::チャンク材質重みを保存する),
        )
        .route("/api/書き出し/ソースアセット", post(source_asset_export_post::ソースアセットを書き出す));
    let ルーター = static_serve::静的配信を組み込む(ルーター, &静的配信ディレクトリ).with_state(状態);
    経路正規化アプリ::組み立てる(ルーター)
}
