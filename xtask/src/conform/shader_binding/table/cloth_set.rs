//! 布シミュレーションの統一セット(set0)の束縛番号の正本と、その番号を宣言するシェーダーの対応。
//! 正本は布のディスクリプタ1ファイルであり、写しは工程ごとのシェーダーに分かれる。
//! 同じ束縛を複数のファイルが宣言するものは、最初に宣言するファイル1つを写しとして突き合わせる。

use super::束縛番号の組;

const 正本: &str = "crates/blitz_render/src/vulkan/cloth/descriptor.rs";
const 介入と積分: &str = "shaders/cloth_step.slang";
const 拘束: &str = "shaders/cloth_constraint.slang";
const 空間ハッシュ: &str = "shaders/cloth_hash.slang";
const アタッチ: &str = "shaders/cloth_attach.slang";

const fn 組(前置き: &'static str, 写しパス: &'static str, 写しの変数名: &'static str) -> 束縛番号の組 {
    束縛番号の組 {
        正本パス: 正本,
        正本の前置き: 前置き,
        写しパス,
        写しの変数名,
        セット番号: 0,
    }
}

pub(super) const 束縛番号の組の一覧: [束縛番号の組; 11] = [
    組("pub(crate) const 定数の束縛番号: 束縛番号 = 束縛番号::生成する", 介入と積分, "params"),
    組("pub(crate) const 粒子の束縛番号: 束縛番号 = 束縛番号::生成する", 介入と積分, "particles"),
    組(
        "pub(crate) const 前位置の束縛番号: 束縛番号 = 束縛番号::生成する",
        介入と積分,
        "previousPositions",
    ),
    組(
        "pub(crate) const 介入の束縛番号: 束縛番号 = 束縛番号::生成する",
        介入と積分,
        "interventions",
    ),
    組(
        "pub(crate) const 拘束の引数の束縛番号: 束縛番号 = 束縛番号::生成する",
        拘束,
        "constraints",
    ),
    組(
        "pub(crate) const セルカウントの束縛番号: 束縛番号 = 束縛番号::生成する",
        空間ハッシュ,
        "cellCounts",
    ),
    組(
        "pub(crate) const セル格納の束縛番号: 束縛番号 = 束縛番号::生成する",
        空間ハッシュ,
        "cellSlots",
    ),
    組(
        "pub(crate) const 布頂点の束縛番号: 束縛番号 = 束縛番号::生成する",
        "shaders/cloth_vertex.slang",
        "clothVertices",
    ),
    組(
        "pub(crate) const スキン済み頂点の束縛番号: 束縛番号 = 束縛番号::生成する",
        アタッチ,
        "skinnedVertices",
    ),
    組(
        "pub(crate) const アタッチ対応の束縛番号: 束縛番号 = 束縛番号::生成する",
        アタッチ,
        "attachPairs",
    ),
    組(
        "pub(crate) const ラグランジュ乗数の束縛番号: 束縛番号 = 束縛番号::生成する",
        拘束,
        "lambdas",
    ),
];
