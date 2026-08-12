//! 8ビット4成分の画素の並びの判定と入れ替え。担当するのは、スワップチェーンの形式がBGRA並びかどうかを決めることと、
//! 赤と青を入れ替えて読み戻し画像のRGBA並びへ揃えることである。
//!
//! 読み出し元から分けるのは、どちらも論理デバイスもバッファも見ない純粋な計算であり、GPUの資源を1つも触らないためである。

use ash::vk;

pub(super) fn 赤と青を入れ替える(データ: &[u8]) -> Vec<u8> {
    let mut 結果 = データ.to_vec();
    for 画素 in 結果.chunks_exact_mut(4) {
        画素.swap(0, 2);
    }
    結果
}

pub(super) fn bgra形式か(形式: vk::Format) -> bool {
    matches!(
        形式,
        vk::Format::B8G8R8A8_UNORM | vk::Format::B8G8R8A8_SRGB | vk::Format::B8G8R8A8_SNORM | vk::Format::B8G8R8A8_UINT | vk::Format::B8G8R8A8_SINT
    )
}
