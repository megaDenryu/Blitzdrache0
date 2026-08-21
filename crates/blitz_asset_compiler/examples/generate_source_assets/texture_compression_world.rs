//! ブロック圧縮の対照世界の定義と、そのソースアセット一式の書き出し。世界の広がり(1チャンク)と対照の素材の
//! 一辺の画素数という世界そのものの決め事をこのモジュールが持ち、素材の色の決め方と板の形は子モジュールが持つ。
//!
//! この世界を既存の検収世界から分けるのは、対照の素材を既存の世界へ足すと、既定の方針で焼かれた既存の入口も
//! 同じカタログを読むことになり、既存の判定値の不変を対照の追加が脅かすためである。
//! 参照: `_doc/設計/テクスチャのブロック圧縮と縮小段生成.md`「段割りと各段の完了条件」の段4

mod geometry;
mod gltf_json;
mod gradient_texture;
mod noise_texture;

use std::path::Path;

use blitz_engine::チャンク座標;

use crate::directory_source::{目録ソースを作る, 目録項目};

/// 対照の素材の一辺の画素数。512画素は縮小段が10段まで並び、途中に4の倍数でない段(1x1まで下ると3・1)が現れる大きさである。
const 対照の素材の一辺の画素数: u32 = 512;
const 一辺メートル: f32 = 100.0;

/// この世界のチャンクは原点1つだけであり、その中身が対照の板そのものである。起動時のシーン名もこの安定IDを指す。
/// 安定IDを`texture_compression`で始めることが、書き換えもピクセル判定も持たない読み戻しだけの検収計画を選ばせる。
/// 参照: `crates/blitz_app/src/smoke/mod.rs`
const 対照の板のアセット識別子: &str = "texture_compression";

const グラデーション画像ファイル名: &str = "gradient.png";
const 雑音画像ファイル名: &str = "noise.png";
const 板のバイナリファイル名: &str = "contrast_plates.bin";
const 板の文書ファイル名: &str = "contrast_plates.gltf";
const 目録ソースファイル名: &str = "chunk_directory.txt";

pub(crate) fn 対照素材のソース一式を書き出す(出力先ディレクトリ: &Path) -> Result<(), String> {
    対照の素材を書き出す(出力先ディレクトリ)?;
    板を書き出す(出力先ディレクトリ)?;
    目録を書き出す(出力先ディレクトリ)
}

fn 対照の素材を書き出す(出力先ディレクトリ: &Path) -> Result<(), String> {
    let 一辺 = 対照の素材の一辺の画素数;
    let グラデーション = image::RgbaImage::from_fn(一辺, 一辺, gradient_texture::滑らかなグラデーションの画素を求める);
    画像をpngファイルへ保存する(&出力先ディレクトリ.join(グラデーション画像ファイル名), &グラデーション)?;
    let 雑音 = image::RgbaImage::from_fn(一辺, 一辺, noise_texture::決定的な雑音の画素を求める);
    画像をpngファイルへ保存する(&出力先ディレクトリ.join(雑音画像ファイル名), &雑音)
}

fn 板を書き出す(出力先ディレクトリ: &Path) -> Result<(), String> {
    バイト列をファイルへ書き込む(&出力先ディレクトリ.join(板のバイナリファイル名), &geometry::バッファバイト列を作る())?;
    バイト列をファイルへ書き込む(
        &出力先ディレクトリ.join(板の文書ファイル名),
        gltf_json::対照の板のgltf文書を作る(板のバイナリファイル名, グラデーション画像ファイル名, 雑音画像ファイル名).as_bytes(),
    )
}

fn 目録を書き出す(出力先ディレクトリ: &Path) -> Result<(), String> {
    let 項目一覧 = vec![目録項目 {
        座標: チャンク座標::生成する(0, 0),
        アセット識別子: 対照の板のアセット識別子.to_string(),
        ソース相対パス: 板の文書ファイル名.to_string(),
    }];
    バイト列をファイルへ書き込む(
        &出力先ディレクトリ.join(目録ソースファイル名),
        目録ソースを作る(一辺メートル, &項目一覧).as_bytes(),
    )
}

fn 画像をpngファイルへ保存する(パス: &Path, 画像: &image::RgbaImage) -> Result<(), String> {
    画像.save(パス).map_err(|誤り| format!("{}: {誤り}", パス.display()))
}

fn バイト列をファイルへ書き込む(パス: &Path, バイト列: &[u8]) -> Result<(), String> {
    std::fs::write(パス, バイト列).map_err(|誤り| format!("{}: {誤り}", パス.display()))
}
