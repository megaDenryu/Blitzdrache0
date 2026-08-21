//! 建物外形カタログを、編集サーバーと画面が読む版付きJSONへ写す境界。

use super::catalog::建物外形カタログ;
use super::error::建物外形カタログエラー;

pub(super) fn 整形済みのバイト列を作る(カタログ: &建物外形カタログ) -> Result<Vec<u8>, 建物外形カタログエラー> {
    let mut バイト列 = serde_json::to_vec_pretty(カタログ)?;
    バイト列.push(b'\n');
    Ok(バイト列)
}
