//! 遠方環境を鏡面畳込みの最詳細段へ複製する転送パスの宣言。担当する工程は「2つの画像ハンドルと1層の範囲と
//! 層数を受け取り、転送種別のパス宣言を返す」ことである。
//!
//! 最詳細段を計算でなく複製で作るのは、粗さ0の畳み込みが元の画像そのものだからである。複製ならビット単位で
//! 一致するため、検収が「最詳細段が遠方環境と一致する」ことを丸めの許容なしで判定できる。

use ash::vk;

use super::層の部分範囲;
use crate::vulkan::graph;

/// GPU計器の区間名。間接照明の生成の合計を組む宣言がこの定数を読む(綴りを2箇所で持たないため)。
pub(crate) const 最詳細段の複製のパス名: &str = "鏡面畳込み最詳細段の複製";

/// 前提: 2つの画像は同じ画素形式と同じ一辺を持つ(呼び出し元が解像度の一致を型付きの失敗で確かめる)。
pub(in crate::vulkan) fn 最詳細段の複製を作る(
    元: graph::画像ハンドル,
    先: graph::画像ハンドル,
    範囲: vk::Extent3D,
    層数: u32,
) -> graph::パス宣言<'static> {
    graph::パス宣言::生成する(
        最詳細段の複製のパス名,
        vec![(元, graph::画像用途::転送元)],
        vec![(先, graph::画像用途::転送先)],
        Vec::new(),
        Vec::new(),
        graph::パス種別::転送,
        move |文脈| {
            let 元の画像 = 文脈.画像を解決する(元);
            let 先の画像 = 文脈.画像を解決する(先);
            let 領域 = [vk::ImageCopy::default()
                .src_subresource(層の部分範囲(0, 層数))
                .dst_subresource(層の部分範囲(0, 層数))
                .extent(範囲)];
            // 安全性: command_bufferは記録中、2つの画像は用途宣言からグラフが導いた転送元・転送先のレイアウトへ遷移済みである。
            unsafe {
                文脈.device().cmd_copy_image(
                    文脈.コマンドバッファ(),
                    元の画像,
                    vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                    先の画像,
                    vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                    &領域,
                );
            }
        },
    )
}
