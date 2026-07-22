//! eguiのテクスチャデルタは部分パッチ(`pos: Some`)を送ることがあるが、blitz_render
//! のUIテクスチャAPIは全体登録のみを提供する(判断33)。ここでCPU側にテクスチャの
//! 全体像を鏡として保持し、部分パッチを反映した「常に全体を表すrgba8」へ変換する。

use std::collections::HashMap;

struct 鏡像 {
    幅: usize,
    高さ: usize,
    rgba8: Vec<u8>,
}

pub(super) struct テクスチャミラー {
    表: HashMap<egui::TextureId, 鏡像>,
}

impl テクスチャミラー {
    pub(super) fn 新規() -> Self {
        Self { 表: HashMap::new() }
    }

    /// パッチを鏡へ反映し、更新後の全体(幅・高さ・rgba8)を返す。
    pub(super) fn 反映して全体を得る(&mut self, id: egui::TextureId, delta: &egui::epaint::ImageDelta) -> (usize, usize, Vec<u8>) {
        let [パッチ幅, パッチ高さ] = delta.image.size();
        let パッチrgba8 = 画像データをrgba8へ変換する(&delta.image);

        match delta.pos {
            None => {
                self.表.insert(
                    id,
                    鏡像 {
                        幅: パッチ幅,
                        高さ: パッチ高さ,
                        rgba8: パッチrgba8.clone(),
                    },
                );
                (パッチ幅, パッチ高さ, パッチrgba8)
            }
            Some([x, y]) => {
                let 鏡 = self.表.entry(id).or_insert_with(|| 鏡像 {
                    幅: パッチ幅,
                    高さ: パッチ高さ,
                    rgba8: パッチrgba8.clone(),
                });
                部分書き込みする(&mut 鏡.rgba8, 鏡.幅, x, y, パッチ幅, パッチ高さ, &パッチrgba8);
                (鏡.幅, 鏡.高さ, 鏡.rgba8.clone())
            }
        }
    }

    pub(super) fn 削除する(&mut self, id: egui::TextureId) {
        self.表.remove(&id);
    }
}

fn 部分書き込みする(
    全体rgba8: &mut [u8], 全体幅: usize, x: usize, y: usize, パッチ幅: usize, パッチ高さ: usize, パッチrgba8: &[u8]
) {
    for 行 in 0..パッチ高さ {
        let 全体オフセット = ((y + 行) * 全体幅 + x) * 4;
        let パッチオフセット = 行 * パッチ幅 * 4;
        全体rgba8[全体オフセット..全体オフセット + パッチ幅 * 4].copy_from_slice(&パッチrgba8[パッチオフセット..パッチオフセット + パッチ幅 * 4]);
    }
}

fn 画像データをrgba8へ変換する(データ: &egui::ImageData) -> Vec<u8> {
    match データ {
        egui::ImageData::Color(画像) => 画像.pixels.iter().flat_map(|色| 色.to_array()).collect(),
        egui::ImageData::Font(画像) => 画像.srgba_pixels(None).flat_map(|色| 色.to_array()).collect(),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::部分書き込みする;

    #[test]
    fn 指定位置にだけパッチが書き込まれる() {
        let 全体幅 = 4;
        let mut 全体 = vec![0u8; 全体幅 * 2 * 4];
        let パッチ = vec![9u8; 2 * 4];
        部分書き込みする(&mut 全体, 全体幅, 1, 1, 2, 1, &パッチ);
        let 開始 = (全体幅 + 1) * 4;
        assert_eq!(&全体[開始..開始 + 8], &パッチ[..]);
        assert_eq!(&全体[0..4], &[0, 0, 0, 0]);
    }
}
