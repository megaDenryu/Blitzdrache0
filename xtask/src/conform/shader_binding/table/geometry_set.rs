//! ジオメトリと可視のセット(set1)の束縛番号の正本と、その番号を宣言するシェーダーの対応。
//! 触れるのは頂点段が読む2本のストレージバッファだけであり、どちらも同じ正本のファイルに並ぶ。

use super::束縛番号の組;

pub(super) const 束縛番号の組の一覧: [束縛番号の組; 2] = [
    束縛番号の組 {
        正本パス: "crates/blitz_render/src/vulkan/descriptor/geometry_set.rs",
        正本の前置き: "pub(crate) const 個体レコードの束縛番号: 束縛番号 = 束縛番号::生成する",
        写しパス: "shaders/instance_transform.slang",
        写しの変数名: "instanceTransforms",
        セット番号: 1,
    },
    束縛番号の組 {
        正本パス: "crates/blitz_render/src/vulkan/descriptor/geometry_set.rs",
        正本の前置き: "pub(crate) const 可視ID列の束縛番号: 束縛番号 = 束縛番号::生成する",
        写しパス: "shaders/visible_id.slang",
        写しの変数名: "visibleInstanceIds",
        セット番号: 1,
    },
];
