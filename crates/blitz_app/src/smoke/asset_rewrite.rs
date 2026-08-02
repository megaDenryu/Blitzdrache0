//! アセットホットリロード検証用の書き換え。上書きする対をシーンで選び、同じ安定IDの生成物差し替えを検証する。
//! quadの計画は代替色の実行時アセットを現在のquadへ、材質差し替えの計画は材質の係数だけが違う生成物をそのシーンへ上書きする。
//!
//! 注意: WindowsのCopyFileEx(std::fs::copyの実体)はコピー元ファイルの更新日時を
//! 先の更新日時としてそのまま引き継ぐ。2つの生成物は同じコンパイル由来のため、
//! コピーしただけでは監視側のmtime比較(現在時刻 > 記録時刻)が進まず、
//! ホットリロードが検知されない。書き込み直後に明示的に現在時刻へ更新する。
use std::path::Path;
use std::time::SystemTime;

use crate::error::起動エラー;

/// 自分の対を持たないシーンが使う既定の対。quadの計画がこれを使う。
const 既定の差し替え元: &str = "quad_alt";
const 既定の差し替え先: &str = "quad";

pub(crate) fn アセットを書き換える(アセットルート: &Path, シーン名: &str) -> Result<(), 起動エラー> {
    let (元の安定id, 先の安定id) = super::material_reload::書き換える対(シーン名).unwrap_or((既定の差し替え元, 既定の差し替え先));
    let 元 = アセットルート.join(format!("{元の安定id}.blitzasset"));
    let 先 = アセットルート.join(format!("{先の安定id}.blitzasset"));
    std::fs::copy(&元, &先)
        .map_err(|誤り| 起動エラー::アセット書き換え失敗(format!("{} -> {}: {誤り}", 元.display(), 先.display())))?;

    let ファイル = std::fs::File::options()
        .write(true)
        .open(&先)
        .map_err(|誤り| 起動エラー::アセット書き換え失敗(format!("{}: {誤り}", 先.display())))?;
    ファイル
        .set_modified(SystemTime::now())
        .map_err(|誤り| 起動エラー::アセット書き換え失敗(format!("{}: {誤り}", 先.display())))
}
