//! シーン描画とシャドウ描画が使うディスクリプタ: binding0-2=combined image sampler×3、
//! binding3=フレームuniform buffer(判断21・判断24)、binding4=シャドウマップの比較サンプラー(判断35)、
//! binding5=描画対象uniform buffer、binding6=個体変換のストレージバッファ。
//! レイアウトは全描画対象で同一のため1つを共有し、セットは描画対象の束ごとに専用プールから
//! 描画対象数×フレームインフライト数だけ割り当てる。プールを束ごとに分けることで、束の解除が
//! プール1つの破棄で完結し、ディスクリプタセット添字が束の内側に閉じる。

mod layout;
mod object_sets;
mod pool;
mod set;
mod shadow_binding;

pub(crate) use layout::ディスクリプタレイアウト;
pub(crate) use object_sets::{描画対象ディスクリプタプール, 描画対象ディスクリプタ参照};
