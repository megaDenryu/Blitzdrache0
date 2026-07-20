//! スキン読込: メッシュを持つノードのスキンをトポロジカル順に並べ替えて取り出す(判断42)。

mod inverse_bind;
mod joint_order;

use std::collections::HashMap;

use blitz_math::変換;

use crate::asset::error::アセットエラー;
use crate::asset::joint::ジョイント;
use crate::asset::skin_data::スキンデータ;
use crate::asset::static_trs::静的TRS;

use super::document::開いた文書;

/// スキン読込で得られるジョイント添字の対応表。JOINTS_0の引き直し(mesh)と
/// アニメーション対象ノードの解決(animation)の両方から参照する。
pub(super) struct 関節解決 {
    pub(super) 旧添字から新添字: Vec<usize>,
    pub(super) グローバルノードから新添字: HashMap<usize, usize>,
}

/// `対象メッシュ添字`を持つノードのスキンを読み込む。スキンが無ければ`None`。
pub(super) fn スキンを取り出す(
    文書: &開いた文書,
    対象メッシュ添字: usize,
) -> Result<Option<(スキンデータ, 関節解決)>, アセットエラー> {
    let Some(スキン) = メッシュを持つノードのスキン(文書, 対象メッシュ添字) else {
        return Ok(None);
    };

    let 親ノード添字表 = joint_order::親ノード添字表を作る(&文書.document);
    let 旧ジョイントノード一覧: Vec<gltf::Node<'_>> = スキン.joints().collect();
    let 旧グローバル一覧: Vec<usize> = 旧ジョイントノード一覧.iter().map(gltf::Node::index).collect();
    let 並べ替え = joint_order::トポロジカル順を求める(&旧グローバル一覧, &親ノード添字表);
    let ジョイント数 = 旧グローバル一覧.len();
    let 逆バインド一覧 = inverse_bind::逆バインド行列一覧を読む(文書, &スキン, ジョイント数)?;

    let mut ジョイント一覧 = Vec::with_capacity(ジョイント数);
    for 新添字 in 0..ジョイント数 {
        let 旧添字 = 並べ替え.新から旧[新添字];
        let 親新添字 = 並べ替え.旧親添字一覧[旧添字].map(|親旧添字| 並べ替え.旧から新[親旧添字]);
        ジョイント一覧.push(ジョイント {
            親添字: 親新添字,
            逆バインド行列: 変換::列優先配列から生成する(逆バインド一覧[旧添字]),
            バインド時: バインド時姿勢を読む(&旧ジョイントノード一覧[旧添字]),
        });
    }

    let グローバルノードから新添字 = 旧グローバル一覧
        .iter()
        .enumerate()
        .map(|(旧添字, &グローバル)| (グローバル, 並べ替え.旧から新[旧添字]))
        .collect();

    Ok(Some((
        スキンデータ { ジョイント一覧 },
        関節解決 {
            旧添字から新添字: 並べ替え.旧から新,
            グローバルノードから新添字,
        },
    )))
}

/// ノードの静的ローカル姿勢(判断49)。行列指定のノードでも`Transform::decomposed`が
/// TRSへ分解するため、指定方式によらず一様に扱える。
fn バインド時姿勢を読む(ノード: &gltf::Node<'_>) -> 静的TRS {
    let (平行移動, 回転, スケール) = ノード.transform().decomposed();
    静的TRS {
        平行移動,
        回転,
        スケール,
    }
}

fn メッシュを持つノードのスキン(文書: &開いた文書, 対象メッシュ添字: usize) -> Option<gltf::Skin<'_>> {
    文書.document.nodes().find_map(|ノード| {
        let ノードのメッシュ添字 = ノード.mesh()?.index();
        if ノードのメッシュ添字 == 対象メッシュ添字 {
            ノード.skin()
        } else {
            None
        }
    })
}
