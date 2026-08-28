//! 派生表現の生成パスの宣言。コンピュートで1つの書き込み先を焼く。担当する工程は「パス名と書き込み先と
//! 読み画像と入力を受け取り、コンピュート種別のパス宣言を返す」ことである。
//!
//! 本番のフレーム記録と読み戻し検査の両方がこの1つの宣言を使うため、検査は本番と同じバリア導出を通る。
//! begin/end renderingは無く、GPU命令をコマンドバッファへ積むクロージャはバインドと計算の発行だけを書く。
//!
//! パス名はGPU計器の区間名としてそのまま報告の表に現れる。
//! 鏡面畳込みは粗さの段ごとにパスを積むため、計器は同じ名前の回をフレーム内で足して1標本にし、報告の値は全縮小段の合計になる。
//! 区間は3つに保ち、段ごとに名前を分けない(参照: `_doc/設計/放射輝度問い合わせ階層.md`「検収(機械判定)」)。

use ash::vk;

use super::派生表現の生成入力;
use crate::vulkan::graph::{パス宣言, パス種別, 画像ハンドル, 画像用途};

/// GPU計器の区間名。設計正本が3つを独立の区間として要求する。
pub(crate) const 拡散照度生成のパス名: &str = "拡散照度生成";
pub(crate) const 鏡面畳込み生成のパス名: &str = "鏡面畳込み生成";
pub(crate) const 反射率積分表生成のパス名: &str = "反射率積分表生成";

pub(in crate::vulkan) fn 作る<'a>(
    パス名: &'static str,
    書き込み先: 画像ハンドル,
    読み画像: Vec<(画像ハンドル, 画像用途)>,
    入力: &'a 派生表現の生成入力,
) -> パス宣言<'a> {
    パス宣言::生成する(
        パス名,
        読み画像,
        vec![(書き込み先, 画像用途::コンピュート書き)],
        Vec::new(),
        Vec::new(),
        パス種別::コンピュート,
        move |文脈| {
            // 宣言(書き:コンピュート書き)が実際の使用と一致することを検査する(判断28: 宣言=真実)。
            // 戻り値のvk::Image自体はディスクリプタセット経由の束縛で既に解決済みのため使わない。
            let _ = 文脈.宣言済みの画像を参照する(書き込み先);

            let device = 文脈.積み先().論理デバイス();
            let command_buffer = 文脈.積み先().コマンドバッファ();
            let set一覧 = [入力.ディスクリプタセット];
            let [横の計算の班数, 縦の計算の班数, 層の計算の班数] = 入力.計算の班数;

            // 安全性: command_bufferは記録中で、pipeline・ディスクリプタセットは生成済み。
            unsafe {
                device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::COMPUTE, 入力.pipeline);
                device.cmd_bind_descriptor_sets(command_buffer, vk::PipelineBindPoint::COMPUTE, 入力.layout, 0, &set一覧, &[]);
            }
            let バイト列 = 入力.即時定数.バイト列();
            if !バイト列.is_empty() {
                // 安全性: command_bufferは記録中で、layoutはこの入力のパイプラインのものである。
                // バイト列の長さはパイプラインレイアウトが宣言した即時定数の範囲と、シェーダー側の宣言の両方に一致する。
                unsafe {
                    device.cmd_push_constants(command_buffer, 入力.layout, vk::ShaderStageFlags::COMPUTE, 0, &バイト列);
                }
            }
            // 安全性: 直前にパイプラインとディスクリプタを束縛済みで、計算の班数は寸法から切り上げた正当な値。
            unsafe {
                device.cmd_dispatch(command_buffer, 横の計算の班数, 縦の計算の班数, 層の計算の班数);
            }
        },
    )
}
