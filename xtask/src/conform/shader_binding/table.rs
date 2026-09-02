//! 突き合わせる束縛番号の組の型と、領域ごとの台帳の束ね。担当するのは「どのセットの台帳が検査の対象か」だけである。
//! 読み取りの規則は隣の工程が持ち、どの宣言とどの宣言が同じ番号でなければならないかは各セットの台帳が持つ。
//! セットで分けるのは、束縛を1本足すときに触るのがそのセットの台帳1つになるようにするためである。

mod cloth_set;
mod geometry_set;
mod lighting_set;
mod material_set;
mod view_pass_set;
mod xpbd_bench_set;

/// 突き合わせる束縛番号の組。正本の行は宣言の前置きで、写しの行は宣言されている変数の名前で見つける。
/// セット番号はシェーダーの属性だけが書き、CPUはレイアウトの並びで表すため、台帳が正本を持つ。
pub(super) struct 束縛番号の組 {
    pub(super) 正本パス: &'static str,
    pub(super) 正本の前置き: &'static str,
    pub(super) 写しパス: &'static str,
    pub(super) 写しの変数名: &'static str,
    pub(super) セット番号: u32,
}

/// セットごとの台帳。並びは検査の順にだけ効く。
pub(super) const 領域ごとの台帳の一覧: [&[束縛番号の組]; 6] = [
    &view_pass_set::束縛番号の組の一覧,
    &geometry_set::束縛番号の組の一覧,
    &material_set::束縛番号の組の一覧,
    &lighting_set::束縛番号の組の一覧,
    &xpbd_bench_set::束縛番号の組の一覧,
    &cloth_set::束縛番号の組の一覧,
];
