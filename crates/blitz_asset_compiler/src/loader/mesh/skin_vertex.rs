//! JOINTS_0/WEIGHTS_0の抽出とジョイント添字のトポロジカル順への引き直し(判断42)。

use blitz_engine::スキン頂点属性;

use crate::error::アセットコンパイルエラー;

use super::super::document::開いた文書;
use super::super::skin::関節解決;

pub(super) fn プリミティブのスキン属性を取り出す(
    文書: &開いた文書,
    プリミティブ: &gltf::Primitive<'_>,
    関節解決: &関節解決,
    頂点数: usize,
) -> Result<Vec<スキン頂点属性>, アセットコンパイルエラー> {
    let 読み取り器 = プリミティブ.reader(|バッファ| 文書.バッファ一覧.get(バッファ.index()).map(Vec::as_slice));

    let 旧ジョイント一覧: Vec<[u16; 4]> = 読み取り器
        .read_joints(0)
        .ok_or(アセットコンパイルエラー::スキン頂点属性なし)?
        .into_u16()
        .collect();
    let ウェイト一覧: Vec<[f32; 4]> = 読み取り器
        .read_weights(0)
        .ok_or(アセットコンパイルエラー::スキン頂点属性なし)?
        .into_f32()
        .collect();

    if 旧ジョイント一覧.len() != 頂点数 || ウェイト一覧.len() != 頂点数 {
        return Err(アセットコンパイルエラー::解析失敗(
            "JOINTS_0/WEIGHTS_0の要素数が頂点数と一致しない".to_string(),
        ));
    }

    旧ジョイント一覧
        .into_iter()
        .zip(ウェイト一覧)
        .map(|(旧添字組, ウェイト)| 新添字組へ変換する(旧添字組, ウェイト, 関節解決))
        .collect()
}

fn 新添字組へ変換する(
    旧添字組: [u16; 4],
    ウェイト: [f32; 4],
    関節解決: &関節解決,
) -> Result<スキン頂点属性, アセットコンパイルエラー> {
    let mut 新添字組 = [0u16; 4];
    for (出力, &旧添字) in 新添字組.iter_mut().zip(旧添字組.iter()) {
        let 旧添字usize = usize::from(旧添字);
        let 新添字usize = *関節解決.旧添字から新添字.get(旧添字usize).ok_or_else(|| {
            アセットコンパイルエラー::解析失敗(format!("JOINTS_0の参照添字{旧添字usize}がジョイント数を超えている"))
        })?;
        *出力 = u16::try_from(新添字usize)
            .map_err(|誤り| アセットコンパイルエラー::解析失敗(format!("ジョイント添字がu16の範囲を超えている: {誤り}")))?;
    }
    Ok(スキン頂点属性 {
        ジョイント: 新添字組,
        ウェイト,
    })
}
