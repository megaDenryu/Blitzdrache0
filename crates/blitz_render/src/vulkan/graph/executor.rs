//! グラフ実行器: パス列を宣言順に走査し、バリアを発行してからパスを記録する。
//! バリア発行の呼び出しはここ(`barrier_apply::発行する`経由)に集約する
//! （参照: `_doc/設計/レンダーグラフ.md`「M5のDoD対応」）。

use ash::vk;

use super::barrier_apply;
use super::barrier_derivation::{self, 地点別バリア};
use super::builder::グラフ;
use super::context::記録文脈;
use super::pass::{パス宣言, パス種別};
use super::pass_resource_usage::パスリソース使用;
use super::registry::画像レジストリ;
use super::rendering_setup;

/// グラフを実行し、コマンドバッファへ記録する。グラフはここで消費される。
pub(crate) fn 実行する(device: &ash::Device, command_buffer: vk::CommandBuffer, グラフ: グラフ<'_>) {
    let (寸法, レジストリ, 初期状態表, 最終用途表, パス列) = グラフ.分解する();

    let リソース使用列: Vec<パスリソース使用> = パス列.iter().map(パスリソース使用::from).collect();
    let 地点別バリア列 = barrier_derivation::導出する(&初期状態表, &リソース使用列, &最終用途表);

    let mut 地点別バリア列 = 地点別バリア列.into_iter();
    for パス in パス列 {
        let 地点別: 地点別バリア = 地点別バリア列
            .next()
            .unwrap_or_else(|| panic!("パス数とバリア導出結果の数が一致しない(実装のバグ)"));
        barrier_apply::発行する(device, command_buffer, &レジストリ, &地点別.バリア一覧);
        パスを記録する(device, command_buffer, &レジストリ, 寸法, パス);
    }

    let 終端: 地点別バリア = 地点別バリア列
        .next()
        .unwrap_or_else(|| panic!("グラフ終端のバリア導出結果が欠けている(実装のバグ)"));
    barrier_apply::発行する(device, command_buffer, &レジストリ, &終端.バリア一覧);
}

fn パスを記録する(
    device: &ash::Device,
    command_buffer: vk::CommandBuffer,
    レジストリ: &画像レジストリ,
    寸法: vk::Extent2D,
    パス: パス宣言<'_>,
) {
    let 宣言済み画像 = パス.宣言済み画像一覧();
    let 記録文脈 = 記録文脈::生成する(device, command_buffer, レジストリ, パス.名前, 宣言済み画像);

    match パス.種別 {
        パス種別::グラフィックス { カラー, 深度, クリア指定 } => {
            rendering_setup::開始する(device, command_buffer, レジストリ, カラー, 深度, &クリア指定, 寸法);
            (パス.記録)(&記録文脈);
            rendering_setup::終了する(device, command_buffer);
        }
        パス種別::転送 => {
            (パス.記録)(&記録文脈);
        }
        パス種別::コンピュート => {
            panic!("コンピュートパスは波2で実装予定のため未対応: パス名={}", パス.名前);
        }
    }
}
