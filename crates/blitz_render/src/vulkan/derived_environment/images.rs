//! 派生表現の3つの画像をまとめて生成し、まとめて破棄する組。触れる状態はこの3つの画像だけであり、
//! 不変条件は「3つがそろって存在するか1つも存在しないかのどちらかである」ことである。
//!
//! 3つを1つの組にするのは、途中で失敗したときの巻き戻しを1箇所へ閉じるためである。画像・ディスクリプタ・
//! パイプラインを1つの関数で順に作ると、後段の失敗ごとに前段の片付けが入れ子になって深くなる。

use super::create::派生表現の解像度一式;
use super::cube_image::派生の立方体画像;
use super::table_image::反射率積分表の画像;
use crate::error::レンダラーエラー;
use crate::vulkan::allocator::GPU資源の確保係;
use crate::vulkan::tracked_device::GPUデバイス;

pub(super) struct 派生表現の画像三点 {
    pub(super) 拡散照度: 派生の立方体画像,
    pub(super) 鏡面畳込み: 派生の立方体画像,
    pub(super) 反射率積分表: 反射率積分表の画像,
}

impl 派生表現の画像三点 {
    pub(super) fn 生成する(
        確保係: &GPU資源の確保係<'_>, 解像度: 派生表現の解像度一式
    ) -> Result<Self, レンダラーエラー> {
        let device = 確保係.論理デバイス();
        let 拡散照度 = 派生の立方体画像::生成する(確保係, 解像度.拡散照度.面の一辺().値(), 1)?;
        let 鏡面 = 解像度.鏡面畳込み;
        let 最詳細段の一辺 = 鏡面.段の一辺(最詳細段(鏡面)).値();
        let 鏡面畳込み = match 派生の立方体画像::生成する(確保係, 最詳細段の一辺, 鏡面.粗さ段数()) {
            Ok(画像) => 画像,
            Err(誤り) => {
                拡散照度.破棄する(device);
                return Err(誤り);
            }
        };
        let 表 = 反射率積分表の画像::生成する(確保係, 解像度.反射率積分表.横(), 解像度.反射率積分表.縦());
        match 表 {
            Ok(反射率積分表) => Ok(Self {
                拡散照度,
                鏡面畳込み,
                反射率積分表,
            }),
            Err(誤り) => {
                鏡面畳込み.破棄する(device);
                拡散照度.破棄する(device);
                Err(誤り)
            }
        }
    }

    pub(super) fn 破棄する(&self, device: &GPUデバイス) {
        self.反射率積分表.破棄する(device);
        self.鏡面畳込み.破棄する(device);
        self.拡散照度.破棄する(device);
    }
}

fn 最詳細段(鏡面: crate::distant_environment::derived::鏡面畳込みの解像度) -> crate::distant_environment::derived::粗さ段 {
    let 全段 = 鏡面.全段();
    let Some(&段) = 全段.first() else {
        panic!("鏡面畳込みの解像度が段を1つも持たない(生成の値域検証が破れている)");
    };
    段
}
