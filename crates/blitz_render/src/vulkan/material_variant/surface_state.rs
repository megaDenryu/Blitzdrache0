//! 材質の3軸のうち、固定機能の描画状態の軸。担当するのは、アルファの扱い・面の向き・深度書込を1つの値として持ち、
//! これらを材質特徴ビットへ紛れ込ませないことである。
//!
//! 特徴ビットと分けるのは、特徴ビットが「同じパイプラインの中でデータによって有無が変わるもの」であるのに対し、
//! この3つはパイプラインそのものを変えるためである。
//! 参照: `_doc/設計/マルチマテリアルと材質境界.md`「材質の3軸」

/// アルファをどう扱うか。不透明は深度順に依らず描け、マスクは画素段で切り捨て、ブレンドは描画順に依存する。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum アルファ様式 {
    不透明,
    マスク,
    ブレンド,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum 面の向き {
    片面,
    両面,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum 深度書込 {
    書く,
    書かない,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct 表面描画状態 {
    アルファ様式: アルファ様式,
    面の向き: 面の向き,
    深度書込: 深度書込,
}

impl 表面描画状態 {
    pub(crate) const fn 生成する(アルファ様式: アルファ様式, 面の向き: 面の向き, 深度書込: 深度書込) -> Self {
        Self {
            アルファ様式,
            面の向き,
            深度書込,
        }
    }

    /// 現行の入力境界が通す唯一の状態。glTFのアルファ様式と両面表示の宣言は実行時形式へ運ばれず、
    /// glb契約検査器が作者へ警告して落とすため、読み込んだ材質はすべてこの状態になる
    /// (参照: `_doc/設計/マルチマテリアルと材質境界.md`「Blender段4までの受理範囲」)。
    pub(crate) const fn 不透明片面() -> Self {
        Self::生成する(アルファ様式::不透明, 面の向き::片面, 深度書込::書く)
    }

    /// 失敗の表示で使う、3つの軸の値の名前。
    pub(crate) const fn 名前の組(self) -> (&'static str, &'static str, &'static str) {
        let アルファ = match self.アルファ様式 {
            アルファ様式::不透明 => "不透明",
            アルファ様式::マスク => "マスク",
            アルファ様式::ブレンド => "ブレンド",
        };
        let 面 = match self.面の向き {
            面の向き::片面 => "片面",
            面の向き::両面 => "両面",
        };
        let 深度 = match self.深度書込 {
            深度書込::書く => "書く",
            深度書込::書かない => "書かない",
        };
        (アルファ, 面, 深度)
    }
}
