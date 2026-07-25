//! blitz_app(起動バイナリ)の型付きエラー。

use thiserror::Error;

/// 起動から実行までで起こりうる失敗を表す。
#[derive(Debug, Error)]
pub(crate) enum 起動エラー {
    #[error("イベントループの生成・実行に失敗した: {0}")]
    イベントループ失敗(#[from] winit::error::EventLoopError),

    #[error("ウィンドウの生成に失敗した: {0}")]
    ウィンドウ生成失敗(#[from] winit::error::OsError),

    #[error("ウィンドウハンドルの取得に失敗した: {0}")]
    ハンドル取得失敗(#[from] raw_window_handle::HandleError),

    #[error("レンダラーの生成・描画に失敗した: {0}")]
    レンダラー失敗(#[from] blitz_render::レンダラーエラー),

    #[error("クリアカラーの生成に失敗した: {0}")]
    クリアカラー不正(#[from] blitz_render::クリアカラーエラー),

    #[error("シェーダー一式の生成に失敗した: {0}")]
    シェーダー一式不正(#[from] blitz_render::シェーダー一式エラー),

    #[error("粒子シェーダー一式の生成に失敗した: {0}")]
    粒子シェーダー一式不正(#[from] blitz_render::粒子シェーダー一式エラー),

    #[error("粒子素材の生成に失敗した: {0}")]
    粒子素材不正(#[from] blitz_render::粒子素材エラー),

    #[error("表面流初期状態の生成に失敗した: {0}")]
    表面流仕様不正(#[from] blitz_sim::表面流仕様エラー),

    #[error("テクスチャ素材の生成に失敗した: {0}")]
    テクスチャ素材不正(#[from] blitz_render::テクスチャ素材エラー),

    #[error("UIテクスチャ素材の生成に失敗した: {0}")]
    UIテクスチャ素材不正(#[from] blitz_render::UIテクスチャ素材エラー),

    #[error("マテリアル素材の生成に失敗した: {0}")]
    マテリアル素材不正(#[from] blitz_render::マテリアル素材エラー),

    #[error("シーン名の生成に失敗した: {0}")]
    シーン名不正(#[from] blitz_engine::アセットIDエラー),

    #[error("アセットカタログの読込に失敗した: {0}(`cargo xtask compile-assets`で生成できる)")]
    カタログ読込失敗(blitz_engine::実行時カタログ読込エラー),

    #[error("シーンの読込に失敗した: {0}(`cargo xtask compile-assets`で生成できる)")]
    シーン読込失敗(blitz_engine::実行時シーン読込エラー),

    #[error("{0}")]
    起動引数不正(#[from] crate::cli::起動引数エラー),

    #[error("チャンク目録の読込に失敗した: {0}(`cargo xtask compile-assets`で生成できる)")]
    チャンク目録読込失敗(blitz_engine::実行時チャンク目録読込エラー),

    #[error("チャンクストリーミングの進行に失敗した: {0}")]
    ストリーミング進行失敗(#[from] blitz_engine::ストリーミング調停エラー),

    #[error("チャンクストリーミングにアセットカタログが必要だが、まだ構築されていない")]
    ストリーミングカタログ未構築,

    #[error("固定経路を組み立てられない: {0}")]
    固定経路不正(#[from] blitz_engine::固定経路エラー),

    #[error("チャンク座標(x={}, z={})が準備完了と報告されたのに準備済みデータが無い", .0.x(), .0.z())]
    ストリーミング準備済みデータ不在(blitz_engine::チャンク座標),

    #[error("チャンク座標(x={}, z={})の描画束IDが起動時シーンの束IDと衝突する", .0.x(), .0.z())]
    ストリーミング束ID衝突(blitz_engine::チャンク座標),

    #[error("スキンメッシュ素材の生成に失敗した: {0}")]
    スキンメッシュ素材不正(#[from] blitz_render::スキンメッシュ素材エラー),

    #[error("布の生成に失敗した: {0}")]
    布生成失敗(#[from] blitz_sim::布生成エラー),

    #[error("布素材の生成に失敗した: {0}")]
    布素材不正(#[from] blitz_render::布素材エラー),

    #[error("シーンのスキン情報が不整合だった: {0}")]
    スキン整合性不正(String),

    #[error("フレームダンプに失敗した: {0}")]
    フレームダンプ失敗(String),

    #[error("スモークのシェーダー書き換えに失敗した: {0}")]
    シェーダー書き換え失敗(String),

    #[error("スモークのアセット書き換えに失敗した: {0}")]
    アセット書き換え失敗(String),

    #[error("スモークのピクセル判定に失敗した: {0}")]
    ピクセル判定失敗(String),
}
