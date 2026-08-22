//! チャンク1件が持つ素材のファイル名。高さ格子・地表材質の重み格子・版付きチャンクソースの3つの綴りを、
//! チャンクの添字から導く。
//!
//! 綴りを1つの型へ閉じるのは、世界のソースを書き出す経路が2つ(プロジェクトの世界ぜんたいと、建物1棟だけの
//! 検証の世界)あり、どちらも同じ名前で書かなければ焼く側が素材を開けないためである。裸の`format!`を
//! 呼び出し側へ散らすと、片方の綴りを直したときにもう片方が取り残される。

/// チャンクの素材のファイル名とは、チャンク1件の書き出しが作る3つのファイルの名前のことである。
pub(super) struct チャンクの素材のファイル名 {
    x: i32,
    z: i32,
}

impl チャンクの素材のファイル名 {
    pub(super) fn チャンクの添字から生成する(x: i32, z: i32) -> Self {
        Self { x, z }
    }

    /// 目録がこのチャンクを指すときに書くアセットの名前。高さ格子のファイル名から拡張子を除いた形である。
    pub(super) fn 目録が指すアセットの名前(&self) -> String {
        format!("editor_terrain_x{}_z{}", self.x, self.z)
    }

    pub(super) fn 高さ格子(&self) -> String {
        format!("{}.heightgrid", self.目録が指すアセットの名前())
    }

    pub(super) fn 地表材質の重み格子(&self) -> String {
        format!("{}.surfaceweights", self.目録が指すアセットの名前())
    }

    pub(super) fn 版付きチャンクソース(&self) -> String {
        format!("editor_chunk_x{}_z{}.json", self.x, self.z)
    }
}
