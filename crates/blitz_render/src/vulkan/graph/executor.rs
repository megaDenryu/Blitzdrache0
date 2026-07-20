//! グラフ実行器: パス列を宣言順に走査し、バリアを発行してからパスを記録する。
//! バリア発行の呼び出しはここ(`barrier_apply::発行する`経由)に集約する
//! （参照: `_doc/設計/レンダーグラフ.md`「M5のDoD対応」）。パス前後のGPU
//! タイムスタンプ書き込み(判断30)もここに集約し、呼び出し元へ「パス名→クエリ開始添字」
//! の対応を返す。

use ash::vk;

use super::barrier_apply;
use super::barrier_derivation::{self, 地点別バリア};
use super::buffer_barrier_derivation;
use super::buffer_registry::バッファレジストリ;
use super::builder::グラフ;
use super::context::記録文脈;
use super::pass::{パス宣言, パス種別};
use super::pass_resource_usage::パスリソース使用;
use super::registry::画像レジストリ;
use super::rendering_setup;
use super::timestamp_write::{クエリ開始添字を求める, タイムスタンプを書く};

/// グラフを実行し、コマンドバッファへ記録する。グラフはここで消費される。
/// `クエリプール`が`Some`ならパス前後にタイムスタンプを書き、パス名と
/// クエリ開始添字の対応を返す(計測無効環境では`None`を渡し、空配列が返る)。
pub(crate) fn 実行する(
    device: &ash::Device,
    command_buffer: vk::CommandBuffer,
    グラフ: グラフ<'_>,
    クエリプール: Option<vk::QueryPool>,
) -> Vec<(&'static str, u32)> {
    let (寸法, 画像レジストリ, バッファレジストリ, 画像初期状態表, バッファ初期状態表, 最終用途表, パス列) =
        グラフ.分解する();

    let リソース使用列: Vec<パスリソース使用> = パス列.iter().map(パスリソース使用::from).collect();
    let 地点別画像バリア列 = barrier_derivation::導出する(&画像初期状態表, &リソース使用列, &最終用途表);
    let パス別バッファバリア列 = buffer_barrier_derivation::導出する(&バッファ初期状態表, &リソース使用列);

    let mut 地点別画像バリア列 = 地点別画像バリア列.into_iter();
    let mut 計測マッピング = Vec::new();
    for (パス添字, パス) in パス列.into_iter().enumerate() {
        let 地点別: 地点別バリア = 地点別画像バリア列
            .next()
            .unwrap_or_else(|| panic!("パス数とバリア導出結果の数が一致しない(実装のバグ)"));
        let バッファバリア一覧 = パス別バッファバリア列
            .get(パス添字)
            .unwrap_or_else(|| panic!("パス数とバッファバリア導出結果の数が一致しない(実装のバグ)"));
        barrier_apply::発行する(
            device,
            command_buffer,
            &画像レジストリ,
            &バッファレジストリ,
            &地点別.バリア一覧,
            バッファバリア一覧,
        );

        let パス名 = パス.名前;
        let クエリ開始添字 = クエリ開始添字を求める(パス添字);
        if let Some(pool) = クエリプール {
            タイムスタンプを書く(device, command_buffer, pool, クエリ開始添字);
            計測マッピング.push((パス名, クエリ開始添字));
        }
        パスを記録する(device, command_buffer, &画像レジストリ, &バッファレジストリ, 寸法, パス);
        if let Some(pool) = クエリプール {
            タイムスタンプを書く(device, command_buffer, pool, クエリ開始添字 + 1);
        }
    }

    let 終端: 地点別バリア = 地点別画像バリア列
        .next()
        .unwrap_or_else(|| panic!("グラフ終端のバリア導出結果が欠けている(実装のバグ)"));
    barrier_apply::発行する(device, command_buffer, &画像レジストリ, &バッファレジストリ, &終端.バリア一覧, &[]);

    計測マッピング
}

fn パスを記録する(
    device: &ash::Device,
    command_buffer: vk::CommandBuffer,
    画像レジストリ: &画像レジストリ,
    バッファレジストリ: &バッファレジストリ,
    寸法: vk::Extent2D,
    パス: パス宣言<'_>,
) {
    let 宣言済み画像 = パス.宣言済み画像一覧();
    let 宣言済みバッファ = パス.宣言済みバッファ一覧();
    let 記録文脈 = 記録文脈::生成する(
        device,
        command_buffer,
        画像レジストリ,
        バッファレジストリ,
        パス.名前,
        宣言済み画像,
        宣言済みバッファ,
    );

    match パス.種別 {
        パス種別::グラフィックス { カラー, 深度, クリア指定 } => {
            rendering_setup::開始する(device, command_buffer, 画像レジストリ, カラー, 深度, &クリア指定, 寸法);
            (パス.記録)(&記録文脈);
            rendering_setup::終了する(device, command_buffer);
        }
        パス種別::転送 | パス種別::コンピュート => {
            (パス.記録)(&記録文脈);
        }
    }
}
