//! 1つの描画対象が材質スロットごとに持つGPU資源の一覧。触れるのはスロット番号とその材質のテクスチャの対応だけであり、
//! 段の選択も可視選別も知らない。スロット1つぶんの確保と解放は`entry`が持つ。
//!
//! スロット番号で添字を引けるようにするのは、プリミティブ描画発行が原型の語彙である材質スロット番号で材質を指すためである。
//! 番号を並びの添字で代用しないのは、未使用スロットを許す契約のもとで番号が飛びうるためである。
//! 引いた添字は材質レコード列の添字でもあるため、この並びと材質レコードの並びは同じ順でなければならない。
//! 参照: `_doc/設計/マルチマテリアルと材質境界.md`「束縛バックエンドの移行境界」

mod entry;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::render_object_material::材質スロット素材一覧;
use crate::vulkan::gpu_environment::物理デバイス問い合わせ;
use crate::vulkan::tracked_device::GPUデバイス;
use crate::vulkan::transfer::転送実行環境;

pub(super) use entry::スロット材質資源;

pub(super) struct スロット別材質資源 {
    一覧: Vec<スロット材質資源>,
}

impl スロット別材質資源 {
    /// 途中で失敗したときは確保済みのスロットをすべて解放してからエラーを返すため、呼び出し元に半分だけ確保された一覧が渡らない。
    pub(super) fn 生成する(
        問い合わせ: 物理デバイス問い合わせ<'_>,
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        転送環境: &転送実行環境,
        素材一覧: &材質スロット素材一覧,
    ) -> Result<Self, レンダラーエラー> {
        let mut 一覧 = Vec::with_capacity(素材一覧.件数());
        for 素材 in 素材一覧.一覧() {
            let 生成結果 = スロット材質資源::生成する(問い合わせ, device, メモリプロパティ, 転送環境, 素材);
            match 生成結果 {
                Ok(資源) => 一覧.push(資源),
                Err(誤り) => {
                    for 済み in &一覧 {
                        済み.破棄する(device);
                    }
                    return Err(誤り);
                }
            }
        }
        Ok(Self { 一覧 })
    }

    pub(super) fn 件数(&self) -> usize {
        self.一覧.len()
    }

    /// そのスロット番号を持つ材質の添字。持たなければ`None`。走査が線形なのは、件数が1つの原型が持つ材質スロットの数であり、
    /// 作者が1つのメッシュへ割り当てた材質の数と同じ桁に収まるためである。
    pub(super) fn スロット番号で添字を引く(&self, スロット番号: u32) -> Option<usize> {
        self.一覧.iter().position(|資源| 資源.スロット番号() == スロット番号)
    }

    pub(super) fn 添字で参照する(&self, 添字: usize) -> Option<&スロット材質資源> {
        self.一覧.get(添字)
    }

    pub(super) fn 破棄する(&self, device: &GPUデバイス) {
        for 資源 in &self.一覧 {
            資源.破棄する(device);
        }
    }
}
