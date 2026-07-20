//! 各段の組み立て結果を`フレーム資源`へ束ねる。
//! `generate_resources`の行数分割のためだけに切り出した内部ヘルパー。

use super::super::frame_resources::フレーム資源;
use super::base_resources::基礎資源;
use super::command_sync_resources::コマンド同期資源;
use super::optional_resources::追加資源;
use super::post_resources::ポスト資源;

pub(super) fn 束ねる(基礎: 基礎資源, コマンド同期: コマンド同期資源, 追加: 追加資源, ポスト: ポスト資源) -> フレーム資源 {
    フレーム資源 {
        深度バッファ: 基礎.深度バッファ,
        シャドウマップ: 基礎.シャドウマップ,
        シャドウパイプライン: コマンド同期.シャドウパイプライン,
        転送環境: 基礎.転送環境,
        ジオメトリ: 基礎.ジオメトリ,
        テクスチャ: 基礎.テクスチャ,
        ユニフォーム: 基礎.ユニフォーム,
        ディスクリプタ: 基礎.ディスクリプタ,
        command_pool: コマンド同期.command_pool,
        command_buffer一覧: コマンド同期.command_buffer一覧,
        フレーム同期: コマンド同期.フレーム同期,
        提示同期: コマンド同期.提示同期,
        pipeline: コマンド同期.pipeline,
        粒子: 追加.粒子,
        gpu計測: 追加.gpu計測,
        ui一式: 追加.ui一式,
        hdrターゲット: ポスト.hdrターゲット,
        トーンマップ: ポスト.トーンマップ,
    }
}
