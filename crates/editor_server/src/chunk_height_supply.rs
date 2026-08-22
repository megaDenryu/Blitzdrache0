//! チャンクの高さ格子を配る局面。担当するのは、保存済みの高さ編集があればそれを、
//! まだ無ければ大域のマザーハイトマップから切り出した初期値を配ることである。
//!
//! 未保存のチャンクへマザーの切り出しを配るのは、エディターが座標を種にした別の初期生成で
//! 高さを作ると、大域由来の隣接チャンクと縁の格子点が食い違い、その食い違ったまま保存された
//! 高さが書き出しの縁の一致検査で落ちるためである。大域そのものが未保存のときだけ、
//! 配る初期値が無いことをそのまま「無し」として返し、初期生成はブラウザ側が担う。

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::failure_response::失敗応答を組み立てる;
use crate::mother_height_cutout::{マザーからチャンクの高さ編集を切り出す, 高さの切り出しエラー};
use crate::resource::チャンク座標;
use crate::server_state::サーバー状態;
use crate::storage::{プロジェクト保管庫, 読み込みエラー};

/// チャンク高さ格子の取り出しエラーとは、チャンクの高さ格子を配る工程が返しうる失敗の区別のことである。
#[derive(thiserror::Error, Debug)]
pub(crate) enum チャンク高さ格子の取り出しエラー {
    #[error("保管庫の読み込みに失敗した: {0}")]
    読み込みに失敗(#[from] 読み込みエラー),

    #[error("マザーハイトマップからの切り出しに失敗した: {0}")]
    切り出しに失敗(#[from] 高さの切り出しエラー),
}

impl IntoResponse for チャンク高さ格子の取り出しエラー {
    fn into_response(self) -> Response {
        match self {
            Self::読み込みに失敗(エラー) => エラー.into_response(),
            Self::切り出しに失敗(エラー) => {
                失敗応答を組み立てる(StatusCode::INTERNAL_SERVER_ERROR, "高さ切り出しエラー", エラー.to_string())
            }
        }
    }
}

impl サーバー状態 {
    pub(crate) fn チャンクの高さ格子を読み未保存ならマザーから切り出す(
        &self,
        座標: チャンク座標,
    ) -> Result<Option<Vec<u8>>, チャンク高さ格子の取り出しエラー> {
        if let Some(バイト列) = self.保管庫().チャンクの高さ格子を読む(座標)? {
            return Ok(Some(バイト列));
        }
        let (Some(構造), Some(マザーバイト列)) = (self.保管庫().大域世界の構造を読む()?, self.保管庫().大域世界の高さ格子を読む()?)
        else {
            return Ok(None);
        };
        let 高さ編集 = マザーからチャンクの高さ編集を切り出す(構造.区画割り, &マザーバイト列, 座標)?;
        Ok(Some(高さ編集.バイト列を取り出す()))
    }
}
