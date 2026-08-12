//! 照明問い合わせのセットのうち、遠方環境の枝だけが持つ3つの束縛先。触れるのは拡散照度の立方体画像(binding4)・
//! 鏡面畳込みの立方体画像(binding5)・反射率積分表(binding6)であり、直接光と影の束縛先には触れない。
//!
//! 不変条件: レイアウトがこの3つを宣言したセットは、使う前に必ず3つとも結ばれている。定数近似の枝はこの3つを
//! 1つも宣言しないため、未使用のダミー束縛を作らない(参照: `_doc/設計/放射輝度問い合わせ階層.md`「世界の間接照明方針と契約の2枝(3-Ic)」)。
//!
//! 注意: ディスクリプタのレイアウトはGENERALである。3つの画像は焼き直さないフレームも中身を保ち、
//! 休むレイアウトをGENERALに固定する不変条件を大気のベイク済み画像と共有するためである
//! (参照: `crates/blitz_render/src/vulkan/graph/initial_state/atmosphere_lut.rs`)。
//!
//! 3つを読む固定サンプラーは`sampler`が所有する。

mod sampler;

use ash::vk;

pub(crate) use sampler::遠方環境を読むサンプラー;

use super::照明問い合わせのセットの書き込み先;
use crate::vulkan::descriptor::束縛番号;

pub(crate) const 拡散照度の束縛番号: 束縛番号 = 束縛番号::生成する(4);
pub(crate) const 鏡面畳込みの束縛番号: 束縛番号 = 束縛番号::生成する(5);
pub(crate) const 反射率積分表の束縛番号: 束縛番号 = 束縛番号::生成する(6);

/// 3つの画像ビュー。どれも消費側が向きと粗さで参照する全段を含むビューである。
#[derive(Clone, Copy)]
pub(crate) struct 遠方環境の束縛先 {
    pub(crate) 拡散照度: vk::ImageView,
    pub(crate) 鏡面畳込み: vk::ImageView,
    pub(crate) 反射率積分表: vk::ImageView,
}

impl 照明問い合わせのセットの書き込み先<'_> {
    /// 3つの画像は焼き直しても束縛先のビューが変わらない固定資源のため、生成時に一度だけ結べばよい。
    pub(crate) fn 遠方環境の3つの画像を結ぶ(
        &self, 束縛先: 遠方環境の束縛先, サンプラー: &遠方環境を読むサンプラー
    ) {
        let 対応 = [
            (拡散照度の束縛番号, 束縛先.拡散照度),
            (鏡面畳込みの束縛番号, 束縛先.鏡面畳込み),
            (反射率積分表の束縛番号, 束縛先.反射率積分表),
        ];
        for (番号, ビュー) in 対応 {
            self.ディスクリプタの書き込み先().サンプラー付きの画像を結ぶ(
                番号,
                ビュー,
                サンプラー.サンプラーのハンドル(),
                vk::ImageLayout::GENERAL,
            );
        }
    }
}

pub(super) fn バインド一覧() -> [vk::DescriptorSetLayoutBinding<'static>; 3] {
    [
        画素段の画像バインド(拡散照度の束縛番号),
        画素段の画像バインド(鏡面畳込みの束縛番号),
        画素段の画像バインド(反射率積分表の束縛番号),
    ]
}

fn 画素段の画像バインド(番号: 束縛番号) -> vk::DescriptorSetLayoutBinding<'static> {
    vk::DescriptorSetLayoutBinding::default()
        .binding(番号.gpu境界値())
        .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
        .descriptor_count(1)
        .stage_flags(vk::ShaderStageFlags::FRAGMENT)
}
