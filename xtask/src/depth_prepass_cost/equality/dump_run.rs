//! 同値性の検収で撮る1条件1枚ぶんのblitz_app起動。受け取るのは条件とダンプの別とダンプ先、返すものは無い。
//!
//! 費用の計測と別の起動にするのは、1回の起動で読み戻せる画像が1枚だけであり、かつ読み戻しの同期が計測窓を歪めるためである。
//! スモーク実行(`--frames`)で起動するのは、フレームダンプが最終フレームでだけ働く仕組みだからである。

use std::path::Path;

use super::super::schedule::実行条件;
use super::super::world;

/// 撮る画像の別。1回の起動で1枚だけ撮る。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ダンプの別 {
    最終深度,
    圧縮前のHDR,
}

impl ダンプの別 {
    const fn 起動引数(self) -> &'static str {
        match self {
            Self::最終深度 => "--dump-depth-frame",
            Self::圧縮前のHDR => "--dump-hdr-frame",
        }
    }

    pub(super) const fn 拡張子(self) -> &'static str {
        match self {
            Self::最終深度 => "depth32",
            Self::圧縮前のHDR => "hdr32",
        }
    }

    pub(super) const fn 呼び名(self) -> &'static str {
        match self {
            Self::最終深度 => "最終深度",
            Self::圧縮前のHDR => "提示前HDR",
        }
    }
}

pub(super) fn 撮る(
    条件: &実行条件, 別: ダンプの別, ダンプ先: &Path, 一日内秒: Option<&String>, フレーム数: u32
) -> Result<(), String> {
    let mut 引数一覧 = world::世界の引数();
    引数一覧.extend(["--frames".to_string(), フレーム数.to_string()]);
    引数一覧.extend(world::時刻の引数(一日内秒));
    引数一覧.extend(world::条件の引数(条件));
    引数一覧.push(別.起動引数().to_string());
    引数一覧.push(ダンプ先.display().to_string());
    let 出力 = world::起こし方.コマンドを作る().args(引数一覧).output().map_err(|誤り| {
        format!(
            "{}を起動できなかった({}の{}): {誤り}",
            world::起こし方.表示の綴り(),
            条件.名前,
            別.呼び名()
        )
    })?;
    let 標準出力 = String::from_utf8_lossy(&出力.stdout).into_owned();
    if !出力.status.success() {
        print!("{標準出力}");
        eprintln!("{}", String::from_utf8_lossy(&出力.stderr));
        return Err(format!("{}の{}の撮影が{}で失敗した", 条件.名前, 別.呼び名(), 出力.status));
    }
    crate::validation_count::零件数を確かめる(&標準出力, 条件.名前)
}
