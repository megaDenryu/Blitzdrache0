//! 空中遠近合成ディスクリプタの生成局面。2つのサンプラー・レイアウト・プール・セットを順に作り、
//! ボリュームだけを束縛する。深度は毎フレームの束縛であり、ここでは結ばない。
//! 呼ばれるのは合成パイプラインを組み立てる直前の1回だけである。
//! 途中で失敗したら、それまでに作ったハンドルをその場で逆順に片付ける。

use ash::vk;

use super::{binding, 空中遠近合成の束縛先, 空中遠近合成ディスクリプタ};
use crate::error::レンダラーエラー;
use crate::vulkan::allocator::GPU資源の確保係;
use crate::vulkan::atmosphere_lut::descriptor_common;
use crate::vulkan::sync::フレームスロット添字;

pub(super) fn 生成する(
    確保係: &GPU資源の確保係<'_>,
    束縛先: &空中遠近合成の束縛先,
) -> Result<空中遠近合成ディスクリプタ, レンダラーエラー> {
    let device = 確保係.論理デバイス();
    let 深度サンプラー = 確保係.最近傍サンプラーを作る()?;
    let ボリュームサンプラー = match 確保係.線形サンプラーを作る() {
        Ok(sampler) => sampler,
        Err(誤り) => return Err(作ったサンプラーを片付けて返す(device, &[深度サンプラー], 誤り)),
    };
    let 二本 = [深度サンプラー, ボリュームサンプラー];
    let layout = match binding::レイアウトを作る(device) {
        Ok(layout) => layout,
        Err(誤り) => return Err(作ったサンプラーを片付けて返す(device, &二本, 誤り)),
    };
    let pool = match binding::プールを作る(device) {
        Ok(pool) => pool,
        Err(誤り) => {
            descriptor_common::途中の資源を片付ける(device, layout, None);
            return Err(作ったサンプラーを片付けて返す(device, &二本, 誤り));
        }
    };
    let set一覧 = match descriptor_common::セットを割り当てる(device, pool, layout) {
        Ok(set一覧) => set一覧,
        Err(誤り) => {
            descriptor_common::途中の資源を片付ける(device, layout, Some(pool));
            return Err(作ったサンプラーを片付けて返す(device, &二本, 誤り));
        }
    };
    for 添字 in フレームスロット添字::全スロット() {
        binding::ボリュームを書き込む(device, set一覧[添字.配列添字()], ボリュームサンプラー, 束縛先);
    }
    Ok(空中遠近合成ディスクリプタ {
        layout,
        pool,
        深度サンプラー,
        ボリュームサンプラー,
        set一覧,
    })
}

/// そこまでに作ったサンプラーを生成の逆順で片付ける。渡す一覧は作った順である。
fn 作ったサンプラーを片付けて返す(
    device: &ash::Device, 一覧: &[vk::Sampler], 誤り: レンダラーエラー
) -> レンダラーエラー {
    for sampler in 一覧.iter().rev() {
        // 安全性: samplerはこのスコープの唯一の所有者で、以降使用しない。
        unsafe { device.destroy_sampler(*sampler, None) };
    }
    誤り
}
