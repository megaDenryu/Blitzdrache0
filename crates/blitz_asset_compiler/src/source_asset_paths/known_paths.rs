//! 複数の世界または開発用入口が共有するソースアセットの綴りの正本。
//!
//! 綴りをこの1つのファイルへ集めるのは、同じファイルの綴りを宣言する側と複製する側が別々に持つと、
//! 片方だけを直した食い違いがファイルを開く段まで出ないためである。
//! ファイル名と、そのファイル名を含む相対パスを同じファイルへ並べるのは、綴りの正本を1箇所に保つためである。
//! smokeの検収は複製のためにファイル名だけを、宣言はソースルートからの相対パスを読む。

use super::file_name::ソースアセットのファイル名;
use super::relative_path::ソースアセットの相対パス;

impl ソースアセットのファイル名 {
    pub const 目印の柱: Self = Self::生成する("destination_marker.gltf");
    pub const 板: Self = Self::生成する("quad.gltf");
    pub const 代替板: Self = Self::生成する("quad_alt.gltf");
    pub const 影のシーン: Self = Self::生成する("shadow_scene.gltf");
    pub const 二材質: Self = Self::生成する("multi_material_two.gltf");
    pub const 代替二材質: Self = Self::生成する("multi_material_two_alt.gltf");
    pub const 単一材質: Self = Self::生成する("multi_material_one.gltf");
    pub const 遠方環境の検収: Self = Self::生成する("indirect_probe.gltf");
}

impl ソースアセットの相対パス {
    pub const フォックス: Self = Self::生成する("samples/Fox/Fox.glb");
    pub const ヘルメット: Self = Self::生成する("samples/DamagedHelmet/DamagedHelmet.glb");
    pub const 植生診断原型: Self = Self::生成する("vegetation_world/archetype_lod.gltf");
    pub const 目印の柱: Self = Self::生成する(concat!("fox_tour_world/", "destination_marker.gltf"));
    pub const 板: Self = Self::生成する(concat!("smoke/", "quad.gltf"));
    pub const 代替板: Self = Self::生成する(concat!("smoke/", "quad_alt.gltf"));
    pub const 影のシーン: Self = Self::生成する(concat!("smoke/", "shadow_scene.gltf"));
    pub const 二材質: Self = Self::生成する(concat!("smoke/", "multi_material_two.gltf"));
    pub const 代替二材質: Self = Self::生成する(concat!("smoke/", "multi_material_two_alt.gltf"));
    pub const 単一材質: Self = Self::生成する(concat!("smoke/", "multi_material_one.gltf"));
    pub const 遠方環境の検収: Self = Self::生成する(concat!("smoke/", "indirect_probe.gltf"));
    pub const 小物の柵: Self = Self::生成する("props/fence_section.glb");
    pub const 小物の樽: Self = Self::生成する("props/barrel.glb");
    pub const 小物の木箱: Self = Self::生成する("props/wooden_crate.glb");
    pub const 小物の岩: Self = Self::生成する("props/boulder.glb");
    pub const 小物の切り株: Self = Self::生成する("props/tree_stump.glb");
    pub const 小物の小石: Self = Self::生成する("props/rock.glb");
    pub const 針葉樹: Self = Self::生成する("props/conifer.glb");
}
