//! 先頭メッシュの幾何をローダーに読ませ、概要を計測する工程。受け取るのは開いた文書、返すのは概要と指摘一覧である。
//! 頂点とインデックスを`loader::mesh`にそのまま読ませるのは、検査が読み方の写しを持たないためである。読めなかったこと自体を1件の違反として報告する。
//! 読み終えたデータが実行時形式の要求を満たすかの判定は`mesh_data_check`が持つ。

use blitz_engine::メッシュデータ;

use super::super::document::開いた文書;
use super::super::material_slots::材質スロット語彙;
use super::super::mesh;
use super::finding::契約指摘;
use super::mesh_data_check;
use super::result::契約検査概要;
use super::target::{名前を写す, 対象位置};

pub(super) fn 検査する(文書: &開いた文書) -> (Option<契約検査概要>, Vec<契約指摘>) {
    let メッシュ数 = 文書.document.meshes().len();
    let Some(メッシュ) = 文書.document.meshes().next() else {
        return (None, Vec::new());
    };
    let 位置 = 対象位置::メッシュ {
        添字: 0,
        名前: 名前を写す(メッシュ.name()),
    };
    let プリミティブ数 = メッシュ.primitives().len();

    let 語彙 = 材質スロット語彙::メッシュ列から作る(std::slice::from_ref(&メッシュ));
    match mesh::メッシュデータを取り出す(文書, &メッシュ, None, &語彙) {
        Ok(データ) => (Some(概要を作る(メッシュ数, プリミティブ数, &データ)), mesh_data_check::検査する(&データ)),
        Err(誤り) => (
            None,
            vec![契約指摘::違反を作る(
                位置,
                format!("頂点とインデックスを読めなかった: {誤り}"),
                "頂点位置(POSITION)を持つ三角形メッシュを書き出す。Blenderの書き出し設定で位置とインデックスを除いていないかを確かめる",
            )],
        ),
    }
}

fn 概要を作る(メッシュ数: usize, プリミティブ数: usize, データ: &メッシュデータ) -> 契約検査概要 {
    契約検査概要 {
        メッシュ数,
        プリミティブ数,
        頂点数: データ.頂点一覧.len(),
        インデックス数: データ.インデックス一覧.len(),
        境界箱: 境界箱を求める(データ),
    }
}

/// 絵に出ない事故の多くが位置と大きさの取り違えであるため、頂点位置の最小と最大を概要へ載せる。
fn 境界箱を求める(データ: &メッシュデータ) -> Option<([f32; 3], [f32; 3])> {
    let 先頭 = データ.頂点一覧.first()?;
    let mut 最小 = 先頭.位置;
    let mut 最大 = 先頭.位置;
    for 頂点 in &データ.頂点一覧 {
        for 軸 in 0..3 {
            最小[軸] = 最小[軸].min(頂点.位置[軸]);
            最大[軸] = 最大[軸].max(頂点.位置[軸]);
        }
    }
    Some((最小, 最大))
}
