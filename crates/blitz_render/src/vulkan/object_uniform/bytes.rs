//! 描画対象ユニフォームをシェーダーのObjectUniformと一致する144バイトへ変換する。
//! 先頭112バイトは個体変換と同じ並びであり、その組み立ては`instance_transform::bytes`が持つ。

use super::super::instance_transform::bytes as 個体変換バイト;
use super::content::描画対象ユニフォーム内容;

pub(super) const バイト長: usize = 144;

pub(super) fn バイト列にする(内容: &描画対象ユニフォーム内容) -> [u8; バイト長] {
    let mut バイト列 = [0u8; バイト長];
    バイト列[..個体変換バイト::バイト長].copy_from_slice(&個体変換バイト::バイト列にする(&内容.変換));
    let mut 位置 = 個体変換バイト::バイト長;
    vec4を書き込む(&mut バイト列, &mut 位置, 内容.ベースカラー係数);
    vec4を書き込む(&mut バイト列, &mut 位置, [内容.金属粗さ係数[0], 内容.金属粗さ係数[1], 0.0, 0.0]);
    バイト列
}

fn vec4を書き込む(バイト列: &mut [u8; バイト長], 位置: &mut usize, 値: [f32; 4]) {
    for 成分 in 値 {
        バイト列[*位置..*位置 + 4].copy_from_slice(&成分.to_le_bytes());
        *位置 += 4;
    }
}
