//! レイアウトからディスクリプタセットを取り出す局面。呼ばれるのは照明問い合わせ資源束の生成時と、
//! 資源表世代を作り直すたびの2つであり、レイアウトそのものは1つも変えない。
//!
//! 保持と破棄から分けるのは、呼ばれる時点と回数が違うためである。レイアウトはレンダラーと同じ寿命で1度だけ作られ、
//! セットの取り出しは世代の作り直しごとに繰り返される。

use ash::vk;

use super::シーンセットレイアウト一式;
use crate::error::レンダラーエラー;
use crate::vulkan::descriptor::{材質の割り当て済みセット, 照明問い合わせの割り当て済みセット};

impl シーンセットレイアウト一式 {
    /// 進行中フレームスロットごとの照明問い合わせのセットを、照明問い合わせ資源束が持つプールから取り出す。
    pub(crate) fn 照明問い合わせのセットを割り当てる(
        &self,
        device: &ash::Device,
        pool: vk::DescriptorPool,
        セット数: usize,
    ) -> Result<Vec<照明問い合わせの割り当て済みセット>, レンダラーエラー> {
        let 一覧 = crate::vulkan::descriptor::alloc::割り当てる(device, pool, self.照明問い合わせ, セット数)?;
        Ok(一覧.into_iter().map(照明問い合わせの割り当て済みセット::刻む).collect())
    }

    /// 資源表世代1つぶんの材質のセットを、その世代専用のプールから1つだけ取り出す。
    pub(crate) fn 材質のセットを1つ割り当てる(
        &self,
        device: &ash::Device,
        pool: vk::DescriptorPool,
    ) -> Result<材質の割り当て済みセット, レンダラーエラー> {
        let 一覧 = crate::vulkan::descriptor::alloc::割り当てる(device, pool, self.材質, 1)?;
        match 一覧.first().copied() {
            Some(セット) => Ok(材質の割り当て済みセット::刻む(セット)),
            None => panic!("材質のセットを1つ要求したのに1つも返らなかった"),
        }
    }
}
