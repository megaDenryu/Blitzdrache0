//! 世代の解決の検査。段4aの(iv)解決結果が世代型付きであり異世代の混在を拒むこと、
//! (v)重複した材質idと未知の材質idを型付きの失敗にすることを固定する。

#![allow(clippy::unwrap_used)]

use crate::error::{レンダラーエラー, 材質資源表エラー};
use crate::texture_material::テクスチャ用途;
use crate::vulkan::material_table::generation_build::構築する;
use crate::vulkan::material_table::generation_id::資源表世代ID;
use crate::vulkan::material_table::{material_id::大域材質ID, stage_reserve::画素段の予約枠};

use super::fixture::{余裕のある上限, 材質を作る, 検査用供給元, 検査用素材};

#[test]
fn 解決結果は世代型付きであり異世代の参照を拒む() {
    let 素材 = 検査用素材(テクスチャ用途::色);
    let mut 供給元 = 検査用供給元::常に成功する();
    let 予約枠 = 画素段の予約枠::現行のシーン画素段();
    let 世代1 = 構築する(&mut 供給元, 資源表世代ID::最初(), 余裕のある上限(), 予約枠, &[材質を作る(1, Some(&素材))]).unwrap();
    let 世代2 = 構築する(
        &mut 供給元,
        資源表世代ID::最初().次を作る().unwrap(),
        余裕のある上限(),
        予約枠,
        &[材質を作る(1, Some(&素材))],
    )
    .unwrap();

    let 参照 = 世代1.解決する(大域材質ID::生成する(1)).unwrap();
    assert_eq!(参照.世代id(), 世代1.世代id());
    assert!(世代1.描画へ渡すレコード添字(参照).is_ok());
    assert_eq!(
        世代2.描画へ渡すレコード添字(参照).unwrap_err(),
        材質資源表エラー::異世代の混在 {
            参照の世代: 世代1.世代id().番号(),
            束縛する世代: 世代2.世代id().番号(),
        }
    );
}

#[test]
fn 重複した材質idと未知の材質idを型付きの失敗にする() {
    let 素材 = 検査用素材(テクスチャ用途::色);
    let mut 供給元 = 検査用供給元::常に成功する();
    let 予約枠 = 画素段の予約枠::現行のシーン画素段();
    let 重複 = 構築する(
        &mut 供給元,
        資源表世代ID::最初(),
        余裕のある上限(),
        予約枠,
        &[材質を作る(3, Some(&素材)), 材質を作る(3, None)],
    );
    assert!(matches!(
        重複,
        Err(レンダラーエラー::材質資源表不正(材質資源表エラー::材質IDの重複 {
            材質id: 3
        }))
    ));

    let 世代 = 構築する(&mut 供給元, 資源表世代ID::最初(), 余裕のある上限(), 予約枠, &[材質を作る(3, None)]).unwrap();
    assert_eq!(
        世代.解決する(大域材質ID::生成する(9)).unwrap_err(),
        材質資源表エラー::未知の材質ID { 材質id: 9 }
    );
}
