//! グラフィックスパスのdynamic rendering開始/終了。アタッチメント記述は
//! パス種別の宣言(カラー・深度・クリア指定)から実行器が組み立てる
//! （判断28: begin/end renderingは実行器の責務、クロージャはバインド+ドローのみ）。

use ash::vk;

use super::clear_spec::クリア指定;
use super::color_attachments::カラー添付列;
use super::depth_attachment::深度アタッチメント;
use super::registry::画像レジストリ;
use crate::vulkan::command_sink::GPU命令の積み先;

pub(super) fn 開始する(
    積み先: GPU命令の積み先<'_>,
    レジストリ: &画像レジストリ,
    カラー: カラー添付列,
    深度: Option<深度アタッチメント>,
    クリア指定: &クリア指定,
) {
    // 注意: render_areaはこのパスのアタッチメント自身の寸法から導出する。スワップチェーン寸法を
    // 使い回すと、寸法の異なるアタッチメント(2048x2048のシャドウマップ等)への描画がその範囲へ
    // クリップされ、範囲外がクリアもされない未定義領域になる(M6で全面影バグとして実際に踏んだ。
    // validationはrender_areaの縮小を検出しない)。
    let 基準ハンドル = カラー
        .先頭()
        .or(深度.map(|深度| 深度.ハンドル))
        .unwrap_or_else(|| panic!("グラフィックスパスにカラーも深度も無い(パス宣言の誤り)"));
    let 寸法 = レジストリ.寸法を取得する(基準ハンドル);
    let (カラーload_op, 深度load_op, カラークリア値) = ロードオペレーションとカラークリア値(クリア指定);
    let カラーアタッチメント一覧 = カラー.記述を並べる(レジストリ, カラーload_op, カラークリア値);

    let 深度クリア値 = vk::ClearValue {
        depth_stencil: vk::ClearDepthStencilValue { depth: 1.0, stencil: 0 },
    };
    // 注意: 深度のstore_opは常にSTOREにする。DONT_CAREにするとパス終了時に内容が未定義になり、
    // 後段が深度をLOADするパス(粒子)やサンプリングするパス(シャドウマップを読むシーン)が
    // 未定義の値を読む(validationは検出しない。M6で全面影バグとして実際に踏んだ)。
    // 誰も後で読まない深度のDONT_CARE化は、グラフの後続用途から導出できるようになった時点で行う。
    let 深度アタッチメント = 深度.map(|深度| {
        vk::RenderingAttachmentInfo::default()
            .image_view(深度.描画先ビュー(レジストリ.ビューを取得する(深度.ハンドル)))
            .image_layout(vk::ImageLayout::DEPTH_ATTACHMENT_OPTIMAL)
            .load_op(深度load_op)
            .store_op(vk::AttachmentStoreOp::STORE)
            .clear_value(深度クリア値)
    });

    let mut rendering_info = vk::RenderingInfo::default()
        .render_area(vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: 寸法,
        })
        .layer_count(1)
        .color_attachments(&カラーアタッチメント一覧);
    if let Some(深度アタッチメント) = &深度アタッチメント {
        rendering_info = rendering_info.depth_attachment(深度アタッチメント);
    }

    // 安全性: command_bufferは記録中で、各画像は直前のバリア発行でOPTIMALレイアウトへ
    // 遷移済み。
    unsafe { 積み先.論理デバイス().cmd_begin_rendering(積み先.コマンドバッファ(), &rendering_info) };
}

/// 返すのはカラーのloadOp・深度のloadOp・カラーのクリア値の3つである。
fn ロードオペレーションとカラークリア値(
    クリア指定: &クリア指定
) -> (vk::AttachmentLoadOp, vk::AttachmentLoadOp, vk::ClearValue) {
    let 色の値 = |カラー: &crate::clear_color::クリアカラー| vk::ClearValue {
        color: vk::ClearColorValue {
            float32: カラー.rgba配列()
        },
    };
    match クリア指定 {
        クリア指定::クリアする { カラー } => (vk::AttachmentLoadOp::CLEAR, vk::AttachmentLoadOp::CLEAR, 色の値(カラー)),
        クリア指定::カラーだけを消去して深度は読み込む { カラー } => {
            (vk::AttachmentLoadOp::CLEAR, vk::AttachmentLoadOp::LOAD, 色の値(カラー))
        }
        クリア指定::ロードする => (vk::AttachmentLoadOp::LOAD, vk::AttachmentLoadOp::LOAD, vk::ClearValue::default()),
    }
}

pub(super) fn 終了する(積み先: GPU命令の積み先<'_>) {
    // 安全性: 対応する開始する呼び出しで記録中のrenderingを閉じる。
    unsafe { 積み先.論理デバイス().cmd_end_rendering(積み先.コマンドバッファ()) };
}
