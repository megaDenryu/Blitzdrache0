//! 既知の開発用ソースアセットを安定IDと実行時形式の出力名へ対応付ける。
//! 世界ごとに何を焼くかもここが決める。板の世界はスモークとサンプルの一式を、地形の世界は起動時シーン1つだけを持つ。

use std::path::{Path, PathBuf};

use blitz_engine::{アセットID, カタログ, チャンク座標};

use super::world::対象世界;

/// ストリーミング対象でないアセットは世界の原点チャンクへ帰属させる。
/// 参照: `_doc/計画/ユビキタス言語.md`「所有チャンク」
const 原点チャンク: チャンク座標 = チャンク座標::生成する(0, 0);

/// 地形世界の起動時シーン。レンダラーはチャンクが1つも常駐しない期間にも描画対象を要求するため、束ID0を占める最小の対象が要る。
/// 板の世界の`quad`と同じソースを別IDで登録するのは、初期カメラがシーン名で決まり、地形の俯瞰視点をこのIDへ紐づけるためである。
/// 参照: `crates/blitz_app/src/app/scene_camera.rs`
const 地形世界の起動時シーン: (&str, &str) = ("terrain_origin", "smoke/quad.gltf");

/// 板の世界の出力ルートへ焼く開発用アセット。第3欄はソースが無いときに失敗させるかどうかである。
const 板の世界の定義一覧: [(&str, &str, bool); 5] = [
    ("quad", "smoke/quad.gltf", true),
    ("quad_alt", "smoke/quad_alt.gltf", true),
    ("shadow_scene", "smoke/shadow_scene.gltf", true),
    ("helmet", "samples/DamagedHelmet/DamagedHelmet.glb", false),
    ("fox", "samples/Fox/Fox.glb", false),
];

/// コンパイル対象のソースが何の形式で書かれているか。読み方が形式ごとに違うため、判別共用体で持つ。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ソース種別 {
    /// glTF(.gltf/.glb)で書かれたシーン。
    Gltfシーン,
    高さ格子,
}

pub(super) struct コンパイル対象 {
    pub(super) id: アセットID,
    pub(super) 所有チャンク: チャンク座標,
    pub(super) 種別: ソース種別,
    pub(super) 出力パス: PathBuf,
}

pub(super) fn 構築する(
    ソースルート: &Path, 出力ルート: &Path, 世界: 対象世界
) -> Result<(カタログ, Vec<コンパイル対象>), String> {
    let mut カタログ = カタログ::空を作る();
    let mut 対象一覧 = Vec::new();
    for (名前, 相対パス, 必須) in 定義一覧を選ぶ(世界) {
        let ソースパス = ソースルート.join(相対パス);
        if !ソースパス.is_file() {
            if 必須 {
                return Err(format!("必須ソースアセットが存在しない: {}", ソースパス.display()));
            }
            println!("[compile_assets] 未取得のためスキップ: {}", ソースパス.display());
            continue;
        }
        let id = アセットID::生成する(名前).map_err(|誤り| 誤り.to_string())?;
        カタログ.登録する(id.clone(), ソースパス);
        対象一覧.push(コンパイル対象 {
            id,
            所有チャンク: 原点チャンク,
            種別: ソース種別::Gltfシーン,
            出力パス: 出力ルート.join(format!("{名前}.blitzasset")),
        });
    }
    Ok((カタログ, 対象一覧))
}

fn 定義一覧を選ぶ(世界: 対象世界) -> Vec<(&'static str, &'static str, bool)> {
    match 世界 {
        対象世界::板の世界 => 板の世界の定義一覧.to_vec(),
        対象世界::地形の世界 => vec![(地形世界の起動時シーン.0, 地形世界の起動時シーン.1, true)],
    }
}
