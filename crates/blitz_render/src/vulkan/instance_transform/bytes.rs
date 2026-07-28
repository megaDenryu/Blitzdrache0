//! 個体変換をシェーダーのInstanceTransformと一致する112バイトへ変換する。
//! 描画対象ユニフォームの先頭112バイトもこの関数が書くため、単一個体の対象がユニフォームの先頭を個体変換として読める。

use super::content::個体変換内容;

pub(crate) const バイト長: usize = 112;

pub(crate) fn バイト列にする(内容: &個体変換内容) -> [u8; バイト長] {
    let mut バイト列 = [0u8; バイト長];
    let mut 位置 = 0usize;
    for 列 in 内容.ローカルからワールド {
        vec4を書き込む(&mut バイト列, &mut 位置, 列);
    }
    for 列 in 内容.法線ローカルからワールド {
        vec4を書き込む(&mut バイト列, &mut 位置, [列[0], 列[1], 列[2], 0.0]);
    }
    バイト列
}

fn vec4を書き込む(バイト列: &mut [u8; バイト長], 位置: &mut usize, 値: [f32; 4]) {
    for 成分 in 値 {
        バイト列[*位置..*位置 + 4].copy_from_slice(&成分.to_le_bytes());
        *位置 += 4;
    }
}
