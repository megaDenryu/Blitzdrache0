//! チャンク1つぶんの大域高さ関数の材料。`blitz_asset_compiler::高さ格子を切り出す`へ渡す
//! `Fn(i16, i16) -> f32`の実体であり、優先順位(そのチャンクの高さ編集が保管庫に保存済みなら
//! それを使い、無ければマザーハイトマップを参照する)をここへ閉じる。世界の外側へはみ出す
//! 添字(1px重複の縁が世界の端に掛かる場合)はマザーハイトマップの縁の値へ丸めて拡張する。

pub(crate) struct 高さ関数材料<'a> {
    pub(crate) チャンクx: i32,
    pub(crate) チャンクz: i32,
    pub(crate) 解像度: u16,
    pub(crate) マザー一辺頂点数: u32,
    pub(crate) マザーバイト列: &'a [u8],
    pub(crate) チャンク別バイト列: Option<&'a [u8]>,
}

impl 高さ関数材料<'_> {
    pub(crate) fn 高さを求める(&self, 添字x: i16, 添字z: i16) -> f32 {
        let ローカルx = ローカル添字を求める(添字x, self.チャンクx, self.解像度);
        let ローカルz = ローカル添字を求める(添字z, self.チャンクz, self.解像度);
        if let (Some(チャンク別), Some(行), Some(列)) = (
            self.チャンク別バイト列,
            コア範囲内(ローカルz, self.解像度),
            コア範囲内(ローカルx, self.解像度),
        ) {
            let 一辺頂点数 = u32::from(self.解像度).saturating_add(1);
            return 格子から高さを読む(チャンク別, 一辺頂点数, 行, 列);
        }
        let 大域x = 添字を大域列へ丸める(添字x, self.マザー一辺頂点数);
        let 大域z = 添字を大域列へ丸める(添字z, self.マザー一辺頂点数);
        格子から高さを読む(self.マザーバイト列, self.マザー一辺頂点数, 大域z, 大域x)
    }
}

fn ローカル添字を求める(添字: i16, チャンク成分: i32, 解像度: u16) -> i32 {
    let 基点 = チャンク成分.saturating_mul(i32::from(解像度));
    i32::from(添字).saturating_sub(基点)
}

fn コア範囲内(ローカル: i32, 解像度: u16) -> Option<u32> {
    let 上限 = i32::from(解像度);
    if ローカル < 0 || ローカル > 上限 {
        return None;
    }
    u32::try_from(ローカル).ok()
}

/// 大域列を`[0, 上限頂点数-1]`へ丸める(縁クランプ)。世界の端のチャンクは重なりの縁が
/// マザーハイトマップの外へ出るため、外へ出た分は最も近い縁の値をそのまま繰り返す。
fn 添字を大域列へ丸める(添字: i16, 上限頂点数: u32) -> u32 {
    if 上限頂点数 == 0 {
        return 0;
    }
    let 上限添字 = i32::try_from(上限頂点数.saturating_sub(1)).unwrap_or(i32::MAX);
    let 丸めた = i32::from(添字).clamp(0, 上限添字);
    u32::try_from(丸めた).unwrap_or(0)
}

fn 格子から高さを読む(バイト列: &[u8], 一辺頂点数: u32, 行: u32, 列: u32) -> f32 {
    let 位置 = u64::from(行).saturating_mul(u64::from(一辺頂点数)).saturating_add(u64::from(列));
    let バイト開始 = usize::try_from(位置.saturating_mul(4)).unwrap_or(usize::MAX);
    バイト列
        .get(バイト開始..バイト開始.saturating_add(4))
        .and_then(|範囲| <[u8; 4]>::try_from(範囲).ok())
        .map(f32::from_le_bytes)
        .unwrap_or(0.0)
}
