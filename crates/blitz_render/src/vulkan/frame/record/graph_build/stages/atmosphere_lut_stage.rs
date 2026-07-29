//! 大気LUTの生成をレンダーグラフの具体的なパスへ展開する。担当する工程は「LUTを焼くフレームで生成パスを宣言順に積む」ことである。
//! 受け取るのはグラフとそのフレームのLUT生成入力、返り値は無く、副作用はグラフへの画像登録とパスの追加だけである。

use crate::vulkan::atmosphere_lut::大気LUT描画入力;
use crate::vulkan::frame::record::atmosphere_lut_pass;
use crate::vulkan::graph;

/// 大気LUTの生成を積む。宣言順を透過率が先になるようにするのは、後段のLUTが透過率LUTを読むためである。
/// レンダーグラフは宣言順に実行してバリアだけを導くため、この順序を依存解析に任せない
/// (参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「大気LUT方式の設計(第7段で実装する)」)。
pub(in crate::vulkan::frame::record::graph_build) fn 大気lutを積む<'a>(
    グラフ: &mut graph::グラフ<'a>, 入力: Option<&'a 大気LUT描画入力>
) {
    let Some(入力) = 入力 else {
        return;
    };
    let 透過率 = グラフ.画像を登録する(
        入力.透過率画像,
        入力.透過率ビュー,
        graph::画像アスペクト::カラー,
        graph::前フレーム大気lut書き直後状態(),
        入力.透過率寸法,
    );
    let 多重散乱 = グラフ.画像を登録する(
        入力.多重散乱画像,
        入力.多重散乱ビュー,
        graph::画像アスペクト::カラー,
        graph::前フレーム大気lut書き直後状態(),
        入力.多重散乱寸法,
    );
    グラフ.パスを積む(atmosphere_lut_pass::作る("透過率生成", 透過率, Vec::new(), &入力.透過率));
    let 透過率を読む = vec![(透過率, graph::画像用途::コンピュート読み)];
    グラフ.パスを積む(atmosphere_lut_pass::作る("多重散乱生成", 多重散乱, 透過率を読む, &入力.多重散乱));
}
