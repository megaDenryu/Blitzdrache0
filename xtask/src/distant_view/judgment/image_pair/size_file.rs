//! 採取が書き出したファイルをそのまま読む工程。受け取るのはファイルのパス、返すのは寸法の対かバイト列である。
//!
//! 寸法の行を読む口を分けて持つのは、幅と高さが並んでいないことと数として読めないことを別の破れで名指すためである。
//! 前者は書き出しがまだ走っていない印であり、後者は書き出しの書式が変わった印である。

use std::path::Path;

use crate::distant_view::error::採取の読み取りの破れ;

pub(super) fn 幅と高さを読む(パス: &Path) -> Result<(usize, usize), 採取の読み取りの破れ> {
    let 本文 = std::fs::read_to_string(パス).map_err(|誤り| 採取の読み取りの破れ::ファイルを読めなかった {
        パス: パス.to_path_buf(),
        誤り,
    })?;
    let mut 語一覧 = 本文.split_whitespace();
    let (Some(幅の綴り), Some(高さの綴り)) = (語一覧.next(), 語一覧.next()) else {
        return Err(採取の読み取りの破れ::寸法の行に幅と高さが並んでいない {
            パス: パス.to_path_buf(),
            本文: 本文.clone(),
        });
    };
    Ok((数として読む(パス, 幅の綴り)?, 数として読む(パス, 高さの綴り)?))
}

pub(super) fn バイト列を読む(パス: &Path) -> Result<Vec<u8>, 採取の読み取りの破れ> {
    std::fs::read(パス).map_err(|誤り| 採取の読み取りの破れ::ファイルを読めなかった {
        パス: パス.to_path_buf(),
        誤り,
    })
}

fn 数として読む(パス: &Path, 綴り: &str) -> Result<usize, 採取の読み取りの破れ> {
    綴り.parse().map_err(|_| 採取の読み取りの破れ::寸法の綴りを数として読めない {
        パス: パス.to_path_buf(),
        綴り: 綴り.to_string(),
    })
}
