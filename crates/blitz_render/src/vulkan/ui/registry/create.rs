//! UIテクスチャ台帳のディスクリプタ資源を生成する。

use std::collections::HashMap;

use super::UIテクスチャレジストリ;
use crate::error::レンダラーエラー;
use crate::vulkan::ui::descriptor;

impl UIテクスチャレジストリ {
    pub(crate) fn 生成する(device: &ash::Device) -> Result<Self, レンダラーエラー> {
        let layout = descriptor::layoutを生成する(device)?;
        let pool = match descriptor::poolを生成する(device) {
            Ok(pool) => pool,
            Err(誤り) => {
                // 安全性: layoutはこのスコープの唯一の所有者で、以降使用しない。
                unsafe { device.destroy_descriptor_set_layout(layout, None) };
                return Err(誤り);
            }
        };
        Ok(Self {
            layout,
            pool,
            表: HashMap::new(),
        })
    }
}
