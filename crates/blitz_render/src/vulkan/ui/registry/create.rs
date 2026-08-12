//! UIテクスチャ台帳のディスクリプタ資源を生成する。

use std::collections::HashMap;

use super::UIテクスチャレジストリ;
use crate::error::レンダラーエラー;
use crate::vulkan::ui::descriptor::UIテクスチャのディスクリプタ資源;

impl UIテクスチャレジストリ {
    pub(crate) fn 生成する(device: &ash::Device) -> Result<Self, レンダラーエラー> {
        Ok(Self {
            ディスクリプタ資源: UIテクスチャのディスクリプタ資源::確保する(device)?,
            表: HashMap::new(),
        })
    }
}
