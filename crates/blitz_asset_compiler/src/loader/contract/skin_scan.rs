//! スキンとアニメーションの検査。受け取るのはglTF文書、返すのは指摘一覧である。
//! 由来は`loader::mesh::skin_vertex`がスキン付きプリミティブにJOINTS_0とWEIGHTS_0の両方を要求すること、
//! `loader::animation`がCUBICSPLINE補間を拒むこと、および`loader::mod`がスキンの無いメッシュではアニメーション一覧を空にすることである。

use super::finding::契約指摘;
use super::target::{名前を写す, 対象位置};

pub(super) fn 検査する(文書: &gltf::Document) -> Vec<契約指摘> {
    let スキンがある = 文書
        .nodes()
        .any(|ノード| ノード.mesh().is_some_and(|メッシュ| メッシュ.index() == 0) && ノード.skin().is_some());

    let mut 指摘一覧 = Vec::new();
    if スキンがある {
        スキン頂点属性を検査する(文書, &mut 指摘一覧);
    }
    アニメーションを検査する(文書, スキンがある, &mut 指摘一覧);
    指摘一覧
}

fn スキン頂点属性を検査する(文書: &gltf::Document, 指摘一覧: &mut Vec<契約指摘>) {
    let Some(メッシュ) = 文書.meshes().next() else {
        return;
    };
    for (添字, プリミティブ) in メッシュ.primitives().enumerate() {
        let 関節がある = プリミティブ.get(&gltf::Semantic::Joints(0)).is_some();
        let 重みがある = プリミティブ.get(&gltf::Semantic::Weights(0)).is_some();
        if 関節がある && 重みがある {
            continue;
        }
        指摘一覧.push(契約指摘::違反を作る(
            対象位置::プリミティブ {
                メッシュ添字: 0, 添字
            },
            "スキン付きのメッシュにJOINTS_0とWEIGHTS_0が揃っていない。ローダーは両方を要求する",
            "Blenderで全頂点をボーンへ重み付けしてから、書き出し設定でスキンを含める",
        ));
    }
}

fn アニメーションを検査する(文書: &gltf::Document, スキンがある: bool, 指摘一覧: &mut Vec<契約指摘>) {
    for (添字, アニメーション) in 文書.animations().enumerate() {
        let 位置 = || 対象位置::アニメーション {
            添字,
            名前: 名前を写す(アニメーション.name()),
        };
        if !スキンがある {
            指摘一覧.push(契約指摘::違反を作る(
                位置(),
                "スキンの無いメッシュにアニメーションが宣言されている。ローダーはスキンが無いとアニメーション一覧を空にするため、この動きは絵に出ない",
                "ボーンとスキンを付けてジョイントの動きとして書き出す。オブジェクト自身の移動・回転はエンジン側のシーン定義で与える",
            ));
            continue;
        }
        if アニメーション
            .channels()
            .any(|チャンネル| チャンネル.sampler().interpolation() == gltf::animation::Interpolation::CubicSpline)
        {
            指摘一覧.push(契約指摘::違反を作る(
                位置(),
                "CUBICSPLINE補間のチャンネルを含む。ローダーはCUBICSPLINEを未対応として拒む",
                "Blenderでキーフレームの補間を線形かステップにしてから書き出す",
            ));
        }
    }
}
