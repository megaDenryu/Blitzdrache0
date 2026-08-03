//! 欠落と余分の差分画像の書き出し。受け取るのは比較結果と寸法、書き出すのはRGBA8の生画像とその寸法である。
//!
//! 色は3つだけにする。欠落を赤、余分を青、どちらでもない画素を黒に置く。中間の色を作らないのは、
//! 目で見て「どこが失われたか」を読む用途であり、量は表の数字が担うためである。

use std::path::Path;

use super::compare::比較結果;

const 欠落の色: [u8; 3] = [255, 0, 0];
const 余分の色: [u8; 3] = [0, 0, 255];

/// 前の実行が残した差分画像を消す。判定を書き出しより先へ置いても、判定で落ちた実行のあとに前回の差分画像が
/// 残っていれば、それがこの実行の裁定材料に見える。書き出しに至らなかった実行が絵を1枚も残さないようにする。
pub(super) fn 前の実行が残した画像を消す(書き先: &Path) -> Result<(), String> {
    for 拡張子 in ["raw", "size", "png"] {
        let パス = 書き先.with_extension(拡張子);
        match std::fs::remove_file(&パス) {
            Ok(()) => {}
            Err(誤り) if 誤り.kind() == std::io::ErrorKind::NotFound => {}
            Err(誤り) => return Err(format!("前の実行の差分画像を消せなかった({}): {誤り}", パス.display())),
        }
    }
    Ok(())
}

pub(super) fn 書き出す(書き先: &Path, 幅: usize, 高さ: usize, 比較: &比較結果) -> Result<(), String> {
    let mut rgba8 = vec![0u8; 幅 * 高さ * 4];
    for 添字 in 0..幅 * 高さ {
        let 色 = if 比較.欠落の印.get(添字).copied().unwrap_or(false) {
            欠落の色
        } else if 比較.余分の印.get(添字).copied().unwrap_or(false) {
            余分の色
        } else {
            [0, 0, 0]
        };
        for 軸 in 0..3 {
            if let (Some(枠), Some(値)) = (rgba8.get_mut(添字 * 4 + 軸), 色.get(軸)) {
                *枠 = *値;
            }
        }
        if let Some(枠) = rgba8.get_mut(添字 * 4 + 3) {
            *枠 = 255;
        }
    }
    std::fs::write(書き先.with_extension("raw"), &rgba8).map_err(|誤り| format!("差分画像を書けなかった: {誤り}"))?;
    std::fs::write(書き先.with_extension("size"), format!("{幅} {高さ}\n")).map_err(|誤り| format!("差分画像の寸法を書けなかった: {誤り}"))
}
