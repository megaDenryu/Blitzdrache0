//! `GET /api/チャンク/{x}/{z}/高さ格子`が、まだ保存されていないチャンクへ大域のマザーハイトマップ
//! からの切り出しを配ることを確かめる。担当するのは、切り出した値が大域の同じ格子点を指すことと、
//! 隣り合うチャンクの縁が一致することと、大域が未保存のときは従来どおり204番へ落ちることである。
//! 書き出しと焼きまで通す検査は`chunk_heightmap_mother_cutout_export.rs`が担当する。
#![allow(clippy::unwrap_used)]
#![allow(non_snake_case)]

use axum::http::StatusCode;

use crate::common::チャンク一辺頂点数;

#[tokio::test]
async fn 未保存のチャンクへはマザーの同じ格子点が配られる() {
    let 一時 = crate::common::一時プロジェクト::生成する("chunk_height_cutout_values");
    crate::common::大域を一意な値で用意する(&一時);

    let (状態, 本体) = crate::common::高さ格子を取得する(&一時, 1, 0).await;
    assert_eq!(状態, StatusCode::OK);
    let 高さ一覧 = crate::common::高さ一覧へ解く(&本体);
    assert_eq!(高さ一覧.len(), チャンク一辺頂点数 * チャンク一辺頂点数);
    // チャンク(1,0)のコアは大域x=2から4・大域z=0から2を指す(1px重複共有の添字対応)。
    for 格子z in 0..チャンク一辺頂点数 {
        for 格子x in 0..チャンク一辺頂点数 {
            assert_eq!(
                高さ一覧[格子z * チャンク一辺頂点数 + 格子x],
                crate::common::マザーの高さ(2 + 格子x, 格子z),
                "格子({格子x}, {格子z})が大域の同じ格子点を指していない"
            );
        }
    }
}

#[tokio::test]
async fn 隣り合う未保存チャンクの縁は一致する() {
    let 一時 = crate::common::一時プロジェクト::生成する("chunk_height_cutout_seam");
    crate::common::大域を一意な値で用意する(&一時);

    let (_, 左の本体) = crate::common::高さ格子を取得する(&一時, 0, 0).await;
    let (_, 右の本体) = crate::common::高さ格子を取得する(&一時, 1, 0).await;
    let 左 = crate::common::高さ一覧へ解く(&左の本体);
    let 右 = crate::common::高さ一覧へ解く(&右の本体);
    for 格子z in 0..チャンク一辺頂点数 {
        let 左の右端 = 左[格子z * チャンク一辺頂点数 + (チャンク一辺頂点数 - 1)];
        let 右の左端 = 右[格子z * チャンク一辺頂点数];
        assert_eq!(左の右端, 右の左端, "行{格子z}の縁が食い違う");
    }
}

#[tokio::test]
async fn 大域が未保存なら従来どおり204を返す() {
    let 一時 = crate::common::一時プロジェクト::生成する("chunk_height_cutout_no_mother");
    // 区画割りだけを保存し、マザーハイトマップは保存しない。
    crate::common::区画割りを保存する(&一時).await;

    let (状態, 本体) = crate::common::高さ格子を取得する(&一時, 0, 0).await;
    assert_eq!(状態, StatusCode::NO_CONTENT);
    assert!(本体.is_empty());
}

#[tokio::test]
async fn 大域の構造も未保存なら204を返す() {
    let 一時 = crate::common::一時プロジェクト::生成する("chunk_height_cutout_empty_project");

    let (状態, _) = crate::common::高さ格子を取得する(&一時, 0, 0).await;
    assert_eq!(状態, StatusCode::NO_CONTENT);
}
