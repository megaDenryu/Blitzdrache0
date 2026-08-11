//! 深度プリパスの段階を展開する工程。受け取るのは深度のハンドルと描画対象入力、返す代わりに、方式が積むと言ったときだけグラフへ1本積む。
//!
//! 積むかどうかの判断をここが持つのは、方式がフレームの組み立て方を決める値であり、パスの宣言そのものは判断を持たないためである。
//! 積まない方式では1本も積まないため、パス別GPU計測にも深度プリパスの区間が現れない。

use ash::vk;

use super::super::base_images::基本画像ハンドル;
use crate::frame_composition::深度プリパス方式;
use crate::vulkan::frame::record::depth_prepass_pass;
use crate::vulkan::frame::描画対象入力;
use crate::vulkan::graph;

pub(in crate::vulkan::frame::record::graph_build) fn 深度プリパスを積む<'a>(
    グラフ: &mut graph::グラフ<'a>,
    基本: &基本画像ハンドル,
    スキン済み: Option<graph::バッファハンドル>,
    描画対象: 描画対象入力<'a>,
    方式: 深度プリパス方式,
    寸法: vk::Extent2D,
) {
    if !方式.深度プリパスを積むか() {
        return;
    }
    グラフ.パスを積む(depth_prepass_pass::深度プリパスを宣言する(
        基本.深度,
        スキン済み,
        描画対象.ジオメトリ,
        描画対象.共有,
        寸法,
    ));
}
