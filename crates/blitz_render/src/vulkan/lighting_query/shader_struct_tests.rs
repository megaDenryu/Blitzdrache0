//! シェーダー側の宣言とCPU側のバイト詰めが同じ並びであることの検査。slangの原文を読み、
//! 各フィールドの開始位置と全体のバイト長を`layout_tests`が固定した数値と突き合わせる。
//! 2つのファイルはコメントでしか結ばれておらず、片方だけを直した食い違いは走らせても絵に出ないため、ここが機械的に見る。

use super::{directional_bytes, header_bytes, local_bytes};
use crate::vulkan::shader_struct::{シェーダー構造体の並び, 読み取る};

const 照明問い合わせの原文: &str = include_str!("../../../../../shaders/lighting_query.slang");

#[test]
fn ヘッダの宣言がcpu側と同じ並びである() {
    let 並び = 読み取る一つ("LightingQueryHeader");
    assert_eq!(並び.バイト長, header_bytes::バイト長);
    開始位置を確かめる(&並び, "directionalLightCount", 0);
    開始位置を確かめる(&並び, "localLightCount", 4);
    開始位置を確かめる(&並び, "lightingEnabled", 8);
    開始位置を確かめる(&並び, "constantIndirectFactor", 12);
    開始位置を確かめる(&並び, "sunDirection", 16);
}

#[test]
fn 方向光レコードの宣言がcpu側と同じ並びである() {
    let 並び = 読み取る一つ("DirectionalLightRecord");
    assert_eq!(並び.バイト長, directional_bytes::バイト長);
    開始位置を確かめる(&並び, "directionToLightAndIntensity", 0);
    開始位置を確かめる(&並び, "color", 16);
    開始位置を確かめる(&並び, "visibility", 32);
}

#[test]
fn 局所光レコードの宣言がcpu側と同じ並びである() {
    let 並び = 読み取る一つ("LocalLightRecord");
    assert_eq!(並び.バイト長, local_bytes::バイト長);
    開始位置を確かめる(&並び, "cameraRelativePosition", 0);
    開始位置を確かめる(&並び, "colorAndIntensity", 16);
    開始位置を確かめる(&並び, "kind", 32);
}

fn 読み取る一つ(構造体名: &str) -> シェーダー構造体の並び {
    match 読み取る(照明問い合わせの原文, 構造体名) {
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
