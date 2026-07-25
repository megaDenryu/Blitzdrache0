//! 既知の開発用ソースアセットを安定IDと実行時形式の出力名へ対応付ける。

use std::path::{Path, PathBuf};

use blitz_engine::{アセットID, カタログ, チャンク座標};

/// ストリーミング対象でないアセットは世界の原点チャンクへ帰属させる。
/// 参照: `_doc/計画/ユビキタス言語.md`「所有チャンク」
const 原点チャンク: チャンク座標 = チャンク座標::生成する(0, 0);

pub(super) struct コンパイル対象 {
    pub(super) id: アセットID,
    pub(super) 所有チャンク: チャンク座標,
    pub(super) 出力パス: PathBuf,
}

pub(super) fn 構築する(ソースルート: &Path, 出力ルート: &Path) -> Result<(カタログ, Vec<コンパイル対象>), String> {
    let 定義一覧 = [
        ("quad", "smoke/quad.gltf", true),
        ("quad_alt", "smoke/quad_alt.gltf", true),
        ("shadow_scene", "smoke/shadow_scene.gltf", true),
        ("helmet", "samples/DamagedHelmet/DamagedHelmet.glb", false),
        ("fox", "samples/Fox/Fox.glb", false),
    ];
    let mut カタログ = カタログ::空を作る();
    let mut 対象一覧 = Vec::new();
    for (名前, 相対パス, 必須) in 定義一覧 {
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
            出力パス: 出力ルート.join(format!("{名前}.blitzasset")),
        });
    }
    Ok((カタログ, 対象一覧))
}
