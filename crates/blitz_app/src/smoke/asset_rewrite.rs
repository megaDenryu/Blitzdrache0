//! アセットホットリロード検証用の書き換え。監視対象アセットルート配下の
//! quad_base_color.pngをquad_alt_color.pngの内容で上書きする。
//!
//! 注意: WindowsのCopyFileEx(std::fs::copyの実体)はコピー元ファイルの更新日時を
//! 先の更新日時としてそのまま引き継ぐ。quad_alt_color.pngとquad_base_color.pngは
//! 同じ世代生成(cargo xtask gen-smoke-asset)由来で更新日時がほぼ一致するため、
//! コピーしただけでは監視側のmtime比較(現在時刻 > 記録時刻)が進まず、
//! ホットリロードが検知されない。書き込み直後に明示的に現在時刻へ更新する。
use std::path::Path;
use std::time::SystemTime;

use crate::error::起動エラー;

pub(crate) fn アセットを書き換える(アセットルート: &Path) -> Result<(), 起動エラー> {
    let 元 = アセットルート.join("smoke").join("quad_alt_color.png");
    let 先 = アセットルート.join("smoke").join("quad_base_color.png");
    std::fs::copy(&元, &先).map_err(|誤り| {
        起動エラー::アセット書き換え失敗(format!("{} -> {}: {誤り}", 元.display(), 先.display()))
    })?;

    let ファイル = std::fs::File::options()
        .write(true)
        .open(&先)
        .map_err(|誤り| 起動エラー::アセット書き換え失敗(format!("{}: {誤り}", 先.display())))?;
    ファイル
        .set_modified(SystemTime::now())
        .map_err(|誤り| 起動エラー::アセット書き換え失敗(format!("{}: {誤り}", 先.display())))
}
