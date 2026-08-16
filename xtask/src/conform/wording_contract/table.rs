//! 契約する綴りの型と、領域ごとの台帳の束ね。担当するのは「どの領域の台帳が検査の対象か」だけである。
//! 検査の手順は親モジュールが持ち、どの綴りがどのファイルに在るべきかは各領域の台帳が持つ。

mod atmosphere_readback;
mod exit_report;
mod house_row_tally;
mod instance_tally;
mod shader_reload;
mod shadow_band_assignment;

/// 契約する綴り1件。出す側と読む側が同じ文字列を持つことを、綴りを台帳が1つ持つことで守る。
pub(super) struct 綴りの契約 {
    pub(super) 綴り: &'static str,
    /// この綴りが文字列リテラルとして現れなければならないファイル。出す側と読む側の両方を並べる。
    pub(super) 現れるファイル一覧: &'static [&'static str],
}

/// 領域ごとの台帳。並びは検査の順にだけ効く。
pub(super) const 領域一覧: [&[綴りの契約]; 6] = [
    &atmosphere_readback::綴り一覧,
    &exit_report::綴り一覧,
    &house_row_tally::綴り一覧,
    &instance_tally::綴り一覧,
    &shader_reload::綴り一覧,
    &shadow_band_assignment::綴り一覧,
];
