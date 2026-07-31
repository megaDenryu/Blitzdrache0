//! ローダーが読む3枚のテクスチャの宣言の検査。受け取るのは材質スロット一覧、返すのは指摘一覧である。
//! 見るのはUV座標の組の番号と画像実体の形式であり、画素の中身は`material_scan`のデコードが見る。
//! 由来は`loader::material`がベースカラー・金属粗さ・法線マップだけを読むこと、`loader::mesh::primitive`がUV座標をTEXCOORD_0からしか読まないこと、
//! および`image`クレートの依存をpngとjpegだけに絞っていること(参照: ワークスペースのCargo.tomlの`image`)である。
//! 全スロットを見るのは、1メッシュが複数の材質を持てるようになり、先頭以外の材質のテクスチャも同じデコーダへ渡るためである。

use super::finding::契約指摘;
use super::slot_materials::材質スロットの参照;
use super::target::対象位置;

/// glTFが宣言できる画像形式のうち、デコーダが持っているもの。
const 対応形式一覧: [&str; 2] = ["image/png", "image/jpeg"];
const 対応拡張子一覧: [&str; 3] = ["png", "jpg", "jpeg"];

pub(super) fn 検査する(スロット一覧: &[材質スロットの参照<'_>]) -> Vec<契約指摘> {
    let mut 指摘一覧 = Vec::new();
    for スロット in スロット一覧 {
        材質を検査する(&mut 指摘一覧, スロット.番号, &スロット.プリミティブ.material());
    }
    指摘一覧
}

fn 材質を検査する(指摘一覧: &mut Vec<契約指摘>, スロット: u32, 材質: &gltf::Material<'_>) {
    let pbr = 材質.pbr_metallic_roughness();
    let 用途一覧 = [
        ("ベースカラー", pbr.base_color_texture().map(|情報| (情報.texture(), 情報.tex_coord()))),
        (
            "金属粗さ",
            pbr.metallic_roughness_texture().map(|情報| (情報.texture(), 情報.tex_coord())),
        ),
        ("法線マップ", 材質.normal_texture().map(|情報| (情報.texture(), 情報.tex_coord()))),
    ];
    for (用途, 参照) in 用途一覧 {
        let Some((テクスチャ, uv組番号)) = 参照 else {
            continue;
        };
        uv組番号を検査する(指摘一覧, スロット, 用途, uv組番号);
        画像形式を検査する(指摘一覧, &テクスチャ.source());
    }
}

fn uv組番号を検査する(指摘一覧: &mut Vec<契約指摘>, スロット: u32, 用途: &'static str, uv組番号: u32) {
    if uv組番号 == 0 {
        return;
    }
    指摘一覧.push(契約指摘::違反を作る(
        対象位置::テクスチャ { スロット, 用途 },
        format!("TEXCOORD_{uv組番号}を参照している。ローダーはTEXCOORD_0だけを読むため、別のUV座標の組を指定しても最初の組で貼られる"),
        "BlenderでUVマップを1つに統合し、そのUVマップを画像テクスチャノードに使う",
    ));
}

fn 画像形式を検査する(指摘一覧: &mut Vec<契約指摘>, 画像: &gltf::image::Image<'_>) {
    let 添字 = 画像.index();
    match 画像.source() {
        gltf::image::Source::View { mime_type, .. } => {
            if !対応形式一覧.contains(&mime_type) {
                指摘一覧.push(未対応形式の指摘(
                    添字,
                    &format!("glbへ埋め込まれた画像のmimeTypeが{mime_type}である"),
                ));
            }
        }
        gltf::image::Source::Uri { uri, mime_type } => 外部参照を検査する(指摘一覧, 添字, uri, mime_type),
    }
}

fn 外部参照を検査する(指摘一覧: &mut Vec<契約指摘>, 添字: usize, uri: &str, mime_type: Option<&str>) {
    if uri.starts_with("data:") {
        指摘一覧.push(契約指摘::違反を作る(
            対象位置::画像 { 添字 },
            "画像がdata URIで埋め込まれている。ローダーはdata URIを未対応として拒む",
            "glb形式で書き出してバイナリチャンクへ埋め込むか、外部のpngファイルとして書き出す",
        ));
        return;
    }
    if let Some(mime_type) = mime_type {
        if !対応形式一覧.contains(&mime_type) {
            指摘一覧.push(未対応形式の指摘(添字, &format!("mimeTypeが{mime_type}である")));
        }
        return;
    }
    let 拡張子 = uri.rsplit('.').next().unwrap_or("").to_ascii_lowercase();
    if !対応拡張子一覧.contains(&拡張子.as_str()) {
        指摘一覧.push(未対応形式の指摘(
            添字,
            &format!("mimeTypeの宣言が無く、参照先{uri}の拡張子も対応形式でない"),
        ));
    }
}

fn 未対応形式の指摘(添字: usize, 内容: &str) -> 契約指摘 {
    契約指摘::違反を作る(
        対象位置::画像 { 添字 },
        format!("{内容}。デコーダはpngとjpegだけを持つ"),
        "画像をpng形式かjpeg形式で書き出す",
    )
}
