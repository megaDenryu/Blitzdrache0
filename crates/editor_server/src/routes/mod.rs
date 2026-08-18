//! ルーターの組み立て。経路とハンドラの対応付けをこの1箇所へ集約する。
//! 段1で持つ口は生存確認と静的配信の2つだけであり、編集資源の読み書き経路は段2で足す。

mod health_get;
mod static_serve;

use axum::{Router, routing::get};

use crate::{normalized_app::経路正規化アプリ, server_state::サーバー状態};

pub fn ルーターを組み立てる(状態: サーバー状態) -> 経路正規化アプリ {
    let 静的配信ディレクトリ = 状態.静的配信ディレクトリ().to_path_buf();
    let ルーター = Router::new().route("/api/生存確認", get(health_get::生存確認を返す));
    let ルーター = static_serve::静的配信を組み込む(ルーター, &静的配信ディレクトリ).with_state(状態);
    経路正規化アプリ::組み立てる(ルーター)
}
