//! 寄せられない綴りの台帳の型と、領域ごとの台帳の束ね。担当するのは「どの台帳が検査の対象か」だけである。
//! どの綴りをどこまで許すかは領域ごとの台帳が決め、判定の手順は`allowance`が持つ。
//! 領域で分けるのは、綴りを足すときに触るのがその領域の台帳1つになるようにするためである。

mod other_files;
mod shader_files;

/// この台帳自身の置き場所。違反の報告先として親が使う。
pub(super) const 台帳のファイル: &str = "xtask/src/conform/duplicate_file_literal/allowance/table.rs";

/// 寄せられない綴り1件。理由を綴りと並べて持つのは、台帳を減らすときに
/// 「何が変われば消せるのか」を読み手が判断できるようにするためである。
pub(super) struct 寄せられない綴り {
    pub(super) 綴り: &'static str,
    /// この綴りが現れてよいファイル。ここに無い場所へ書かれたら許さず、ここから消えたら陳腐化として報告する。
    pub(super) 現れてよい場所一覧: &'static [&'static str],
    pub(super) 寄せられない理由: &'static str,
}

/// 領域ごとの台帳。並びは検査の順にだけ効く。
pub(super) const 領域一覧: [&[寄せられない綴り]; 2] = [&shader_files::一覧, &other_files::一覧];
