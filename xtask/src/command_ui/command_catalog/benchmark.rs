//! 性能計測系コマンドの一覧。担当するのは固定シーン・大規模世界・原点移動・LODの一括計測など、
//! 特定の律速要因を切り分けない全体計測の10件である(切り分け計測は`measurement`が持つ)。
//! 引数を解釈するのは2件だけであり、その定義はコマンドごとの別ファイルが持つ。

mod distant_view_arguments;
mod large_world_bench_arguments;

use super::entry::コマンド項目;

pub(super) const 一覧: &[コマンド項目] = &[
    コマンド項目::引数なしで生成する(
        "固定シーンの性能計測",
        "  bench            リリース版の固定シーンを600フレーム実行し、GPU時間とCPU側フレーム間隔分布を表示する",
    ),
    コマンド項目::引数なしで生成する(
        "実表示間隔つき性能計測",
        "  bench-display-timing  benchに実表示間隔の計測を足して実行する(計測が描画ループを止めるため既存の時系列とは比較できない)",
    ),
    コマンド項目::引数なしで生成する(
        "流体試作の性能計測",
        "  m10-bench        M10流体GPU試作を固定条件で実行し、検証件数とGPU時間を表示する",
    ),
    コマンド項目::引数なしで生成する(
        "連続実行のメモリ推移計測",
        "  m11-soak         3600フレーム連続実行し、RAM・VRAM推移を約5秒間隔で表示する",
    ),
    コマンド項目::引数なしで生成する(
        "対象数別の性能計測",
        "  object-bench     二対象の画素判定後、1・10・100対象のGPU/CPU時間とGPUメモリを計測する",
    ),
    コマンド項目::引数なしで生成する(
        "原点移動の不変性確認",
        "  origin-invariance  世界全体へkm級の大域平行移動を加えても読み戻し画像がバイト一致し、カメラだけを微小に動かすと変わることを確かめる",
    ),
    コマンド項目::引数なしで生成する(
        "地形段差の継ぎ目確認",
        "  lod-crack        地形LODの段差0・1・最大を四方向と細粗入替でGPU描画し、内側の継ぎ目に背景色が露出しないことを確かめる",
    ),
    コマンド項目::引数定義を添えて生成する(
        "大規模世界の固定経路計測",
        "  large-world-bench [条件...]  8km×10km世界をkm級固定経路で3回走らせ、CPU区間・GPU時間・会計・validation・p50/p99を表示する(--print-planはGPUを起動しない)",
        large_world_bench_arguments::大規模世界の固定経路計測の引数定義,
    ),
    コマンド項目::引数定義を添えて生成する(
        "遠景の撮影と判定",
        "  distant-view <--capture-reference|--capture-candidate|--capture-reference-no-ssao|--capture-candidate-no-ssao|--capture-candidate-no-distant-shadow|--capture-reference-no-post|--capture-candidate-no-post|--capture-shadow-reference|--capture-shadow-candidate|--capture-shadow-reference-visibility|--capture-shadow-candidate-visibility|--capture-scatter-reference|--capture-scatter-candidate|--capture-scatter-reference-no-post|--capture-scatter-candidate-no-post|--capture-scatter-reference-bare|--capture-scatter-candidate-bare|--bake-scatter-reference|--judge|--judge-shadow|--judge-scatter|--print-plan>  8km×10km固定構図を撮影し、遠景の検査点(--judge)を2段で、影の距離区分の再配分の検査点(--judge-shadow)を深度一致と影可視度への帰属で、散布の検査点(--judge-scatter)を機構ごとの3段で判定する(--bake-scatter-referenceは散布を焼かない対照のアセットを焼く)",
        distant_view_arguments::遠景の撮影と判定の引数定義,
    ),
    コマンド項目::引数なしで生成する(
        "原点移動と地形段の一括計測",
        "  ow3-dod          原点移動・LOD継ぎ目・半径2ストリーミングを本番経路でまとめて測り、複数LOD画像をPNGへ書き出す",
    ),
];
