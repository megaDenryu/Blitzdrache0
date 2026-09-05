//! 楽曲の型契約。楽曲エディターが打ち込む1曲ぶんを、素データ様式でブラウザとサーバーが共有する。
//!
//! テンポ・音量・ステップ数を素の数値と名前の格で持つのは、この層がserdeとJSONの直列化境界に接するためである。
//! 単位と値域を型で守る数学DDDは、境界の内側にあるブラウザの編集モデルが担う
//! (参照: `_doc/設計/ゲーム開発用エディター基盤.md`「判断8」、`_doc/設計/楽曲エディター.md`)。

mod chord;
mod chord_progression;
mod command;
mod instrument;
mod mixer;
mod music_id;
mod note_rows;
mod pattern;
mod pattern_id;
mod pattern_roster;
mod preset_progression;
#[cfg(test)]
mod preset_progression_tests;
mod progression_reference;
mod progression_roster;
mod section;
mod song;
mod track;
mod track_grid;
mod value_range;
mod version;

pub use chord::{和音, 和音の種類};
pub use chord_progression::コード進行;
pub use command::{
    テンポを変える, トラックの楽器を変える, トラックの進行の割り当てを変える, トラックの音量を変える, パターンの打点を全部消す,
    パターンの表示名を変える, パターンの進行を変える, パターンを削除する, パターンを追加する, ミキサー設定を変える, 打ち込みの対象, 打点を消す,
    打点を置く, 曲の節を並べ替える, 曲の節を削除する, 曲の節を変える, 曲の節を追加する, 楽曲の表示名を変える, 楽曲編集コマンド, 独自の進行を保存する,
    独自の進行を削除する, 範囲の打点を消す, 音を伸ばす,
};
pub use instrument::{打楽器の種類, 楽器};
pub use mixer::ミキサー設定;
pub use music_id::楽曲ID;
pub use note_rows::音の並び;
pub use pattern::パターン;
pub use pattern_id::パターンID;
pub use preset_progression::{既定のコード進行, 既定のコード進行一覧};
pub use progression_reference::コード進行参照;
pub use section::曲の節;
pub use song::{楽曲, 楽曲の現在の形式版};
pub use track::{トラックの種類, トラック定義};
pub use track_grid::トラックの格子;
pub use value_range::{
    テンポの上限, テンポの下限, パターンのステップ数, 和音の根音の上限, 和音の根音の下限, 和音の続くステップ数の上限, 和音の続くステップ数の下限,
    曲の節の繰り返し回数の上限, 曲の節の繰り返し回数の下限, 遅延のステップ数の上限, 遅延のステップ数の下限, 音量と効果の比の上限,
    音量と効果の比の下限, 音高番号の上限, 音高番号の下限,
};
pub use version::{楽曲の版の移行エラー, 読み込んだ楽曲の版};
