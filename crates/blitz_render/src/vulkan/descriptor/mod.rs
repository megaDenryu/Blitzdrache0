//! scene系パイプライン族が束縛する4セットのディスクリプタ。set0=ビューとパス(定数3本)、set1=ジオメトリと可視
//! (個体レコード・可視ID列)、set2=材質(材質レコード・テクスチャ)、set3=照明問い合わせ(シャドウマップ)である。
//! 番号と役割の契約は`_doc/設計/GPU資源束縛の分離と索引化.md`「束縛頻度による4セット」が正本であり、
//! 各セットの束縛番号はその役割のモジュールが持つ。
//!
//! set0とset3は描画対象で変わらないため`shared_sets`が1組だけ持ち、set2は資源表世代ごとに材質資源表が持つ。
//! 束ごとに変わるのはset1だけであり`object_sets`が持つ。レイアウトは束をまたいで同一のため`scene_set_layouts`が1つだけ所有する。
//!
//! 割り当て済みのセットへ現物を書き込む口は、役割ごとの書き込み先の型が持つ。どの型も論理デバイスと1つのセットの組
//! (`sink`の`ディスクリプタの書き込み先`)を包み、自分の束縛番号だけを結ぶメソッドで公開する。

mod alloc;
mod binding_number;
mod declared_bindings;
mod empty_set;
mod geometry_set;
pub(crate) mod lighting_set;
pub(crate) mod material_set;
mod object_sets;
mod scene_set_layouts;
mod shared_sets;
mod sink;
mod view_pass_set;

pub(crate) use binding_number::束縛番号;
pub(crate) use declared_bindings::{
    宣言から作ったセットレイアウト, 宣言から割り当てたセット, 宣言した束縛の並び, 結ぶ現物
};
pub(crate) use lighting_set::{
    照明問い合わせのディスクリプタプール, 照明問い合わせのバッファ組, 照明問い合わせの割り当て済みセット
};
pub(crate) use material_set::材質の割り当て済みセット;
pub(crate) use object_sets::{ジオメトリセット参照, 描画対象ディスクリプタプール};
pub(crate) use scene_set_layouts::シーンセットレイアウト一式;
pub(crate) use shared_sets::{共有セット束縛, 共有ディスクリプタセット};

/// 木の内側だけで使う低水準の書き込み先。`pub`を付けないため、この束縛は`descriptor`の子孫からしか見えない。
use sink::ディスクリプタの書き込み先;
