//! glTF読込と画像デコードの型付きエラー。

use blitz_engine::アセットID;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum アセットコンパイルエラー {
    #[error("カタログに未登録のアセットID: {0}")]
    カタログ未登録(アセットID),
    #[error("ファイル読込に失敗した: {0}")]
    ファイル読込失敗(String),
    #[error("glTFの解析に失敗した: {0}")]
    解析失敗(String),
    #[error("画像デコードに失敗した: {0}")]
    画像デコード失敗(String),
    #[error("glTFにメッシュが無い")]
    メッシュなし,
    #[error("メッシュにプリミティブが無い")]
    プリミティブなし,
    #[error("プリミティブに頂点位置(POSITION)が無い")]
    頂点位置なし,
    #[error("未対応のバッファ形式(data URI等)")]
    未対応バッファ形式,
    #[error("未対応の画像形式(data URI等)")]
    未対応画像形式,
    #[error("スキン付きプリミティブにJOINTS_0またはWEIGHTS_0が無い")]
    スキン頂点属性なし,
    #[error("CUBICSPLINE補間は未対応(判断42のスコープ外)")]
    補間CUBICSPLINE未対応,
    #[error("アセット実行時形式への変換に失敗した: {0}")]
    実行時形式変換失敗(#[from] blitz_engine::アセット実行時形式エラー),
    #[error("チャンク目録ソースの形式宣言が不正である: {0}")]
    チャンク目録ソース形式不正(String),
    #[error("未対応のチャンク目録ソース版だった: {0}")]
    チャンク目録ソース未対応版(String),
    #[error("チャンク目録ソースの{行番号}行目が不正である: {内容}")]
    チャンク目録ソース行不正 { 行番号: usize, 内容: String },
    #[error("高さ格子の扱いに失敗した: {0}")]
    高さ格子失敗(#[from] crate::height_grid::高さ格子エラー),
}
