//! 大気LUTの生成をレンダーグラフの具体的なパスへ展開する。担当する工程は「そのフレームで焼く組と、空パスがLUTを引くかどうかに応じて、画像を登録し生成パスを宣言順に積む」ことである。受け取るのはグラフとそのフレームのLUT生成入力、返すのは空パスへ渡す読み先のハンドルと、積んだ生成パスの本数である。
//!
//! 焼かないフレームでも画像を登録しうるのは、大気LUT腕の空パスが焼かないフレームでもLUTを引くためである。
//! 画像とパスの登録そのものは`register`、条件依存の3枚の腕ごとの積み方は`three_planes`が担う。

mod register;
mod three_planes;

use register::{二枚を読む, 焼く画像を登録する, 生成パスを積む};
use three_planes::条件依存の三枚を積む;

use crate::vulkan::atmosphere_lut::大気LUT描画入力;
use crate::vulkan::graph;

/// 空パスが引く2枚のLUTのハンドル。多重散乱を含めないのは、引く側が経路を刻まないためである。
#[derive(Clone, Copy)]
pub(in crate::vulkan::frame::record::graph_build) struct 大気LUT読みハンドル {
    pub(in crate::vulkan::frame::record::graph_build) 透過率: graph::画像ハンドル,
    pub(in crate::vulkan::frame::record::graph_build) スカイビュー: graph::画像ハンドル,
}

/// 大気LUTの積み上げの結果。読み先のハンドルは、空パスがLUTを引くフレームでだけ値を持つ。
pub(in crate::vulkan::frame::record::graph_build) struct 大気LUT積み上げ結果 {
    pub(in crate::vulkan::frame::record::graph_build) 読みハンドル: Option<大気LUT読みハンドル>,
    /// このフレームで積んだ生成パスの本数。0から4のいずれかであり、計器がこの値を数える。
    pub(in crate::vulkan::frame::record::graph_build) 生成パス数: u32,
}

/// 大気LUTの生成を積む。宣言順を透過率→多重散乱→スカイビュー→空中遠近に固定するのは、後段のLUTが前段のLUTを読むためである。
/// レンダーグラフは宣言順に実行してバリアだけを導くため、この順序を依存解析に任せない
/// (参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「大気LUT方式の設計(第7段で実装する)」)。
pub(in crate::vulkan::frame::record::graph_build) fn 大気lutを積む<'a>(
    グラフ: &mut graph::グラフ<'a>,
    入力: Option<&'a 大気LUT描画入力>,
    空パスが引くか: bool,
) -> 大気LUT積み上げ結果 {
    let Some(入力) = 入力 else {
        return 大気LUT積み上げ結果 {
            読みハンドル: None,
            生成パス数: 0,
        };
    };
    let 三枚 = 条件依存の三枚を積む(グラフ, 入力, 空パスが引くか);
    let 空中遠近の本数 = 空中遠近を積む(グラフ, 入力, 三枚.透過率, 三枚.多重散乱);
    大気LUT積み上げ結果 {
        読みハンドル: 三枚.読みハンドル,
        生成パス数: 三枚.生成パス数 + 空中遠近の本数,
    }
}

/// 空中遠近を積む。宣言順でスカイビューの後ろへ置くのは設計正本の固定順に従うためであり、依存の上では
/// スカイビューの結果を読まないため前後どちらでも成り立つ。
fn 空中遠近を積む<'a>(
    グラフ: &mut graph::グラフ<'a>,
    入力: &'a 大気LUT描画入力,
    透過率: graph::画像ハンドル,
    多重散乱: graph::画像ハンドル,
) -> u32 {
    let Some(空中遠近入力) = &入力.空中遠近 else {
        return 0;
    };
    let 書き先 = 焼く画像を登録する(グラフ, &入力.空中遠近画像);
    生成パスを積む(グラフ, "空中遠近生成", 書き先, 二枚を読む(透過率, 多重散乱), 空中遠近入力);
    1
}
