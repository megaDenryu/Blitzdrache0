//! 正解表を読めない事情。破れうる前提を枝で数え上げ、どの前提が破れたかを型で言う。

use thiserror::Error;

#[derive(Debug, Error)]
pub enum 正解表の読み込みエラー {
    #[error("正解表のファイルを読めない: {パス}: {誤り}")]
    ファイルを読めない { パス: String, 誤り: std::io::Error },

    #[error("正解表のJSONを解けない: {0}")]
    正解表のJSONを解けない(String),

    #[error("正解表に「{キー名}」が無い")]
    キーが無い { キー名: &'static str },

    #[error("正解表の「{キー名}」が期待した形でない")]
    値の形が違う { キー名: &'static str },

    #[error("正解表の座標系が「{綴り}」である。この読み手はglTFの座標系だけを受け取る")]
    座標系がglTFでない { 綴り: String },
}
