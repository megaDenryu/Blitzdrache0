//! エディターチャンクソースの版の判別と最新への変換の検査。担当するのは、形式版1から形式版3までが読めること、
//! 版1が「先頭の層だけの重み」へ写ること、素材のパスがソースの隣から解決されることである。
//! 散布の欄の検査は`scatter_tests`が持ち、この検査が用意する置き場と素材の下ごしらえを共有する。

#![allow(clippy::unwrap_used)]

use std::path::{Path, PathBuf};

use super::エディターチャンクソース;
use crate::surface_material::weights::{地表材質の重み格子, 地表材質の重み格子を格納する, 地表材質の重み格子諸元};

pub(super) const 一辺の標本数: u32 = 3;

/// 検査ごとに別の場所へ書くのは、同じ名前のファイルを並行実行の検査どうしが上書きし合わないためである。
pub(super) fn 検査の置き場を用意する(名前: &str) -> PathBuf {
    let 置き場 = std::env::temp_dir().join("blitzdrache0_editor_chunk_source").join(名前);
    std::fs::remove_dir_all(&置き場).ok();
    std::fs::create_dir_all(&置き場).unwrap();
    置き場
}

pub(super) fn 高さ格子を置く(置き場: &Path) {
    let 諸元 = crate::height_grid::高さ格子諸元::生成する(2, 1, 4.0).unwrap();
    let 標本数 = usize::try_from(諸元.標本数()).unwrap();
    let 格子 = crate::height_grid::高さ格子::生成する(諸元, vec![0.0; 標本数]).unwrap();
    std::fs::write(置き場.join("地形.heightgrid"), crate::height_grid::高さ格子を格納する(&格子)).unwrap();
}

pub(super) fn 重み格子を置く(置き場: &Path) {
    let 諸元 = 地表材質の重み格子諸元::生成する(一辺の標本数).unwrap();
    let 標本数 = usize::try_from(一辺の標本数 * 一辺の標本数).unwrap();
    let mut 重み一覧 = Vec::new();
    for _ in 0..標本数 {
        重み一覧.extend_from_slice(&[0, 0, 255, 0]);
    }
    let 格子 = 地表材質の重み格子::生成する(諸元, 重み一覧).unwrap();
    std::fs::write(置き場.join("材質.surfaceweights"), 地表材質の重み格子を格納する(&格子)).unwrap();
}

pub(super) fn ソースを置いて読む(名前: &str, 本文: &str, 重みを置くか: bool) -> エディターチャンクソース {
    let 置き場 = 検査の置き場を用意する(名前);
    高さ格子を置く(&置き場);
    if 重みを置くか {
        重み格子を置く(&置き場);
    }
    let ソースパス = 置き場.join("チャンク.json");
    std::fs::write(&ソースパス, 本文).unwrap();
    エディターチャンクソース::ファイルから読む(&ソースパス).unwrap()
}

#[test]
fn 形式版1は重みを持たず先頭の層だけの格子へ写る() {
    let ソース = ソースを置いて読む("version1", r#"{"形式版":1,"高さ格子":"地形.heightgrid","建物配置一覧":[]}"#, false);
    let 格子 = ソース.地表材質の重み格子を得る(一辺の標本数).unwrap();
    assert_eq!(格子.標本の重み(1, 1).unwrap(), [255, 0, 0, 0]);
    assert_eq!(ソース.素材ファイルのパス一覧().len(), 1);
}

#[test]
fn 形式版2は隣の重み格子を読む() {
    let ソース = ソースを置いて読む(
        "version2",
        r#"{"形式版":2,"高さ格子":"地形.heightgrid","地表材質の重み格子":"材質.surfaceweights","建物配置一覧":[]}"#,
        true,
    );
    let 格子 = ソース.地表材質の重み格子を得る(一辺の標本数).unwrap();
    assert_eq!(格子.標本の重み(1, 1).unwrap(), [0, 0, 255, 0]);
    assert_eq!(ソース.素材ファイルのパス一覧().len(), 2);
}

#[test]
fn 対応しない形式版は型付きエラーで拒む() {
    let 置き場 = 検査の置き場を用意する("version_unknown");
    let ソースパス = 置き場.join("チャンク.json");
    std::fs::write(&ソースパス, r#"{"形式版":99,"高さ格子":"地形.heightgrid","建物配置一覧":[]}"#).unwrap();
    assert!(エディターチャンクソース::ファイルから読む(&ソースパス).is_err());
}

#[test]
fn 素材の相対パスが置き場の外を指すと拒む() {
    let 置き場 = 検査の置き場を用意する("version2_escape");
    let ソースパス = 置き場.join("チャンク.json");
    std::fs::write(
        &ソースパス,
        r#"{"形式版":2,"高さ格子":"地形.heightgrid","地表材質の重み格子":"../外.surfaceweights","建物配置一覧":[]}"#,
    )
    .unwrap();
    assert!(エディターチャンクソース::ファイルから読む(&ソースパス).is_err());
}
