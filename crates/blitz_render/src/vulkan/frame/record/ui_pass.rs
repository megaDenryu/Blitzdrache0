//! 開発用UI描画パスの宣言(判断33・34): シーン(+粒子)と同じカラーアタッチメントへ
//! 追記描画する(loadOpはLOAD側、深度なし)。頂点/インデックスバッファは呼び出し元が
//! 既にホスト可視バッファへ書き込み済みで、束縛は1回だけ行い、メッシュごとの範囲は
//! `cmd_draw_indexed`のfirst_index/vertex_offsetで指定する。

use ash::vk;

use crate::vulkan::frame::UI描画入力;
use crate::vulkan::graph::{クリア指定, パス宣言, パス種別, 画像ハンドル, 画像用途};

pub(super) fn 作る<'a>(カラー: 画像ハンドル, ui入力: &'a UI描画入力, 寸法: vk::Extent2D) -> パス宣言<'a> {
    パス宣言::生成する(
        "UI描画",
        Vec::new(),
        vec![(カラー, 画像用途::カラー出力)],
        Vec::new(),
        Vec::new(),
        パス種別::グラフィックス { カラー, 深度: None, クリア指定: クリア指定::ロードする },
        move |文脈| {
            let device = 文脈.device();
            let command_buffer = 文脈.コマンドバッファ();
            記録する(device, command_buffer, ui入力, 寸法);
        },
    )
}

fn 記録する(device: &ash::Device, command_buffer: vk::CommandBuffer, ui入力: &UI描画入力, 寸法: vk::Extent2D) {
    let viewport = vk::Viewport::default()
        .x(0.0)
        .y(0.0)
        .width(寸法幅をf32へ変換する(寸法.width))
        .height(寸法幅をf32へ変換する(寸法.height))
        .min_depth(0.0)
        .max_depth(1.0);
    let viewport一覧 = [viewport];
    let 画面寸法バイト列 = 画面寸法プッシュ定数バイト列を作る(寸法);
    let 頂点バッファ一覧 = [ui入力.頂点バッファ];
    let 頂点オフセット一覧 = [0u64];

    // 安全性: command_bufferは記録中で、pipeline・layout・両バッファは生成済み。
    // 頂点/インデックスバッファの束縛はここで1回だけ行い、メッシュごとの範囲は
    // 後続ループのfirst_index/vertex_offsetで指定する。
    unsafe {
        device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, ui入力.pipeline);
        device.cmd_set_viewport(command_buffer, 0, &viewport一覧);
        device.cmd_push_constants(command_buffer, ui入力.layout, vk::ShaderStageFlags::VERTEX, 0, &画面寸法バイト列);
        device.cmd_bind_vertex_buffers(command_buffer, 0, &頂点バッファ一覧, &頂点オフセット一覧);
        device.cmd_bind_index_buffer(command_buffer, ui入力.インデックスバッファ, 0, vk::IndexType::UINT32);
    }

    for 項目 in &ui入力.項目一覧 {
        let シザー一覧 = [項目.シザー];
        let ディスクリプタセット一覧 = [項目.ディスクリプタセット];
        // 安全性: command_bufferは記録中で、ディスクリプタセットは呼び出し元が
        // 今フレームぶん生成済み。
        unsafe {
            device.cmd_set_scissor(command_buffer, 0, &シザー一覧);
            device.cmd_bind_descriptor_sets(
                command_buffer,
                vk::PipelineBindPoint::GRAPHICS,
                ui入力.layout,
                0,
                &ディスクリプタセット一覧,
                &[],
            );
            device.cmd_draw_indexed(
                command_buffer,
                項目.インデックス数,
                1,
                項目.インデックス要素オフセット,
                項目.頂点要素オフセット,
                0,
            );
        }
    }
}

fn 画面寸法プッシュ定数バイト列を作る(寸法: vk::Extent2D) -> [u8; 8] {
    let 幅 = 寸法幅をf32へ変換する(寸法.width);
    let 高さ = 寸法幅をf32へ変換する(寸法.height);
    let mut バイト列 = [0u8; 8];
    バイト列[0..4].copy_from_slice(&幅.to_le_bytes());
    バイト列[4..8].copy_from_slice(&高さ.to_le_bytes());
    バイト列
}

/// ウィンドウ寸法(u32、実運用では65535を超えない)をf32へ変換する。
/// u32→f32はFrom/TryFromが無いため、u16を経由してasキャストを避ける
/// (`vulkan::frame::draw_commands`と同じ変換方針)。
fn 寸法幅をf32へ変換する(値: u32) -> f32 {
    let 値u16 = u16::try_from(値).unwrap_or_else(|_| panic!("ウィンドウ寸法がu16に収まらない: {値}"));
    f32::from(値u16)
}
