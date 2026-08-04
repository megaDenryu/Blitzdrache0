//! 目視見本世界`terrain_visual`の起動材料。担当するのは、この世界のシーン名とアセットルートを1箇所に置くことと、実行時形式を作って実在を確かめることである。
//!
//! 表を入口ごとに持たないのは、複数の検収入口が同じ世界を同じアセットルートから起動するためである。
//! 片方だけがルートを変えると、絵を撮る入口と数を採る入口が別の世界を見ることになり、並べて読む根拠が消える。

pub const シーン名: &str = "terrain_visual";
pub const アセットルート: &str = "target/terrain_visual_assets";

/// 目視見本の実行時形式。この世界だけの出力ルートへ焼かれる。
const 実行時形式のパス: &str = "target/terrain_visual_assets/terrain_visual.blitzasset";

/// ソースアセットの生成から実行時形式の実在確認までを行う。
pub fn 用意する() -> Result<(), String> {
    if !crate::gen_source_assets::生成する() || !crate::compile_assets::目視見本世界を既定で生成する() {
        return Err("目視見本のアセット生成に失敗した".to_string());
    }
    実行時形式の実在を確かめる()
}

/// 外部のアセットリポジトリが無い環境では、小物の原型が1つも解決できずコンパイルが失敗する。
/// 失敗を見落として起動へ進むと、カタログ未登録という遠い場所の失敗に化けるため、ここで名指しして落とす。
fn 実行時形式の実在を確かめる() -> Result<(), String> {
    if std::path::Path::new(実行時形式のパス).is_file() {
        return Ok(());
    }
    Err(format!(
        "{実行時形式のパス}が作られていない。外部のアセットリポジトリが見つからなかった可能性が高い(compile-assetsの出力に理由が出る)"
    ))
}
