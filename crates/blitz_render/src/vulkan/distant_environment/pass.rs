//! 遠方環境の生成パスの宣言。コンピュートで立方体画像の6層を1度の計算の発行で焼く。
//! 本番のフレーム記録と読み戻し検査の両方がこの1つの宣言を使うため、検査は本番と同じバリア導出を通る。
//! begin/end renderingは無く、記録クロージャはバインドと計算の発行だけを書く。
//!
//! 書き込み先を「書き画像」として宣言することで、後続パスがこの画像を読むときのバリアをグラフが導く。
//! パス名はGPU計器の区間名としてそのまま`--report-gpu-times`の表に現れる。

use ash::vk;

use super::遠方環境の生成入力;
use crate::vulkan::graph::{パス宣言, パス種別, 画像ハンドル, 画像用途};

/// GPU計器の区間名。設計正本が「遠方環境生成」を独立の区間として要求する
/// (参照: `_doc/設計/放射輝度問い合わせ階層.md`「検収(機械判定)」)。
pub(crate) const 遠方環境生成のパス名: &str = "遠方環境生成";

pub(crate) fn 作る<'a>(
    書き込み先: 画像ハンドル, 読み画像: Vec<(画像ハンドル, 画像用途)>, 入力: &'a 遠方環境の生成入力
) -> パス宣言<'a> {
    パス宣言::生成する(
        遠方環境生成のパス名,
        読み画像,
        vec![(書き込み先, 画像用途::コンピュート書き)],
        Vec::new(),
        Vec::new(),
        パス種別::コンピュート,
        move |文脈| {
            // 宣言(書き:コンピュート書き)が実際の使用と一致することを検査する(判断28: 宣言=真実)。
            // 戻り値のvk::Image自体はディスクリプタセット経由の束縛で既に解決済みのため使わない。
            let _ = 文脈.画像を解決する(書き込み先);

            let device = 文脈.device();
            let command_buffer = 文脈.コマンドバッファ();
            let set一覧 = [入力.ディスクリプタセット];
            let [横の計算の班数, 縦の計算の班数, 層の計算の班数] = 入力.計算の班数;

            // 安全性: command_bufferは記録中で、pipeline・ディスクリプタセットは生成済み。
            unsafe {
                device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::COMPUTE, 入力.pipeline);
                device.cmd_bind_descriptor_sets(command_buffer, vk::PipelineBindPoint::COMPUTE, 入力.layout, 0, &set一覧, &[]);
            }
            let バイト列 = 入力.即時定数.バイト列();
            // 安全性: command_bufferは記録中で、layoutはこの入力のパイプラインのものである。
            // バイト列の長さはパイプラインレイアウトが宣言した即時定数の範囲と、シェーダー側の宣言の両方に一致する。
            unsafe {
                device.cmd_push_constants(command_buffer, 入力.layout, vk::ShaderStageFlags::COMPUTE, 0, &バイト列);
            }
            // 安全性: 直前にパイプラインとディスクリプタを束縛済みで、計算の班数は寸法から切り上げた正当な値。
            unsafe {
                device.cmd_dispatch(command_buffer, 横の計算の班数, 縦の計算の班数, 層の計算の班数);
            }
        },
    )
}
