//! 大気LUT画像をグラフへ登録するときの初期状態。焼き直す画像と引くだけの画像の2つを持つ。
//!
//! 2つを同じ場所に置くのは、どちらも「大気LUTが休むレイアウトはGENERALである」という1つの不変条件の表側と裏側で
//! あるためである。焼き直す画像だけはその不変条件を破ってよい(全テクセルを書き直すため内容を捨てられる)。
//! この不変条件は`画像用途::コンピュート読み`がSHADER_READ_ONLY_OPTIMALへ移さないことで保たれている。

use ash::vk;

use crate::vulkan::graph::state::画像状態;

/// そのフレームで焼き直す大気LUT画像の初期状態。
///
/// 注意: layoutをUNDEFINEDにできるのは、このフレームがそのLUTの全テクセルを書き直すからである。
/// 引くだけのLUTへこの状態を使ってはならない。UNDEFINEDからの遷移は中身を捨てることを許すため、
/// 前のフレームが焼いた値が消えうる。
pub(crate) fn 大気lutを焼く画像の初期状態() -> 画像状態 {
    画像状態::生成する(
        vk::PipelineStageFlags2::COMPUTE_SHADER,
        vk::AccessFlags2::SHADER_STORAGE_WRITE,
        vk::ImageLayout::UNDEFINED,
    )
}

/// そのフレームで引くだけの大気LUT画像の初期状態。
///
/// 注意: layoutはGENERALである。LUTは焼き直さないフレームも中身を保つため、UNDEFINEDで内容を捨てられない。
/// GENERALに固定できるのは、最初に焼いたフレームの書き込みでGENERALへ移った後、読む用途も同じGENERALを
/// 指定しており二度と別のレイアウトへ移らないためである。
/// 段とアクセスは保守的に和を取る。前フレームの最後の使用は、そのフレームが焼いたか引いただけかで変わる。
pub(crate) fn 大気lutを引く画像の初期状態() -> 画像状態 {
    画像状態::生成する(
        vk::PipelineStageFlags2::COMPUTE_SHADER,
        vk::AccessFlags2::SHADER_STORAGE_WRITE | vk::AccessFlags2::SHADER_SAMPLED_READ,
        vk::ImageLayout::GENERAL,
    )
}
