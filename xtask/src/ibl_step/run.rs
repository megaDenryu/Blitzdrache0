//! 段差を撮るblitz_appの起動。受け取るのは撮るものと書き出しのベース名、返すものは無く、
//! 成功したときには跨ぎごとの`<ベース名>_<区間識別>_<向き>_<側>`が書かれている。
//!
//! 一日ぶんを1回の起動で撮らずに束へ切るのは、1撮影の圧縮前HDRが1枚十数メガバイトあり、
//! 一日ぶんを残したままにすると数十ギガバイトを占めるためである。束ごとに読み終えた生の画像を捨てて進む。
//!
//! リリースで走らせるのは、1束が数十枚の画素をCPUへ運んで書き出すためである。画素の値は最適化水準で変わらない。
//! 時間再構成は`--no-taa`で外す。この入口の判定がバイト一致に依るため、フレームをまたぐ混合が入ると前のフレームの残りが絵に混ざる。

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::visual_sample_world::{アセットルート, シーン名};

/// 1回の起動が何を撮るか。走査の指定の枝と1対1に対応する。
pub(super) enum 撮るもの<'指定> {
    /// 跨ぎの範囲を圧縮前のHDRで撮る。段差を測る本体である。
    範囲の段差 { 最初の跨ぎ番号: usize, 跨ぎの件数: usize },
    /// 1つの跨ぎを下側・上側・下側の順に圧縮前のHDRで撮る。持ち越しと一回限りの揺れを反証する。
    跨ぎの対照 { 跨ぎ番号: usize },
    /// 1つの跨ぎの対を、明るさの圧縮を通した提示画像で撮る。オーナーが絵で見る材料である。
    /// 自動露出を外すのは、対の2枚を同じ露出条件に揃えるためである。露出が別々に追従すると、
    /// 見えている差が間接照明の段差なのか露出の違いなのか決められない。
    跨ぎの絵 { 跨ぎ番号: usize, 呼び名: &'指定 str },
}

pub(super) fn 撮る(撮るもの: &撮るもの<'_>, ベース名: &Path) -> Result<(), String> {
    let 条件名 = 条件名を組む(撮るもの);
    println!("[xtask] ibl-step撮影: {条件名}");
    let 出力 = Command::new("cargo")
        .args(["run", "--release", "-q", "-p", "blitz_app", "--", "--scene", シーン名])
        .args(["--asset-root", アセットルート])
        .arg("--no-taa")
        .args(条件別引数(撮るもの))
        .arg(書き出しの引数(撮るもの))
        .arg(ベース名)
        .output()
        .map_err(|誤り| format!("blitz_appを起動できなかった({条件名}): {誤り}"))?;
    let 標準出力 = String::from_utf8_lossy(&出力.stdout).into_owned();
    if !出力.status.success() {
        print!("{標準出力}");
        eprintln!("{}", String::from_utf8_lossy(&出力.stderr));
        return Err(format!("blitz_appが{}で失敗した({条件名})", 出力.status));
    }
    crate::validation_count::零件数を確かめる(&標準出力, &条件名)
}

fn 条件名を組む(撮るもの: &撮るもの<'_>) -> String {
    match 撮るもの {
        撮るもの::範囲の段差 {
            最初の跨ぎ番号, 跨ぎの件数
        } => format!("跨ぎ番号{最初の跨ぎ番号}から{跨ぎの件数}件"),
        撮るもの::跨ぎの対照 { 跨ぎ番号 } => format!("跨ぎ番号{跨ぎ番号}の対照"),
        撮るもの::跨ぎの絵 { 跨ぎ番号, 呼び名 } => format!("跨ぎ番号{跨ぎ番号}の絵({呼び名})"),
    }
}

fn 条件別引数(撮るもの: &撮るもの<'_>) -> Vec<String> {
    match 撮るもの {
        撮るもの::範囲の段差 {
            最初の跨ぎ番号, 跨ぎの件数
        } => vec!["--ibl-step-scan".to_string(), format!("{最初の跨ぎ番号},{跨ぎの件数}")],
        撮るもの::跨ぎの対照 { 跨ぎ番号 } => vec!["--ibl-step-control".to_string(), 跨ぎ番号.to_string()],
        撮るもの::跨ぎの絵 { 跨ぎ番号, .. } => {
            vec!["--ibl-step-scan".to_string(), format!("{跨ぎ番号},1"), "--no-auto-exposure".to_string()]
        }
    }
}

fn 書き出しの引数(撮るもの: &撮るもの<'_>) -> &'static str {
    match 撮るもの {
        撮るもの::範囲の段差 { .. } | 撮るもの::跨ぎの対照 { .. } => "--dump-hdr-frame",
        撮るもの::跨ぎの絵 { .. } => "--dump-frame",
    }
}

/// その撮影のベース名。blitz_app側が足す後置きと同じ形をここでも組む。
/// 形を1つの関数に閉じるのは、書く側と読む側の食い違いが「ファイルが無い」という遠い失敗に化けるためである。
pub(super) fn 撮影のベース名(ベース名: &Path, 上側の区間識別: u16, 方向: &str, 側: &str) -> PathBuf {
    let 名前 = ベース名
        .file_name()
        .map_or_else(|| "step".to_string(), |名前| 名前.to_string_lossy().into_owned());
    ベース名.with_file_name(format!("{名前}_{上側の区間識別}_{方向}_{側}"))
}

/// 読み終えた束の生の画像を捨てる。次の束が同じ場所へ書くわけではないため、消さないと一日ぶんが積み上がる。
pub(super) fn 撮影を捨てる(ベース名: &Path, 上側の区間識別: u16, 方向: &str) {
    for 側 in ["low", "high"] {
        let 基準 = 撮影のベース名(ベース名, 上側の区間識別, 方向, 側);
        for 拡張子 in ["hdr32", "size"] {
            let _ = std::fs::remove_file(基準.with_extension(拡張子));
        }
    }
}
