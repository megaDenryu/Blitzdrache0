//! 先頭メッシュのプリミティブの検査。担当するのは描画の並び方(mode)と、UV座標・法線・接線の宣言である。
//! 先頭メッシュに限るのは、ローダーが先頭メッシュしか読まないためである(2個目以降は`document_scan`が違反として出す)。
//! 由来は`loader::mesh::primitive`が並び方を読まずに三角形として扱い、UV欠落を(0,0)で充填し、法線・接線の欠落を計算で充填することである。
//! 材質の割り当て忘れは`material_declaration_scan`が、プリミティブが焼いた結果へ残ったかは`primitive_preservation`が見る。

use super::inspection::開いた文書の契約検査;
use super::target::{名前を写す, 対象位置};

impl 開いた文書の契約検査<'_> {
    pub(super) fn 先頭メッシュのプリミティブを検査する(&mut self) {
        let Some(メッシュ) = self.文書の全体().meshes().next() else {
            return;
        };
        let プリミティブ一覧: Vec<gltf::Primitive<'_>> = メッシュ.primitives().collect();
        if プリミティブ一覧.is_empty() {
            self.違反を記す(
                対象位置::メッシュ {
                    添字: 0,
                    名前: 名前を写す(メッシュ.name()),
                },
                "プリミティブが1つも無い",
                "面を持つメッシュを書き出す。Blenderで頂点だけのオブジェクトや空オブジェクトを書き出していないかを確かめる",
            );
            return;
        }

        for (添字, プリミティブ) in プリミティブ一覧.iter().enumerate() {
            self.描画の並び方を検査する(添字, プリミティブ);
            self.頂点属性の宣言を検査する(添字, プリミティブ);
        }
    }

    /// ローダーはインデックス列を三角形リストとしてそのまま渡す。三角形以外の並び方は絵が崩れる。
    fn 描画の並び方を検査する(&mut self, 添字: usize, プリミティブ: &gltf::Primitive<'_>) {
        if プリミティブ.mode() == gltf::mesh::Mode::Triangles {
            return;
        }
        self.違反を記す(
            位置(添字),
            format!(
                "描画の並び方が{:?}である。ローダーはインデックス列を三角形リストとして渡す",
                プリミティブ.mode()
            ),
            "三角形として書き出す。Blenderの書き出し設定で三角形化を有効にし、線や点のオブジェクトを含めない",
        );
    }

    fn 頂点属性の宣言を検査する(&mut self, 添字: usize, プリミティブ: &gltf::Primitive<'_>) {
        let uvがある = プリミティブ.get(&gltf::Semantic::TexCoords(0)).is_some();
        let 材質 = プリミティブ.material();
        let pbr = 材質.pbr_metallic_roughness();
        let テクスチャがある = pbr.base_color_texture().is_some() || pbr.metallic_roughness_texture().is_some() || 材質.normal_texture().is_some();

        if テクスチャがある && !uvがある {
            self.違反を記す(
                位置(添字),
                "マテリアルがテクスチャを持つのにUV座標(TEXCOORD_0)が無い。ローダーはUVを(0,0)で充填するため、絵は画像の1テクセルの色で塗り潰される",
                "BlenderでUV展開してから、書き出し設定でUV座標を含める",
            );
        }
        if 材質.normal_texture().is_some() && プリミティブ.get(&gltf::Semantic::Tangents).is_none() {
            self.違反を記す(
                位置(添字),
                "法線マップを持つのに接線(TANGENT)が無い。ローダーは法線に直交する任意の接線を構成するため、法線マップの向きが作者の意図と一致しない",
                "Blenderの書き出し設定で接線を含める。UVが無いと接線は書き出されないため、先にUV展開する",
            );
        }
        if プリミティブ.get(&gltf::Semantic::Normals).is_none() {
            self.警告を記す(
                位置(添字),
                "法線(NORMAL)が無い。ローダーは面積で重み付けした滑らかな法線を計算して充填するため、平らな面の陰影が作者の指定と変わる",
                "Blenderの書き出し設定で法線を含める",
            );
        }
    }
}

fn 位置(添字: usize) -> 対象位置 {
    対象位置::プリミティブ {
        メッシュ添字: 0, 添字
    }
}
