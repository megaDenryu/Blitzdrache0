//! 頂点量の診断用の原型glTF一式の書き出し。段の数も段ごとの寸法も粗い原型と同じで、違うのは面を格子へ
//! 細分化したことだけである。外形と面の平面が一致するため深度が覆う範囲は変わらず、投入インデックス数だけが増える。
//!
//! 別のファイルとして書き出すのは、代表世界のアセットのバイトを1つも動かさずに頂点量の軸を作るためである。
//! この原型を読むのは計測指定で選ぶ診断世界だけであり、代表世界も既存の検収も読まない。
//! 参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「シャドウ性能の是正(フェーズ2性能課題、2026-08-03着手)」

mod faces;
mod geometry;

use std::path::Path;

use super::geometry::直方体諸元;
use super::gltf_json;

const 共有バッファファイル名: &str = "archetype_lod_subdivided.bin";
const 文書ファイル名: &str = "archetype_lod_subdivided.gltf";

pub(super) fn 書き出す(出力先ディレクトリ: &Path, 諸元一覧: &[直方体諸元]) -> Result<(), String> {
    super::書き込む(
        &出力先ディレクトリ.join(共有バッファファイル名),
        &geometry::バッファバイト列を作る(諸元一覧),
    )?;
    super::書き込む(
        &出力先ディレクトリ.join(文書ファイル名),
        gltf_json::文書を作る(諸元一覧, 共有バッファファイル名, geometry::直方体の量).as_bytes(),
    )
}
