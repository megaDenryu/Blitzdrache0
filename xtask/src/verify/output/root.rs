//! 検証の出力ルート: 検証の出力とビルドの中間データが載る木の根を表す型。
//!
//! **この木の名前の綴りを持つのは、リポジトリの中でこのファイルだけである。** `cargo xtask conform`の
//! 検証の出力の置き場の検査が、ほかのファイルにこの木を指す綴りが現れることを違反として止める。
//!
//! 木の下の場所を導くメソッドをこの型が持つのは、役割を持つパスが自分の配置を知るためである。
//! 呼び出し側が`join`を並べて場所を綴ると、同じ場所の綴り方が入口の数だけ増える。

use std::path::PathBuf;

use super::name::検証の出力の置き場名;

/// 検証の出力とビルドの成果物が載る木の名前。Cargoが`CARGO_TARGET_DIR`の既定として使う綴りと同じである。
const 出力の木の名前: &str = "target";
/// デバッグの成果物をCargoが置く場所。木の名前の直下である。
const デバッグの成果物の場所: &str = "debug";
/// 差分ビルドの中間データをCargoが置く場所。デバッグの成果物の場所の直下である。
const 差分ビルドの中間データの場所: &str = "incremental";
/// codexのレビューが使うビルドの出力先。親のビルドと同時に走っても壊れないよう木の下で分けてある。
const CODEXレビューのビルドの出力先の場所: &str = "codex-review";

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub(crate) struct 検証の出力ルート(&'static str);

impl 検証の出力ルート {
    pub(crate) const fn 既定() -> Self {
        Self(出力の木の名前)
    }

    /// 名前1つが指す置き場。名前がパスへ組み上がる口はここと`置き場の中のファイル`だけである。
    pub(crate) fn 名前が指す置き場(self, 名前: 検証の出力の置き場名) -> PathBuf {
        PathBuf::from(self.0).join(名前.綴りを見せる())
    }

    /// 置き場1つの直下にあるファイル。焼き上がりの実在を確かめる側と、書き出す側が同じ口を通る。
    pub(crate) fn 置き場の中のファイル(self, 名前: 検証の出力の置き場名, ファイル名: &str) -> PathBuf {
        self.名前が指す置き場(名前).join(ファイル名)
    }

    /// 木そのものの名前。掃除の入口が、作業ツリーごとの出力先(この名前に続けて印を付けた兄弟)を探すのに使い、
    /// 規約の機械検査が「この名前を綴ったファイルがほかに無いか」を見るのにも使う。
    pub(crate) fn 木の名前(self) -> &'static str {
        self.0
    }

    /// いまのビルドが実際に使っている出力の木。`CARGO_TARGET_DIR`が据えられていればその値、無ければ既定の木である。
    /// cargoが構築をその置き場へ書くため、構築したものを起こす側と掃除の入口が同じ場所を見るのにこれを読む。
    pub(crate) fn いま使っている木() -> PathBuf {
        std::env::var_os("CARGO_TARGET_DIR").map_or_else(|| PathBuf::from(出力の木の名前), PathBuf::from)
    }

    pub(crate) fn 差分ビルドの中間データの置き場(self) -> PathBuf {
        PathBuf::from(self.0).join(デバッグの成果物の場所).join(差分ビルドの中間データの場所)
    }

    pub(crate) fn codexレビューのビルドの出力先(self) -> PathBuf {
        PathBuf::from(self.0).join(CODEXレビューのビルドの出力先の場所)
    }
}
