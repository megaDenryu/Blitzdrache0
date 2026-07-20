//! smoke実行前の一時コピー: shaders/配下の全.slangファイルとassets/smoke/一式を
//! target/配下へ複製する。スモークシナリオが監視対象ファイルを書き換えるため、
//! リポジトリ本体は汚さない。shaders/はimportでモジュール分割されているため
//! エントリファイル(scene.slang)単体でなくディレクトリ単位でコピーする
//! (分割先のpbr.slangがコピー先に無いとslangcのimport解決が失敗する)。

use std::path::PathBuf;

const エントリファイル名: &str = "scene.slang";
const アセットファイル一覧: [&str; 4] =
    ["quad.gltf", "quad.bin", "quad_base_color.png", "quad_alt_color.png"];

pub(super) fn シェーダーを一時コピーする() -> Result<PathBuf, String> {
    let コピー先ディレクトリ = PathBuf::from("target/smoke_shaders");
    std::fs::create_dir_all(&コピー先ディレクトリ)
        .map_err(|誤り| format!("コピー先ディレクトリの作成に失敗した: {誤り}"))?;

    let 読み取り結果 = std::fs::read_dir("shaders")
        .map_err(|誤り| format!("shaders/ディレクトリの読み取りに失敗した: {誤り}"))?;
    for エントリ結果 in 読み取り結果 {
        let エントリ =
            エントリ結果.map_err(|誤り| format!("shaders/ディレクトリの読み取りに失敗した: {誤り}"))?;
        let 元パス = エントリ.path();
        let 拡張子が対象か =
            元パス.extension().and_then(|拡張子| 拡張子.to_str()) == Some("slang");
        if !元パス.is_file() || !拡張子が対象か {
            continue;
        }
        let ファイル名 = エントリ.file_name();
        std::fs::copy(&元パス, コピー先ディレクトリ.join(&ファイル名))
            .map_err(|誤り| format!("{}のコピーに失敗した: {誤り}", 元パス.display()))?;
    }

    Ok(コピー先ディレクトリ.join(エントリファイル名))
}

pub(super) fn アセットを一時コピーする() -> Result<PathBuf, String> {
    let ルート = PathBuf::from("target/smoke_assets");
    let コピー先ディレクトリ = ルート.join("smoke");
    std::fs::create_dir_all(&コピー先ディレクトリ)
        .map_err(|誤り| format!("コピー先ディレクトリの作成に失敗した: {誤り}"))?;

    for ファイル名 in アセットファイル一覧 {
        let 元 = PathBuf::from("assets/smoke").join(ファイル名);
        let 先 = コピー先ディレクトリ.join(ファイル名);
        std::fs::copy(&元, &先)
            .map_err(|誤り| format!("{}のコピーに失敗した: {誤り}", 元.display()))?;
    }
    Ok(ルート)
}
