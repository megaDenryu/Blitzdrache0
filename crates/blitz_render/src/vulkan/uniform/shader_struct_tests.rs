//! シェーダー側の宣言とCPU側のバイト詰めが同じ並びであることの検査。slangの原文を読み、
//! 各フィールドの開始位置と全体のバイト長を`layout_tests`が固定した数値と突き合わせる。
//! 2つのファイルはコメントでしか結ばれておらず、片方だけを直した食い違いは走らせても絵に出ないため、ここが機械的に見る。

use super::{cascade_bytes, sky_bytes, view_bytes};
use crate::vulkan::shader_struct::{シェーダー構造体の並び, 読み取る};

const ビュー定数の原文: &str = include_str!("../../../../../shaders/view_uniform.slang");
const 多段影定数の原文: &str = include_str!("../../../../../shaders/cascade_shadow_uniform.slang");
const 空パス定数の原文: &str = include_str!("../../../../../shaders/sky_pass_uniform.slang");

#[test]
fn ビュー定数の宣言がcpu側と同じ並びである() {
    let 並び = 読み取る一つ(ビュー定数の原文, "ViewUniform");
    assert_eq!(並び.バイト長, view_bytes::バイト長);
    開始位置を確かめる(&並び, "viewProjection", 0);
    開始位置を確かめる(&並び, "cameraRelativePosition", 64);
    開始位置を確かめる(&並び, "cameraForward", 80);
    開始位置を確かめる(&並び, "previousViewProjection", 96);
}

#[test]
fn 多段影定数の宣言がcpu側と同じ並びである() {
    let 並び = 読み取る一つ(多段影定数の原文, "CascadeShadowUniform");
    assert_eq!(並び.バイト長, cascade_bytes::バイト長);
    開始位置を確かめる(&並び, "cascadeLightViewProjection", 0);
    開始位置を確かめる(&並び, "cascadeSplitDepths", 256);
    開始位置を確かめる(&並び, "cascadeTransitionWidths", 272);
    開始位置を確かめる(&並び, "cascadeDebugAndTexelSize", 288);
}

#[test]
fn 空パス定数の宣言がcpu側と同じ並びである() {
    let 並び = 読み取る一つ(空パス定数の原文, "SkyPassUniform");
    assert_eq!(並び.バイト長, sky_bytes::バイト長);
    開始位置を確かめる(&並び, "inverseViewProjection", 0);
    開始位置を確かめる(&並び, "nightSkyRadianceAndSunCos", 64);
    開始位置を確かめる(&並び, "sunDirectionAndObserverAltitude", 80);
    開始位置を確かめる(&並び, "sunDiskRadianceAndSunIrradiance", 96);
}

fn 読み取る一つ(原文: &str, 構造体名: &str) -> シェーダー構造体の並び {
    match 読み取る(原文, 構造体名) {
        Ok(並び) => 並び,
        Err(誤り) => panic!("{構造体名}の宣言を読めない: {誤り}"),
    }
}

fn 開始位置を確かめる(並び: &シェーダー構造体の並び, 名前: &str, 期待する位置: usize) {
    match 並び.開始位置(名前) {
        Some(位置) => assert_eq!(位置, 期待する位置, "{名前}の開始位置がCPU側と食い違う"),
        None => panic!("{名前}がシェーダーの宣言に無い"),
    }
}
