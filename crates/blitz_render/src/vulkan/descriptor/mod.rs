//! scene系パイプライン族が束縛する4セットのディスクリプタ。set0=ビューとパス(定数3本)、set1=ジオメトリと可視
//! (個体レコード・可視ID列)、set2=材質(材質レコード・テクスチャ)、set3=照明問い合わせ(シャドウマップ)である。
//! 番号と役割の契約は`_doc/設計/GPU資源束縛の分離と索引化.md`「束縛頻度による4セット」が正本であり、
//! 各セットのバインディング番号はその役割のモジュールが持つ。
//!
//! set0とset3は描画対象で変わらないため`shared_sets`が1組だけ持ち、set2は資源表世代ごとに材質資源表が持つ。
//! 束ごとに変わるのはset1だけであり`object_sets`が持つ。レイアウトは束をまたいで同一のため`scene_set_layouts`が1つだけ所有する。

mod alloc;
mod empty_set;
mod geometry_set;
mod lighting_set;
pub(crate) mod material_set;
mod object_sets;
mod scene_set_layouts;
mod shared_sets;
mod view_pass_set;

pub(crate) use object_sets::{ジオメトリセット参照, 描画対象ディスクリプタプール};
pub(crate) use scene_set_layouts::シーンセットレイアウト一式;
pub(crate) use shared_sets::{共有セット束縛, 共有ディスクリプタセット};
