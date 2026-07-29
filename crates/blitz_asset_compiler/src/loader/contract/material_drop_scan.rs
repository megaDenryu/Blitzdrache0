//! ローダーが読まないマテリアル宣言の検査。受け取るのはglTF文書、返すのは警告の一覧である。
//! 由来は`loader::material`がベースカラー・金属粗さ・法線マップと3つの係数だけを読み、それ以外の材質宣言を一度も参照しないことである。
//! 違反ではなく警告にするのは、これらの宣言を捨てても焼けて絵が出るためである。作者が諦めていれば運用は成立する。

use super::finding::契約指摘;
use super::target::{名前を写す, 対象位置};

pub(super) fn 検査する(文書: &gltf::Document) -> Vec<契約指摘> {
    let Some(材質) = 文書
        .meshes()
        .next()
        .and_then(|メッシュ| メッシュ.primitives().next())
        .map(|プリミティブ| プリミティブ.material())
    else {
        return Vec::new();
    };
    let 位置 = || 対象位置::マテリアル {
        名前: 名前を写す(材質.name()),
    };

    let mut 指摘一覧 = Vec::new();
    if 材質.emissive_factor() != [0.0, 0.0, 0.0] || 材質.emissive_texture().is_some() {
        指摘一覧.push(契約指摘::警告を作る(
            位置(),
            "発光(emissive)が宣言されている。ローダーは読まないため自ら光る見え方は絵に出ない",
            "発光で表していた明るさをベースカラーへ寄せるか、絵に出ないことを受け入れる",
        ));
    }
    if 材質.occlusion_texture().is_some() {
        指摘一覧.push(契約指摘::警告を作る(
            位置(),
            "遮蔽テクスチャ(occlusionTexture)が宣言されている。ローダーは読まないため、くぼみの暗さは絵に出ない",
            "遮蔽をベースカラーへ焼き込むか、絵に出ないことを受け入れる",
        ));
    }
    if 材質.alpha_mode() != gltf::material::AlphaMode::Opaque {
        指摘一覧.push(契約指摘::警告を作る(
            位置(),
            format!(
                "透過の扱いが{:?}で宣言されている。ローダーは読まないため不透明として描かれる",
                材質.alpha_mode()
            ),
            "透過を使わない形状へ作り替えるか、不透明として描かれることを受け入れる",
        ));
    }
    if !材質.double_sided() {
        指摘一覧.push(契約指摘::警告を作る(
            位置(),
            "片面表示(doubleSidedが偽)で宣言されている。レンダラーは面の裏表を捨てないため裏面も描かれる",
            "裏面が見えて困る形状なら面を閉じる。参照: crates/blitz_render/src/vulkan/pipeline/graphics_pipeline.rs のcull_mode",
        ));
    }
    指摘一覧
}
