//! 大気LUT標本ディスクリプタの生成局面。サンプラー・レイアウト・プール・セットを順に作り、束縛先を書き込む。
//! 呼ばれるのは空パイプラインを組み立てる直前の1回だけであり、以降のフレームはセットを引くだけである。
//! 途中で失敗したら、それまでに作ったハンドルをその場で逆順に片付ける。

use ash::vk;

use super::{binding, 大気LUT標本の束縛先, 大気LUT標本ディスクリプタ};
use crate::error::レンダラーエラー;
use crate::vulkan::atmosphere_lut::descriptor_common;
use crate::vulkan::linear_sampler;
use crate::vulkan::sync::フレームスロット添字;

pub(super) fn 生成する(
    device: &ash::Device,
    束縛先: &大気LUT標本の束縛先,
) -> Result<大気LUT標本ディスクリプタ, レンダラーエラー> {
    let sampler = linear_sampler::作る(device)?;
    let layout = match binding::レイアウトを作る(device) {
        Ok(layout) => layout,
        Err(誤り) => return Err(サンプラーを片付けて返す(device, sampler, 誤り)),
    };
    let pool = match binding::プールを作る(device) {
        Ok(pool) => pool,
        Err(誤り) => {
            descriptor_common::途中の資源を片付ける(device, layout, None);
            return Err(サンプラーを片付けて返す(device, sampler, 誤り));
        }
    };
    let set一覧 = match descriptor_common::セットを割り当てる(device, pool, layout) {
        Ok(set一覧) => set一覧,
        Err(誤り) => {
            descriptor_common::途中の資源を片付ける(device, layout, Some(pool));
            return Err(サンプラーを片付けて返す(device, sampler, 誤り));
        }
    };
    for 添字 in フレームスロット添字::全スロット() {
        binding::書き込む(device, set一覧[添字.配列添字()], sampler, 束縛先, 添字);
    }
    Ok(大気LUT標本ディスクリプタ {
        layout,
        pool,
        sampler,
        set一覧,
    })
}

fn サンプラーを片付けて返す(device: &ash::Device, sampler: vk::Sampler, 誤り: レンダラーエラー) -> レンダラーエラー {
    // 安全性: samplerはこのスコープの唯一の所有者で、以降使用しない。
    unsafe { device.destroy_sampler(sampler, None) };
    誤り
}
