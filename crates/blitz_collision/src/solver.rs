//! 形ごとの問い合わせが共有する求解の部品。線分が形に当たる時刻と、掃引したカプセルが形に触れる時刻を、
//! 形の種類に依らない無次元の量と、2次方程式・2直線の最近接・最小の候補の選択という道具で解く。
//!
//! ここに置くのは、高さ場の三角形と任意姿勢の直方体のどちらの求解も同じ式で書けるものだけである。
//! 形ごとに違うのは、どの特徴がどこにあってどちらを向いているかであり、その特徴から式を組んだ後の手順は
//! 同じである。式を形ごとに書き写すと、片方だけを直した食い違いが答えの違いとして出るまで見つからない。
//!
//! 変位を能力で受けるのは、高さ場の格子原点を基準にした変位と、形の局所座標を基準にした変位という2つの
//! 実装が同じ式を使うためである。2つを1つの型にまとめないのは、成分の意味も、成分の絶対値へ課す上限の
//! 根拠も別だからである。
//! 参照: `_doc/設計/世界の形と衝突基盤.md`「判断14: 決定性の維持」

mod axis_end;
mod displacement;
mod error;
mod line_pair;
mod quadratic;
mod quadratic_from_geometry;
mod ratio;
mod segment_parameter;
mod smallest;

pub use axis_end::軸の端;
pub use error::無次元の媒介変数の生成エラー;
pub use ratio::掃引で動けた割合;
pub use segment_parameter::線分の媒介変数;

pub(crate) use displacement::求解が扱う変位;
pub(crate) use line_pair::二つの直線の最近接の媒介変数;
pub(crate) use quadratic::掃引の2次方程式;
pub(crate) use smallest::最も小さい量の候補だけを覚える器;
