//! 自動露出の3本のパスの宣言。消去でヒストグラムを0へ戻し、集計でHDR中間画像を数え、導出と適応で露出状態を更新する。
//! 3本を1つのファイルへ置くのは、この順そのものが担当する関心事だからである。消去を落とせば前フレームの件数へ積み増した分布になり、導出を集計より前へ置けば1フレーム前の分布から補正段を作る。
//! 注意: 集計はHDR中間画像を`シェーダー読みコンピュート段`で読む。光のにじみの縮小チェーンが同じ画像を画素段で読むため、同じレイアウトの読みどうしになりバリアが省ける。

use ash::vk;

use crate::vulkan::auto_exposure::{ヒストグラムのバイト数, 自動露出描画入力};
use crate::vulkan::graph::{
    バッファハンドル, バッファ用途, パス宣言, パス種別, 画像ハンドル, 画像用途, 記録文脈
};

/// 集計の1班が覆う画素の一辺。`shaders/auto_exposure_histogram.slang`の`threadsPerSide`と一致させる。
const 集計の班の一辺: u32 = 16;

pub(super) fn 消去を作る<'a>(ヒストグラム: バッファハンドル) -> パス宣言<'a> {
    パス宣言::生成する(
        "自動露出のヒストグラム消去",
        Vec::new(),
        Vec::new(),
        Vec::new(),
        vec![(ヒストグラム, バッファ用途::消去の書き込み)],
        パス種別::転送,
        move |文脈| {
            let バッファ = 文脈.バッファを解決する(ヒストグラム);
            // 安全性: コマンドバッファは記録中で、埋める長さは確保長ぴったりである。
            unsafe {
                文脈
                    .device()
                    .cmd_fill_buffer(文脈.コマンドバッファ(), バッファ, 0, ヒストグラムのバイト数(), 0)
            };
        },
    )
}

pub(super) fn 集計を作る<'a>(
    hdr: 画像ハンドル,
    ヒストグラム: バッファハンドル,
    入力: &'a 自動露出描画入力,
    寸法: vk::Extent2D,
) -> パス宣言<'a> {
    パス宣言::生成する(
        "自動露出のヒストグラム集計",
        vec![(hdr, 画像用途::シェーダー読みコンピュート段)],
        Vec::new(),
        Vec::new(),
        vec![(ヒストグラム, バッファ用途::コンピュート読み書き)],
        パス種別::コンピュート,
        move |文脈| {
            let _ = 文脈.バッファを解決する(ヒストグラム);
            let 定数: Vec<u8> = [寸法.width, 寸法.height].iter().flat_map(|値| 値.to_le_bytes()).collect();
            let 班数 = [寸法.width.div_ceil(集計の班の一辺), 寸法.height.div_ceil(集計の班の一辺), 1];
            コンピュートを積む(文脈, 入力.集計pipeline, 入力.集計layout, 入力.セット, &定数, 班数);
        },
    )
}

pub(super) fn 導出と適応を作る<'a>(
    ヒストグラム: バッファハンドル,
    露出状態: バッファハンドル,
    入力: &'a 自動露出描画入力,
) -> パス宣言<'a> {
    パス宣言::生成する(
        "自動露出の導出と適応",
        Vec::new(),
        Vec::new(),
        vec![(ヒストグラム, バッファ用途::コンピュート読み)],
        vec![(露出状態, バッファ用途::コンピュート読み書き)],
        パス種別::コンピュート,
        move |文脈| {
            let _ = 文脈.バッファを解決する(露出状態);
            let 定数: Vec<u8> = 入力.導出の即時定数.iter().flat_map(|値| 値.to_le_bytes()).collect();
            コンピュートを積む(文脈, 入力.導出pipeline, 入力.導出layout, 入力.セット, &定数, [1, 1, 1]);
        },
    )
}

fn コンピュートを積む(
    文脈: &記録文脈,
    pipeline: vk::Pipeline,
    layout: vk::PipelineLayout,
    セット: vk::DescriptorSet,
    定数: &[u8],
    班数: [u32; 3],
) {
    let device = 文脈.device();
    let command_buffer = 文脈.コマンドバッファ();
    let セット一覧 = [セット];
    // 安全性: command_bufferは記録中で、pipeline・layout・セットは生成済み。定数の長さはレイアウトが宣言した範囲と一致する。
    unsafe {
        device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::COMPUTE, pipeline);
        device.cmd_bind_descriptor_sets(command_buffer, vk::PipelineBindPoint::COMPUTE, layout, 0, &セット一覧, &[]);
        device.cmd_push_constants(command_buffer, layout, vk::ShaderStageFlags::COMPUTE, 0, 定数);
        device.cmd_dispatch(command_buffer, 班数[0], 班数[1], 班数[2]);
    }
}
