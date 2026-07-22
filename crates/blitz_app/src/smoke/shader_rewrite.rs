//! ホットリロード検証用の文字列置換。監視対象ファイル(xtaskが渡した一時コピー)の
//! TINT定数を書き換えて保存する。表記はcube.slangの1行とそのまま一致させる。

use std::path::Path;

use crate::error::起動エラー;

const 初期色の行: &str = "static const float3 TINT = float3(1.0, 1.0, 1.0);";
const 書き換え後の行: &str = "static const float3 TINT = float3(0.1, 0.9, 0.2);";

pub(crate) fn シェーダーを書き換える(パス: &Path) -> Result<(), 起動エラー> {
    let 内容 =
        std::fs::read_to_string(パス).map_err(|誤り| 起動エラー::シェーダー書き換え失敗(format!("読み込みに失敗した: {誤り}")))?;
    if !内容.contains(初期色の行) {
        return Err(起動エラー::シェーダー書き換え失敗(
            "置換対象の行が見つからなかった".to_string(),
        ));
    }
    let 新しい内容 = 内容.replace(初期色の行, 書き換え後の行);
    std::fs::write(パス, 新しい内容).map_err(|誤り| 起動エラー::シェーダー書き換え失敗(format!("書き込みに失敗した: {誤り}")))
}
