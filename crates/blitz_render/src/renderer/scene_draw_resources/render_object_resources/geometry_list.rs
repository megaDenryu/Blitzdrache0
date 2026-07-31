//! 1つの描画対象が持つ詳細段ごとのジオメトリバッファ。触れるのは自分が確保したバッファだけで、段の選択は保持している列から1本を返す読み取りに閉じる。
//! 全段を生成時にまとめて確保するため、段の選択がディスクI/Oも確保も解放も起こさない(参照: `_doc/設計/地形とカメラ相対描画.md`「LOD」)。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::lod_mesh::メッシュ素材;
use crate::vulkan;
use crate::vulkan::tracked_device::GPUデバイス;
use crate::vulkan::transfer::転送実行環境;

pub(super) struct 段別ジオメトリ {
    段一覧: Vec<vulkan::geometry::ジオメトリバッファ>,
}

impl 段別ジオメトリ {
    /// 途中で失敗したときは確保済みの段をすべて解放してからエラーを返すため、呼び出し元に半分だけ確保された列が渡らない。
    pub(super) fn 生成する(
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        転送環境: &転送実行環境,
        メッシュ列: &[メッシュ素材],
    ) -> Result<Self, レンダラーエラー> {
        let mut 段一覧 = Vec::with_capacity(メッシュ列.len());
        for メッシュ in メッシュ列 {
            let 生成結果 = vulkan::geometry::ジオメトリバッファ::生成する(
                device,
                メモリプロパティ,
                転送環境,
                メッシュ.頂点一覧(),
                メッシュ.インデックス一覧(),
            );
            match 生成結果 {
                Ok(バッファ) => 段一覧.push(バッファ),
                Err(誤り) => {
                    for バッファ in &段一覧 {
                        バッファ.破棄する(device);
                    }
                    return Err(誤り);
                }
            }
        }
        Ok(Self { 段一覧 })
    }

    /// 段番号で参照するジオメトリ。焼かれた段数を超える番号は最も粗い段を返す。距離が最も粗い段の閾値より遠いという意味であり、
    /// そのとき最も粗い段を描くのが要求どおりの結果だからである。段番号はそのまま列の添字である。
    pub(super) fn 段番号で選ぶ(&self, 段番号: usize) -> &vulkan::geometry::ジオメトリバッファ {
        match self.段一覧.get(self.有効な段番号(段番号)) {
            Some(バッファ) => バッファ,
            None => panic!("段別ジオメトリは1段以上を持つ不変条件に違反した"),
        }
    }

    /// 焼かれた段数の内側へ丸めた段番号。ジオメトリの選択とプリミティブの絞り込みが同じ段を指すよう、丸めの規則はここだけが持つ。
    pub(super) fn 有効な段番号(&self, 段番号: usize) -> usize {
        match self.段一覧.len().checked_sub(1) {
            Some(最大番号) => 段番号.min(最大番号),
            None => panic!("段別ジオメトリは1段以上を持つ不変条件に違反した"),
        }
    }

    pub(super) fn 破棄する(&self, device: &GPUデバイス) {
        for バッファ in &self.段一覧 {
            バッファ.破棄する(device);
        }
    }
}
