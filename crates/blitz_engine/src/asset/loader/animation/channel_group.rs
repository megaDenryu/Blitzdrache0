//! 1アニメーション分のチャンネルをジョイント添字ごとへ振り分ける。

use crate::asset::error::アセットエラー;
use crate::asset::joint_channel::ジョイントアニメーションチャンネル;

use super::super::document::開いた文書;
use super::super::skin::関節解決;
use super::property;

pub(super) fn ジョイントチャンネル一覧を取り出す(
    文書: &開いた文書,
    アニメーション: &gltf::Animation<'_>,
    関節解決: &関節解決,
    ジョイント数: usize,
) -> Result<Vec<ジョイントアニメーションチャンネル>, アセットエラー> {
    let mut 一覧 = vec![ジョイントアニメーションチャンネル::空を作る(); ジョイント数];

    for チャンネル in アニメーション.channels() {
        let 対象ノード添字 = チャンネル.target().node().index();
        let Some(&ジョイント添字) = 関節解決.グローバルノードから新添字.get(&対象ノード添字) else {
            continue;
        };

        let 読み取り器 = チャンネル.reader(|バッファ| 文書.バッファ一覧.get(バッファ.index()).map(Vec::as_slice));
        property::プロパティを反映する(
            読み取り器,
            チャンネル.target().property(),
            チャンネル.sampler().interpolation(),
            &mut 一覧[ジョイント添字],
        )?;
    }

    Ok(一覧)
}
