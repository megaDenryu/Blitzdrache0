//! 大気のベイク済み画像の生成をレンダーグラフの具体的なパスへ展開する。担当する工程は「そのフレームで焼く組と、空段階の各パスがどのベイク済み画像を参照するかに応じて、画像を登録し生成パスを宣言順に積む」ことである。受け取るのはグラフとそのフレームのベイク済み画像生成入力、返すのは各パスへ渡す読み先のハンドルと、積んだ生成パスの本数である。
//!
//! 焼かないフレームでも画像を登録しうるのは、大気のベイク済み画像方式の空パスと合成パスが焼かないフレームでもベイク済み画像を参照するためである。
//! 逆に誰も参照しない画像は登録しない。登録だけしてどのパスも使わない資源はバリアを1本も生まず、
//! 「参照する先がある」という誤った読み取りだけを型に残すためである。
//! 画像とパスの登録そのものは`register`、条件依存の3枚の枝ごとの積み方は`three_planes`、
//! 空中遠近ボリュームの積み方は`aerial_volume`が担う。

mod aerial_volume;
mod register;
mod three_planes;

use aerial_volume::空中遠近を積む;
use three_planes::条件依存の三枚を積む;

use crate::vulkan::atmosphere_lut::大気のベイク済み画像の描画入力;
use crate::vulkan::graph;

/// 空パスが参照する2枚のベイク済み画像のハンドル。多重散乱を含めないのは、参照する側が経路を刻まないためである。
#[derive(Clone, Copy)]
pub(in crate::vulkan::frame::record::graph_build) struct 大気のベイク済み画像読みハンドル {
    pub(in crate::vulkan::frame::record::graph_build) 透過率: graph::画像ハンドル,
    pub(in crate::vulkan::frame::record::graph_build) スカイビュー: graph::画像ハンドル,
}

/// 経路を刻む生成が読む2枚のハンドル。空パスの2枚と別に持つのは、読む側も組み合わせも違うためである。
/// 遠方環境用スカイビューの生成がこの2枚を読む。
#[derive(Clone, Copy)]
pub(in crate::vulkan::frame::record::graph_build) struct 経路生成が読む二枚 {
    pub(in crate::vulkan::frame::record::graph_build) 透過率: graph::画像ハンドル,
    pub(in crate::vulkan::frame::record::graph_build) 多重散乱: graph::画像ハンドル,
}

/// 空段階の各パスが参照する先。2つを別のOptionで持つのは、在る条件が別だからである。空パスの2枚は方式が
/// 大気のベイク済み画像方式であることだけを条件にし、合成のボリュームは合成パスが在ることを条件にする。
#[derive(Clone, Copy)]
pub(in crate::vulkan::frame::record::graph_build) struct 空段階の読み先 {
    /// 空パスがベイク済み画像を参照するフレームでだけ値を持つ。
    pub(in crate::vulkan::frame::record::graph_build) 空パスの二枚: Option<大気のベイク済み画像読みハンドル>,
    /// 合成パスがあるフレームでだけ値を持つ。
    pub(in crate::vulkan::frame::record::graph_build) 合成のボリューム: Option<graph::画像ハンドル>,
}

/// 大気のベイク済み画像の積み上げの結果。
pub(in crate::vulkan::frame::record::graph_build) struct 大気のベイク済み画像積み上げ結果 {
    pub(in crate::vulkan::frame::record::graph_build) 読み先: 空段階の読み先,
    /// 透過率と多重散乱を登録したハンドル。ベイク済み画像を作らない構成では`None`である。
    /// 空パスの読み先と別に持つのは、遠方環境用スカイビューの生成が空パスの有無に関わらずこの2枚を読むためである。
    pub(in crate::vulkan::frame::record::graph_build) 経路の二枚: Option<経路生成が読む二枚>,
    /// このフレームで積んだ生成パスの本数。0から4のいずれかであり、計器がこの値を数える。
    pub(in crate::vulkan::frame::record::graph_build) 生成パス数: u32,
}

/// 大気のベイク済み画像の生成を積む。宣言順を透過率→多重散乱→スカイビュー→空中遠近に固定するのは、後段のベイク済み画像が前段のベイク済み画像を読むためである。
/// レンダーグラフは宣言順に実行してバリアだけを導くため、この順序を依存解析に任せない
/// (参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「大気のベイク済み画像方式の設計(第7段で実装する)」)。
pub(in crate::vulkan::frame::record::graph_build) fn 大気のベイク済み画像を積む<'a>(
    グラフ: &mut graph::グラフ<'a>,
    入力: Option<&'a 大気のベイク済み画像の描画入力>,
    空パスが参照するか: bool,
    合成が参照するか: bool,
) -> 大気のベイク済み画像積み上げ結果 {
    let Some(入力) = 入力 else {
        return 大気のベイク済み画像積み上げ結果 {
            読み先: 空段階の読み先 {
                空パスの二枚: None,
                合成のボリューム: None,
            },
            経路の二枚: None,
            生成パス数: 0,
        };
    };
    let 三枚 = 条件依存の三枚を積む(グラフ, 入力);
    let 空中遠近 = 空中遠近を積む(グラフ, 入力, 三枚.透過率, 三枚.多重散乱, 合成が参照するか);
    大気のベイク済み画像積み上げ結果 {
        読み先: 空段階の読み先 {
            空パスの二枚: 空パスが参照するか.then_some(大気のベイク済み画像読みハンドル {
                透過率: 三枚.透過率,
                スカイビュー: 三枚.スカイビュー,
            }),
            合成のボリューム: 空中遠近.ハンドル,
        },
        経路の二枚: Some(経路生成が読む二枚 {
            透過率: 三枚.透過率,
            多重散乱: 三枚.多重散乱,
        }),
        生成パス数: 三枚.生成パス数 + 空中遠近.生成パス数,
    }
}
