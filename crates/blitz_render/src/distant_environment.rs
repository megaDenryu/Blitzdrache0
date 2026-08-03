//! 遠方環境(順3-Iの終端供給元)の公開の面。担当するのは「そのフレームで焼き直すかどうかの指示」と
//! 「GPU読み戻し検査の入口」の2つを外へ出すことである。
//!
//! 平坦な再エクスポートへ混ぜずモジュールごと公開するのは、派生表現(拡散照度・鏡面畳込み・反射率積分表)の
//! 数学と検査入口がここへ加わるためであり、遠方環境に属する型を1つの名前空間で辿れるようにするためである。
//! 参照: `_doc/設計/放射輝度問い合わせ階層.md`「順3-Iの実装設計」

pub mod consume;
pub mod derived;
pub mod derived_probe;
mod frame_input;
pub mod input;
mod pass_counts;
pub mod probe;
mod shader_set;

pub use frame_input::遠方環境の入力;
pub use pass_counts::間接照明生成パス数;
pub use shader_set::遠方環境のシェーダー一式;
