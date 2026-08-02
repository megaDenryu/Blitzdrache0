//! scene系のセット番号の契約のうち、コマンド記録側が知る必要のある番号と、その番号への束縛の規律を持つ。
//! 触れるのはビューとパスのセット(set0)と照明問い合わせのセット(set3)だけであり、発行ごとに変わるset1・set2には触れない。
//!
//! 不変条件: ディスクリプタセットの束縛はパイプラインレイアウトの互換性で無効になる。シーン描画・布描画・シャドウ記録は
//! set1とset2の宣言が違うため互換ではなく、パイプラインを切り替えた側がこの2つを結び直さなければならない。
//! 呼び出し側が番号を書かないことで、番号の契約がここ1箇所に閉じる
//! (参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「束縛頻度による4セット」)。

use ash::vk;

use super::共有セット束縛;

pub(super) const ビューとパスのセット番号: u32 = 0;
pub(super) const ジオメトリのセット番号: u32 = 1;
pub(super) const 照明問い合わせのセット番号: u32 = 3;

/// set0とset3を束縛する。間のset1とset2は発行ごとに結ぶため、2回に分けて呼ぶ。
pub(super) fn 共有セットを束縛する(
    device: &ash::Device,
    command_buffer: vk::CommandBuffer,
    layout: vk::PipelineLayout,
    共有: 共有セット束縛,
) {
    let 対応 = [
        (ビューとパスのセット番号, 共有.ビューとパス),
        (照明問い合わせのセット番号, 共有.照明問い合わせ),
    ];
    for (番号, セット) in 対応 {
        // 安全性: command_bufferは記録中で、layoutとセットは互換の組として生成済みである。
        unsafe {
            device.cmd_bind_descriptor_sets(command_buffer, vk::PipelineBindPoint::GRAPHICS, layout, 番号, &[セット], &[]);
        }
    }
}

/// ビューとパスのセットだけを束縛する。シャドウ記録のように照明問い合わせのセットを宣言しないパイプラインが使う。
pub(super) fn ビューとパスのセットを束縛する(
    device: &ash::Device,
    command_buffer: vk::CommandBuffer,
    layout: vk::PipelineLayout,
    共有: 共有セット束縛,
) {
    // 安全性: command_bufferは記録中で、layoutとセットは互換の組として生成済みである。
    unsafe {
        device.cmd_bind_descriptor_sets(
            command_buffer,
            vk::PipelineBindPoint::GRAPHICS,
            layout,
            ビューとパスのセット番号,
            &[共有.ビューとパス],
            &[],
        );
    }
}
