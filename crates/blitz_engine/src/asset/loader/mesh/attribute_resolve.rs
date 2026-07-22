//! 未確定頂点属性(NORMAL/TANGENT欠如の可能性あり)を最終的なメッシュ頂点属性へ解決する
//! (判断46)。複数プリミティブ連結後の全体に対して1回だけ計算する。

use crate::asset::vertex_attribute::メッシュ頂点属性;

use super::normal_fill::法線を確定する;
use super::primitive::未確定頂点属性;
use super::tangent_fill::接線を確定する;

pub(super) fn 頂点属性を確定する(
    頂点一覧: Vec<未確定頂点属性>, インデックス一覧: &[u32]
) -> Vec<メッシュ頂点属性> {
    let 頂点数 = 頂点一覧.len();
    let mut 位置一覧 = Vec::with_capacity(頂点数);
    let mut 法線一覧 = Vec::with_capacity(頂点数);
    let mut 接線一覧 = Vec::with_capacity(頂点数);
    let mut uv一覧 = Vec::with_capacity(頂点数);
    for 属性 in 頂点一覧 {
        位置一覧.push(属性.位置);
        法線一覧.push(属性.法線);
        接線一覧.push(属性.接線);
        uv一覧.push(属性.uv);
    }

    let 確定法線一覧 = 法線を確定する(&位置一覧, インデックス一覧, 法線一覧);
    let 確定接線一覧 = 接線を確定する(&確定法線一覧, 接線一覧);

    位置一覧
        .into_iter()
        .zip(確定法線一覧)
        .zip(確定接線一覧)
        .zip(uv一覧)
        .map(|(((位置, 法線), 接線), uv)| メッシュ頂点属性 { 位置, 法線, 接線, uv })
        .collect()
}
