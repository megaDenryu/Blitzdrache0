//! 検収が与える解析入力をGPUの3つの画像へ載せる資源。触れるのは3本のホスト可視バッファと、そのフレームへ
//! 渡す転送の材料だけであり、焼き上げの計画にも描画にも触れない。
//!
//! 大気から焼く経路と同居させず別の資源にするのは、この経路が本番のフレームで1度も作られないためである。
//! 注入がある実行では焼き上げの計画が「何も焼かない」に固定され、生成パスが1本も積まれない代わりに
//! この転送が毎フレーム3本積まれる(参照: `_doc/設計/放射輝度問い合わせ階層.md`「3-Icの消費式と実装段割り」)。
//!
//! 注意: 転送を毎フレーム積むのは、初回のレイアウト遷移を確実に起こすためでもある。ディスクリプタはGENERALで
//! 結んであるが、画像が実際にそのレイアウトへ移るのはグラフが用途を宣言されたパスを積んだフレームだけである。
//! 注入を初回だけにすると、以降のフレームで画像が「中身を保つ」扱いに変わる境界を別に守らねばならなくなる。

mod bytes;
mod create;
mod upload_buffer;

use ash::vk;

use upload_buffer::注入元バッファ;

use crate::vulkan::derived_environment::派生表現一式;
use crate::vulkan::tracked_device::GPUデバイス;

/// 注入の転送が使う元バッファと段ごとの範囲。1フレームの積み上げが名前で受け取る。
#[derive(Clone)]
pub(crate) struct 解析入力の注入 {
    pub(crate) 拡散照度の元: vk::Buffer,
    pub(crate) 鏡面畳込みの元: vk::Buffer,
    pub(crate) 反射率積分表の元: vk::Buffer,
    /// 鏡面畳込みの段ごとの範囲。並びは段番号の昇順であり、バイト列の並びと同じである。
    pub(crate) 鏡面畳込みの段ごとの範囲: Vec<vk::Extent3D>,
    pub(crate) 拡散照度の範囲: vk::Extent3D,
    pub(crate) 反射率積分表の範囲: vk::Extent3D,
    pub(crate) 層数: u32,
}

pub(crate) struct 解析入力の注入資源 {
    pub(super) 拡散照度: 注入元バッファ,
    pub(super) 鏡面畳込み: 注入元バッファ,
    pub(super) 反射率積分表: 注入元バッファ,
}

impl 解析入力の注入資源 {
    pub(crate) fn 生成する(
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        派生表現: &派生表現一式,
        入力: &crate::distant_environment::遠方環境の解析入力,
    ) -> Result<Self, crate::error::レンダラーエラー> {
        create::生成する(device, メモリプロパティ, 派生表現, 入力)
    }

    /// そのフレームの転送の材料。範囲は焼いた画像の解像度が答えるため、注入する側が段の一辺を別に持たない。
    pub(crate) fn 転送の材料を作る(&self, 派生表現: &派生表現一式) -> 解析入力の注入 {
        let 鏡面 = 派生表現.鏡面畳込み画像();
        解析入力の注入 {
            拡散照度の元: self.拡散照度.handle,
            鏡面畳込みの元: self.鏡面畳込み.handle,
            反射率積分表の元: self.反射率積分表.handle,
            鏡面畳込みの段ごとの範囲: (0..鏡面.段数()).map(|段| 鏡面.段の範囲(段)).collect(),
            拡散照度の範囲: 派生表現.拡散照度画像().段の範囲(0),
            反射率積分表の範囲: 派生表現.反射率積分表の画像().範囲(),
            層数: 鏡面.層数(),
        }
    }

    /// 前提: レンダラー全体の破棄順は renderer/destroy.rs が持ち、この資源は遠方環境の照明資源の1段として呼ばれる(GPU待機済み)。
    pub(crate) fn 破棄する(&self, device: &GPUデバイス) {
        self.反射率積分表.破棄する(device);
        self.鏡面畳込み.破棄する(device);
        self.拡散照度.破棄する(device);
    }
}
