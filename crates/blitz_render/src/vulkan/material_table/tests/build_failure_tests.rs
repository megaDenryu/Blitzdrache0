//! 世代の構築が途中で失敗したときの後始末の検査。段4aの(ii)公開中の世代が不変であり、
//! その世代のために作った画像だけを退役させることを固定する。

#![allow(clippy::unwrap_used)]

use crate::error::レンダラーエラー;
use crate::texture_material::テクスチャ用途;
use crate::vulkan::material_table::generation_build::構築する;
use crate::vulkan::material_table::generation_id::資源表世代ID;
use crate::vulkan::material_table::ledger::資源表世代台帳;
use crate::vulkan::material_table::stage_reserve::画素段の予約枠;

use super::fixture::{余裕のある上限, 材質を作る, 検査用供給元, 検査用素材};

#[test]
fn 構築の失敗は部分生成資源だけを破棄し公開中の世代を変えない() {
    let 素材 = 検査用素材(テクスチャ用途::色);
    let 予約枠 = 画素段の予約枠::現行のシーン画素段();
    let mut 供給元 = 検査用供給元::常に成功する();
    let 初期世代 = 構築する(&mut 供給元, 資源表世代ID::最初(), 余裕のある上限(), 予約枠, &[材質を作る(1, Some(&素材))]).unwrap();
    let mut 台帳 = 資源表世代台帳::最初の世代を公開する(初期世代);
    let 公開前の世代番号 = 台帳.公開中().世代id().番号();
    let 公開前の画像枚数 = 台帳.公開中().画像枚数();

    // 正準フォールバック3枚を常駐させた後、材質のテクスチャを常駐させる4回目で失敗させる。
    let mut 失敗する供給元 = 検査用供給元::指定回で失敗する(4);
    let 新世代 = 構築する(
        &mut 失敗する供給元,
        台帳.次の世代idを発行する().unwrap(),
        余裕のある上限(),
        予約枠,
        &[材質を作る(2, Some(&素材))],
    );

    assert!(matches!(新世代, Err(レンダラーエラー::テクスチャblit非対応)));
    assert_eq!(失敗する供給元.生存枚数(), 0, "作りかけの画像を1枚も残さない");
    assert_eq!(失敗する供給元.退役枚数(), 3);
    assert_eq!(台帳.公開中().世代id().番号(), 公開前の世代番号);
    assert_eq!(台帳.公開中().画像枚数(), 公開前の画像枚数);
    assert_eq!(台帳.公開中().材質件数(), 1);
}
