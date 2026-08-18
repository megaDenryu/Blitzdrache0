//! `editor_web/dist`の静的配信。ビルド前（distが無い）場合は500番で落とすのではなく、
//! その旨と対処手順を返す専用の応答にする。

use axum::{Router, http::StatusCode, response::IntoResponse};
use tower_http::services::ServeDir;

use crate::server_state::サーバー状態;

pub fn 静的配信を組み込む(
    ルーター: Router<サーバー状態>, 静的配信ディレクトリ: &std::path::Path
) -> Router<サーバー状態> {
    if 静的配信ディレクトリ.is_dir() {
        ルーター.fallback_service(ServeDir::new(静的配信ディレクトリ))
    } else {
        ルーター.fallback(未ビルド応答)
    }
}

async fn 未ビルド応答() -> impl IntoResponse {
    (
        StatusCode::SERVICE_UNAVAILABLE,
        "editor_web/dist が無い。editor_webで `npm install` の後 `npm run build` を実行するか、\
         `cargo xtask editor` で開発サーバーを併せて起動してから、このページへ改めて来る。",
    )
}
