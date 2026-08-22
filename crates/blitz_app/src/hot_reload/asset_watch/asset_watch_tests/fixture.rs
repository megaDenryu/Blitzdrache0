//! 一式の再読込の試験が使う、実行時アセットの置き場を1つ組み立てる材料。
//! 担当するのは、カタログ・チャンク目録・起動時シーン・地表層テクスチャ集・生成台帳をディスクへ置くことだけである。

#![cfg(test)]
#![allow(clippy::unwrap_used)]

use std::path::{Path, PathBuf};

use blitz_engine::surface_layer_textures::{
    エンジン材質名, 世界の地表層テクスチャ集の安定IDの綴り, 地表層ごとの材質索引, 地表層のタイル, 地表層の数, 地表層テクスチャ集,
    地表層テクスチャ集を実行時形式へ格納する,
};
use blitz_engine::texture_storage::{テクスチャ格納形式, 格納済みテクスチャ};
use blitz_engine::{
    アセットID, カタログ, カタログを実行時形式へ格納する, チャンク一辺, チャンク目録, チャンク目録を実行時形式へ格納する, 実行時アセットの公開完了印,
};

pub(super) const 起動時シーンの安定IDの綴り: &str = "startup_scene";

/// 試験が作り分ける世界の種別。地表層テクスチャ集をカタログへ載せるかどうかの2つである。
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum 試験の世界の種別 {
    タイルを持たない世界,
    タイルを持つ世界,
}

pub(super) fn 試験の置き場を用意する(名前: &str) -> PathBuf {
    let 置き場 = std::env::temp_dir().join("blitzdrache0_hot_reload_asset_watch").join(名前);
    std::fs::remove_dir_all(&置き場).ok();
    std::fs::create_dir_all(&置き場).unwrap();
    置き場
}

/// カタログ・チャンク目録・起動時シーンを置き、生成台帳へ名指した世代の綴りを書く。
/// 地表層テクスチャ集を載せるかどうかで、タイルを持たない世界と持つ世界を作り分ける。
pub(super) fn 一式を書き出す(置き場: &Path, 世代の綴り: &str, 種別: 試験の世界の種別) {
    let シーンのパス = 置き場.join(format!("{起動時シーンの安定IDの綴り}.blitzasset"));
    std::fs::write(&シーンのパス, b"startup-scene-bytes").unwrap();
    let mut カタログ = カタログ::空を作る();
    カタログ.登録する(アセットID::生成する(起動時シーンの安定IDの綴り).unwrap(), シーンのパス);
    if 種別 == 試験の世界の種別::タイルを持つ世界 {
        let タイルのパス = 置き場.join(format!("{世界の地表層テクスチャ集の安定IDの綴り}.blitzasset"));
        std::fs::write(&タイルのパス, 地表層テクスチャ集を実行時形式へ格納する(&四層のテクスチャ集()).unwrap()).unwrap();
        カタログ.登録する(アセットID::生成する(世界の地表層テクスチャ集の安定IDの綴り).unwrap(), タイルのパス);
    }
    std::fs::write(置き場.join("catalog.blitzcatalog"), カタログを実行時形式へ格納する(&カタログ).unwrap()).unwrap();
    let 目録 = チャンク目録::空を作る(チャンク一辺::生成する(64.0).unwrap());
    std::fs::write(
        置き場.join("chunk_directory.blitzchunks"),
        チャンク目録を実行時形式へ格納する(&目録).unwrap(),
    )
    .unwrap();
    std::fs::write(置き場.join(実行時アセットの公開完了印::ファイル名()), 世代の綴り).unwrap();
}

/// 層の数ぶんの1画素タイルを持つテクスチャ集。層と材質を1対1に結び、絵の中身は試験の関心ではない。
fn 四層のテクスチャ集() -> 地表層テクスチャ集 {
    let タイル一覧 = (0..地表層の数).map(一層ぶんのタイルを作る).collect::<Vec<_>>();
    let 索引 = 地表層ごとの材質索引::件数の内側であることを確かめて生成する([0, 1, 2, 3], タイル一覧.len()).unwrap();
    地表層テクスチャ集::生成する(タイル一覧, 索引).unwrap()
}

fn 一層ぶんのタイルを作る(層番号: usize) -> 地表層のタイル {
    let 画素 = u8::try_from(層番号).unwrap();
    let 絵 = 格納済みテクスチャ::生成する(1, 1, テクスチャ格納形式::RGBA8, vec![vec![画素, 画素, 画素, 255]]).unwrap();
    地表層のタイル::生成する(エンジン材質名::生成する(&format!("層{層番号}")).unwrap(), 絵)
}
