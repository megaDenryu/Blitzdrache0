//! scene系のセット番号の契約のうち、コマンド記録側が知る必要のある番号と、その番号への束縛の規律を持つ。
//! 触れるのは発行ごとに変わらないビューとパスのセット(set0)・材質のセット(set2)・照明問い合わせのセット(set3)であり、
//! 描画対象ごとに変わるジオメトリのセット(set1)には触れない。
//!
//! 不変条件: ディスクリプタセットの束縛はパイプラインレイアウトの互換性で無効になる。シーン描画・布描画・シャドウ記録は
//! set1とset2の宣言が違うため互換ではなく、パイプラインを切り替えた側がこの2つを結び直さなければならない。
//! 呼び出し側が番号を書かないことで、番号の契約がここ1箇所に閉じる
//! (参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「束縛頻度による4セット」)。

use ash::vk;

use super::{セット別束縛計数, 共有セット束縛};

pub(super) const ビューとパスのセット番号: u32 = 0;
pub(super) const ジオメトリのセット番号: u32 = 1;
pub(super) const 材質のセット番号: u32 = 2;
pub(super) const 照明問い合わせのセット番号: u32 = 3;

/// シーン描画がパスの先頭で1回だけ結ぶset0・set2・set3。間のset1だけを描画対象ごとに結ぶ。
/// 材質のセットをここで結ぶことが、束縛の回数が材質数にもプリミティブ数にも比例しないことの実装である。
pub(super) fn シーンの共有セットを束縛する(
    device: &ash::Device,
    command_buffer: vk::CommandBuffer,
    layout: vk::PipelineLayout,
    共有: 共有セット束縛,
) {
    let 対応 = [
        (ビューとパスのセット番号, 共有.ビューとパス),
        (材質のセット番号, 共有.材質),
        (照明問い合わせのセット番号, 共有.照明問い合わせ),
    ];
    束縛する(device, command_buffer, layout, &対応, 共有.計数);
}

/// 布の描画が結ぶset0とset3。布のパイプラインはset2を資源を持たない空のレイアウトで宣言するため、材質のセットは結ばない。
pub(super) fn 布の共有セットを束縛する(
    device: &ash::Device,
    command_buffer: vk::CommandBuffer,
    layout: vk::PipelineLayout,
    共有: 共有セット束縛,
) {
    let 対応 = [
        (ビューとパスのセット番号, 共有.ビューとパス),
        (照明問い合わせのセット番号, 共有.照明問い合わせ),
    ];
    束縛する(device, command_buffer, layout, &対応, 共有.計数);
}

fn 束縛する(
    device: &ash::Device,
    command_buffer: vk::CommandBuffer,
    layout: vk::PipelineLayout,
    対応: &[(u32, vk::DescriptorSet)],
    計数: &セット別束縛計数,
) {
    for (番号, セット) in 対応 {
        計数.数える(*番号);
        // 安全性: command_bufferは記録中で、layoutとセットは互換の組として生成済みである。
        unsafe {
            device.cmd_bind_descriptor_sets(command_buffer, vk::PipelineBindPoint::GRAPHICS, layout, *番号, &[*セット], &[]);
        }
    }
}

/// ビューとパスのセットだけを束縛する。シャドウ記録のように照明問い合わせのセットを宣言しないパイプラインが使う。
pub(super) fn ビューとパスのセットを束縛する(
    device: &ash::Device,
    command_buffer: vk::CommandBuffer,
    layout: vk::PipelineLayout,
    共有: 共有セット束縛<'_>,
) {
    共有.計数.数える(ビューとパスのセット番号);
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
