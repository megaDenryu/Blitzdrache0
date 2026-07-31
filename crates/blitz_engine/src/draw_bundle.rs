//! 描画束の語彙。静的な描画片(原型・詳細段・プリミティブ添字・材質スロットID)へ、材質と可視個体区間の参照を結ぶ。
//! 1メッシュ複数材質を描くとき、可視性の単位は個体のままで描画の単位だけをプリミティブへ展開するという決定の実装がここである。
//! 片の識別は`fragment`、区間の参照は`visible_interval`、結んだ単位は`bundle`、束1つぶんの器は`list`、組み立ての工程は`build`にある。
//! 参照: `_doc/設計/マルチマテリアルと材質境界.md`「可視ID列の契約」

mod build;
#[cfg(test)]
mod build_fixture;
#[cfg(test)]
mod build_tests;
mod bundle;
mod error;
mod fragment;
mod list;
mod visible_interval;

pub use build::描画束を組み立てる;
pub use bundle::描画束;
pub use error::描画束エラー;
pub use fragment::静的描画片;
pub use list::描画束一覧;
pub use visible_interval::可視個体区間参照;
