//! 布シミュのコンピュートパイプライン9本(判断54)。全本が同じディスクリプタレイアウトを
//! 共有し、プッシュ定数は無い(定数はb0のUBOで渡す)。

use ash::vk;

use crate::cloth_shader_set::布シェーダー一式;
use crate::error::レンダラーエラー;
use crate::vulkan::allocator::GPU資源の確保係;

pub(super) struct 布パイプライン群 {
    pub(super) layout: vk::PipelineLayout,
    pub(super) 介入: vk::Pipeline,
    pub(super) 積分: vk::Pipeline,
    pub(super) アタッチ: vk::Pipeline,
    pub(super) 拘束: vk::Pipeline,
    pub(super) ハッシュ消去: vk::Pipeline,
    pub(super) ハッシュ格納: vk::Pipeline,
    pub(super) 分離: vk::Pipeline,
    pub(super) 仕上げ: vk::Pipeline,
    pub(super) 頂点生成: vk::Pipeline,
}

pub(super) fn 布パイプライン群を生成する(
    確保係: &GPU資源の確保係<'_>,
    ディスクリプタlayout: vk::DescriptorSetLayout,
    シェーダー: &布シェーダー一式,
) -> Result<布パイプライン群, レンダラーエラー> {
    let device = 確保係.論理デバイス();
    let ディスクリプタlayout一覧 = [ディスクリプタlayout];
    let layout_info = vk::PipelineLayoutCreateInfo::default().set_layouts(&ディスクリプタlayout一覧);
    // 安全性: deviceは生成済みで有効。layout_infoは本関数内で構築した値のみを参照する。
    let layout = unsafe { device.create_pipeline_layout(&layout_info, None)? };

    let 仕様一覧: [(&[u8], &std::ffi::CStr); 9] = [
        (シェーダー.介入.コード(), c"interventionMain"),
        (シェーダー.積分.コード(), c"integrateMain"),
        (シェーダー.アタッチ.コード(), c"attachMain"),
        (シェーダー.拘束.コード(), c"constraintMain"),
        (シェーダー.ハッシュ消去.コード(), c"hashClearMain"),
        (シェーダー.ハッシュ格納.コード(), c"hashStoreMain"),
        (シェーダー.分離.コード(), c"separateMain"),
        (シェーダー.仕上げ.コード(), c"finishMain"),
        (シェーダー.頂点生成.コード(), c"vertexGenMain"),
    ];
    let mut 生成済み: Vec<vk::Pipeline> = Vec::with_capacity(仕様一覧.len());
    for (spirv, エントリ名) in 仕様一覧 {
        match 確保係.コンピュートパイプラインを生成する(layout, spirv, エントリ名) {
            Ok(handle) => 生成済み.push(handle),
            Err(誤り) => {
                // 安全性: 生成済みパイプラインとlayoutはこのスコープの唯一の所有者で、以降使用しない。
                unsafe {
                    for &handle in &生成済み {
                        device.destroy_pipeline(handle, None);
                    }
                    device.destroy_pipeline_layout(layout, None);
                }
                return Err(誤り);
            }
        }
    }
    Ok(布パイプライン群 {
        layout,
        介入: 生成済み[0],
        積分: 生成済み[1],
        アタッチ: 生成済み[2],
        拘束: 生成済み[3],
        ハッシュ消去: 生成済み[4],
        ハッシュ格納: 生成済み[5],
        分離: 生成済み[6],
        仕上げ: 生成済み[7],
        頂点生成: 生成済み[8],
    })
}

impl 布パイプライン群 {
    pub(super) fn 破棄する(&self, device: &ash::Device) {
        // 安全性: 各ハンドルはSelfが唯一の所有者であり、破棄時点でGPU側の使用がdevice_wait_idle済みであることを呼び出し元が保証する。
        unsafe {
            for handle in [
                self.介入,
                self.積分,
                self.アタッチ,
                self.拘束,
                self.ハッシュ消去,
                self.ハッシュ格納,
                self.分離,
                self.仕上げ,
                self.頂点生成,
            ] {
                device.destroy_pipeline(handle, None);
            }
            device.destroy_pipeline_layout(self.layout, None);
        }
    }
}
