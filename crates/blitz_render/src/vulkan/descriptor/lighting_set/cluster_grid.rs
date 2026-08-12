//! 照明問い合わせのセットのうち、クラスタ格子の区間表(binding8)と光添字列(binding9)だけを担う束縛先。
//! 触れるのはこの2つの番号と、そこへバッファを結ぶ操作だけである。
//!
//! 遠方環境の3つと違い、この2つは両方の照明束縛レイアウトの枝が必ず宣言する。全世界がクラスタの経路を通り、
//! 局所光が0件の世界は件数0のセルだけを持つ格子を読むためである。枝を分けないことで、照明束縛レイアウトの枝も
//! パイプラインキーの成分も増えない
//! (参照: `_doc/設計/クラスタ多光源と点光源の影.md`「判断f: 照明束縛レイアウトの枝を増やさず、パイプラインキーへ照明の能力の成分も足さない」)。
//!
//! 画素段だけが読む束縛としてここへ置き、書く側(選別のコンピュート)は同じバッファを自分の生成側のセットへ結ぶ。
//! set3の段の指定を画素段へ限る方針を保つためである。

use ash::vk;

use super::照明問い合わせのバッファ組;

pub(crate) const クラスタ格子の束縛番号: u32 = 8;
pub(crate) const クラスタ光添字列の束縛番号: u32 = 9;

pub(super) fn バインド一覧() -> [vk::DescriptorSetLayoutBinding<'static>; 2] {
    [
        画素段の記憶バインド(クラスタ格子の束縛番号),
        画素段の記憶バインド(クラスタ光添字列の束縛番号),
    ]
}

/// そのスロットの2本を、番号と対にして返す。結ぶ手順は呼び出し元が直接光の3本と共有する。
pub(super) fn 番号とバッファの対(バッファ組: 照明問い合わせのバッファ組) -> [(u32, vk::Buffer); 2] {
    [
        (クラスタ格子の束縛番号, バッファ組.クラスタ格子),
        (クラスタ光添字列の束縛番号, バッファ組.クラスタ光添字列),
    ]
}

fn 画素段の記憶バインド(binding: u32) -> vk::DescriptorSetLayoutBinding<'static> {
    vk::DescriptorSetLayoutBinding::default()
        .binding(binding)
        .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
        .descriptor_count(1)
        .stage_flags(vk::ShaderStageFlags::FRAGMENT)
}
