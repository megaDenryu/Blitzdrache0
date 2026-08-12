//! 遠方環境ディスクリプタの生成局面。サンプラー・レイアウト・プール・セットを順に作り、束縛先を書き込む。
//! 呼ばれるのは遠方環境一式の組み立て時の1回だけであり、以降のフレームはセットを参照するだけである。
//! 途中で失敗したら、それまでに作ったハンドルをその場で逆順に片付ける。

use ash::vk;

use super::{binding, 遠方環境の束縛先, 遠方環境ディスクリプタ};
use crate::error::レンダラーエラー;
use crate::vulkan::allocator::GPU資源の確保係;
use crate::vulkan::sync::フレームスロット添字;

pub(super) fn 生成する(
    確保係: &GPU資源の確保係<'_>,
    束縛先: 遠方環境の束縛先<'_>,
) -> Result<遠方環境ディスクリプタ, レンダラーエラー> {
    let device = 確保係.論理デバイス();
    let sampler = 確保係.線形サンプラーを作る()?;
    let layout = match binding::レイアウトを作る(device) {
        Ok(layout) => layout,
        Err(誤り) => return Err(サンプラーを片付けて返す(device, sampler, 誤り)),
    };
    let pool = match binding::プールを作る(device) {
        Ok(pool) => pool,
        Err(誤り) => {
            layout.破棄する(device);
            return Err(サンプラーを片付けて返す(device, sampler, 誤り));
        }
    };
    let set一覧 = match layout.進行中フレームスロットごとのセットを割り当てる(device, pool) {
        Ok(set一覧) => set一覧,
        Err(誤り) => {
            // 安全性: poolはこのスコープの唯一の所有者で、以降使用しない。
            unsafe { device.destroy_descriptor_pool(pool, None) };
            layout.破棄する(device);
            return Err(サンプラーを片付けて返す(device, sampler, 誤り));
        }
    };
    for 添字 in フレームスロット添字::全スロット() {
        binding::書き込む(device, &set一覧[添字.配列添字()], sampler, &束縛先, 添字);
    }
    Ok(遠方環境ディスクリプタ {
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
