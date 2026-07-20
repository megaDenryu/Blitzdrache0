//! CLI引数から得る型: 起動モード・起動設定一式。

use std::path::PathBuf;

/// 起動時に指定できる実行モード。
#[derive(Debug, Clone, Copy)]
pub(crate) enum 起動モード {
    /// ユーザーが閉じるまで無期限に実行する。
    無期限実行,
    /// 指定フレーム数を描画したら自動終了する(DoDのスモーク検証用)。
    スモーク実行 { フレーム数: u32 },
}

/// CLI引数から得た起動設定一式。
pub(crate) struct 起動設定 {
    pub(crate) モード: 起動モード,
    /// ホットリロードの監視対象となるエントリファイル。既定は`shaders/scene.slang`
    /// (存在しなければ監視無効)。`import`で参照される他の.slangファイル
    /// (`pbr.slang`等)はこのエントリファイルと同じディレクトリから解決される
    /// ため、個別指定はしない。ディレクトリ全体をmtime走査で監視する。
    pub(crate) シェーダー監視パス: PathBuf,
    /// 表示するシーンのアセットID。既定は`quad`(常に存在し決定的)。
    pub(crate) シーン名: String,
    /// カタログの各アセットパスの基準ディレクトリ。既定は`assets`。
    pub(crate) アセットルート: PathBuf,
    /// `--unlit`指定でfalse。既定はtrue(PBRライティング有効、判断26)。
    pub(crate) ライティング有効: bool,
    /// `--particles`指定でtrue。既定はfalse(GPU粒子トイ、判断29)。
    pub(crate) 粒子有効: bool,
    /// `--report-gpu-times`指定でtrue。既定はfalse(パス別GPU時間の終了時コンソール出力、判断30)。
    pub(crate) gpu時間報告: bool,
    /// `--dev-ui`指定でtrue。既定はfalse(開発用UIの起動時有効化、判断34。実行中はF3でも切替可能)。
    pub(crate) 開発ui初期有効: bool,
    /// `--dump-frame <ベース名>`指定で、最終フレーム(--frames必須)の読み戻し画像を
    /// `<ベース名>.raw`(RGBA8連結)と`<ベース名>.size`(幅 高さ)へ書き出す。
    /// 親エージェントの検収(絵の目視監査)用。既定はNone。
    pub(crate) フレームダンプ先: Option<PathBuf>,
    /// `--no-post`指定でfalse。既定はtrue(HDR中間バッファ+トーンマップパス、判断38・39)。
    /// falseならシーンが直接スワップチェーンへ描く構成に戻る(DoD「チェーンの追加・削除可能」の機械実証)。
    pub(crate) ポスト処理有効: bool,
    /// `--exposure <倍率>`指定で変更。既定は1.0(トーンマップ前にHDR輝度へ掛ける露出倍率、判断39)。
    pub(crate) 露出: f32,
    /// `--blend <0..1>`指定で変更。既定は0.0(アニメーションクリップ2本のブレンド係数、判断45)。
    pub(crate) ブレンド: f32,
    /// `--cloth`指定でtrue。既定はfalse(XPBD布シミュレーション、判断52。スキン付きシーン限定)。
    pub(crate) 布有効: bool,
}
