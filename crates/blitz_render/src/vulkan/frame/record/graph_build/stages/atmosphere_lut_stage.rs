//! 大気LUTの生成をレンダーグラフの具体的なパスへ展開する。担当する工程は「そのフレームで焼く組と、空段階の各パスがどのLUTを引くかに応じて、画像を登録し生成パスを宣言順に積む」ことである。受け取るのはグラフとそのフレームのLUT生成入力、返すのは各パスへ渡す読み先のハンドルと、積んだ生成パスの本数である。
//!
//! 焼かないフレームでも画像を登録しうるのは、大気LUT腕の空パスと合成パスが焼かないフレームでもLUTを引くためである。
//! 逆に誰も引かない画像は登録しない。登録だけしてどのパスも使わない資源はバリアを1本も生まず、
//! 「引く先がある」という誤った読み取りだけを型に残すためである。
//! 画像とパスの登録そのものは`register`、条件依存の3枚の腕ごとの積み方は`three_planes`、
//! 空中遠近ボリュームの積み方は`aerial_volume`が担う。

mod aerial_volume;
mod register;
mod three_planes;

use aerial_volume::空中遠近を積む;
use three_planes::条件依存の三枚を積む;

use crate::vulkan::atmosphere_lut::大気LUT描画入力;
use crate::vulkan::graph;

/// 空パスが引く2枚のLUTのハンドル。多重散乱を含めないのは、引く側が経路を刻まないためである。
#[derive(Clone, Copy)]
pub(in crate::vulkan::frame::record::graph_build) struct 大気LUT読みハンドル {
    pub(in crate::vulkan::frame::record::graph_build) 透過率: graph::画像ハンドル,
    pub(in crate::vulkan::frame::record::graph_build) スカイビュー: graph::画像ハンドル,
}

/// 空段階の各パスが引く先。2つを別のOptionで持つのは、在る条件が別だからである。空パスの2枚は方式が
/// 大気LUT腕であることだけを条件にし、合成のボリュームは合成パスが在ることを条件にする。
#[derive(Clone, Copy)]
pub(in crate::vulkan::frame::record::graph_build) struct 空段階の読み先 {
    /// 空パスがLUTを引くフレームでだけ値を持つ。
    pub(in crate::vulkan::frame::record::graph_build) 空パスの二枚: Option<大気LUT読みハンドル>,
    /// 合成パスがあるフレームでだけ値を持つ。
    pub(in crate::vulkan::frame::record::graph_build) 合成のボリューム: Option<graph::画像ハンドル>,
}

/// 大気LUTの積み上げの結果。
pub(in crate::vulkan::frame::record::graph_build) struct 大気LUT積み上げ結果 {
    pub(in crate::vulkan::frame::record::graph_build) 読み先: 空段階の読み先,
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
    合成が引くか: bool,
) -> 大気LUT積み上げ結果 {
    let Some(入力) = 入力 else {
        return 大気LUT積み上げ結果 {
            読み先: 空段階の読み先 {
                空パスの二枚: None,
                合成のボリューム: None,
            },
            生成パス数: 0,
        };
    };
    let 三枚 = 条件依存の三枚を積む(グラフ, 入力);
    let 空中遠近 = 空中遠近を積む(グラフ, 入力, 三枚.透過率, 三枚.多重散乱, 合成が引くか);
    大気LUT積み上げ結果 {
        読み先: 空段階の読み先 {
            空パスの二枚: 空パスが引くか.then_some(大気LUT読みハンドル {
                透過率: 三枚.透過率,
                スカイビュー: 三枚.スカイビュー,
            }),
            合成のボリューム: 空中遠近.ハンドル,
        },
        生成パス数: 三枚.生成パス数 + 空中遠近.生成パス数,
    }
}
