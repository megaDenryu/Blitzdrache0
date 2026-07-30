//! 4枚のベイク済み画像を形の並び順に確保する工程。担当するのは「形の並びを受け取り、その順に画像を作り、
//! 途中で失敗したらそれまでに作った画像を逆順に破棄する」ことである。
//!
//! 巻き戻しをここに閉じるのは、確保が4回に増えたことで入れ子の分岐が深くなり、
//! 何を持っていて何を片付けるべきかが呼び出し元から見えにくくなるためである。

use ash::vk;

use crate::error::レンダラーエラー;
use crate::vulkan::atmosphere_lut::image::大気のベイク済み画像;
use crate::vulkan::atmosphere_lut::大気のベイク済み画像の形;
use crate::vulkan::tracked_device::GPUデバイス;

pub(super) fn 順に作る(
    device: &GPUデバイス,
    メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    形一覧: [大気のベイク済み画像の形; 4],
) -> Result<[大気のベイク済み画像; 4], レンダラーエラー> {
    let mut 作った: Vec<大気のベイク済み画像> = Vec::with_capacity(形一覧.len());
    for 形 in 形一覧 {
        match 大気のベイク済み画像::生成する(device, メモリプロパティ, 形) {
            Ok(画像) => 作った.push(画像),
            Err(誤り) => {
                while let Some(画像) = 作った.pop() {
                    画像.破棄する(device);
                }
                return Err(誤り);
            }
        }
    }
    let 件数 = 作った.len();
    Ok(作った
        .try_into()
        .unwrap_or_else(|_| panic!("大気のベイク済み画像を{}枚要求したのに{件数}枚できた", 形一覧.len())))
}
