//! プリミティブ描画項目の語彙。静的な描画片(原型・詳細段・プリミティブ添字・材質スロットID)へ、材質と可視個体区間の参照を結ぶ。
//! 1メッシュ複数材質を描くとき、可視性の単位は個体のままで描画の単位だけをプリミティブへ展開するという決定の実装がここである。
//! 片の識別は`fragment`、描くインデックス区間は`draw_range`、材質の指し方は`material_binding`、可視個体区間の参照は`visible_interval`、
//! 結んだ単位は`item`、束1つぶんの器は`list`、器へ描画対象1つぶんを積む工程は`build`にある。
//! 参照: `_doc/設計/マルチマテリアルと材質境界.md`「可視ID列の契約」

mod build;
#[cfg(test)]
mod build_fixture;
#[cfg(test)]
mod build_tests;
mod draw_range;
mod error;
mod fragment;
mod item;
mod list;
mod material_binding;
mod visible_interval;

pub use draw_range::プリミティブ描画範囲;
pub use error::プリミティブ描画項目エラー;
pub use fragment::静的描画片;
pub use item::プリミティブ描画項目;
pub use list::プリミティブ描画項目一覧;
pub use material_binding::材質束縛参照;
pub use visible_interval::可視個体区間参照;
