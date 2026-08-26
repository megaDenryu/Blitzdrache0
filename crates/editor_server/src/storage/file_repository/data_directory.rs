//! 編集データディレクトリの型。役割を持つパスであり、その配下の各資源ファイルの
//! パスの綴りをこの型のメソッドの1箇所へ閉じる(グローバルCLAUDE.md「プリミティブ執着禁止は
//! パス・テキスト・名前にも適用する」)。`world`・`chunk`は生の`&Path`へ剥がした綴りを
//! 自分では組み立てず、このメソッドを呼ぶだけにする。

use std::path::PathBuf;

use super::file_names::{マテリアル台帳ファイル名, 材質重みファイル名, 構造ファイル名, 高さ格子ファイル名};
use crate::{
    project_root::プロジェクトルート,
    resource::{チャンク座標, 楽曲ID},
};

#[derive(Clone)]
pub(super) struct 編集データディレクトリ(PathBuf);

impl 編集データディレクトリ {
    pub(super) fn 生成する(プロジェクトルート: &プロジェクトルート) -> Self {
        Self(プロジェクトルート.パス().join("editor_data"))
    }

    pub(super) fn 大域世界構造パス(&self) -> PathBuf {
        self.0.join("大域世界").join(構造ファイル名)
    }

    pub(super) fn 大域世界高さ格子パス(&self) -> PathBuf {
        self.0.join("大域世界").join(高さ格子ファイル名)
    }

    pub(super) fn マテリアル台帳パス(&self) -> PathBuf {
        self.0.join(マテリアル台帳ファイル名)
    }

    pub(super) fn 楽曲ディレクトリ(&self) -> PathBuf {
        self.0.join("楽曲")
    }

    pub(super) fn 楽曲パス(&self, 名乗り: &楽曲ID) -> PathBuf {
        self.楽曲ディレクトリ().join(format!("{名乗り}.json"))
    }

    fn チャンクディレクトリ(&self, 座標: チャンク座標) -> PathBuf {
        self.0.join("チャンク").join(座標.アンダースコア区切り表記())
    }

    pub(super) fn チャンク構造パス(&self, 座標: チャンク座標) -> PathBuf {
        self.チャンクディレクトリ(座標).join(構造ファイル名)
    }

    pub(super) fn チャンク高さ格子パス(&self, 座標: チャンク座標) -> PathBuf {
        self.チャンクディレクトリ(座標).join(高さ格子ファイル名)
    }

    pub(super) fn チャンク材質重みパス(&self, 座標: チャンク座標) -> PathBuf {
        self.チャンクディレクトリ(座標).join(材質重みファイル名)
    }
}
