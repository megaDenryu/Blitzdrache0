//! 局所可視度を一定値へ固定した1条件ぶんのblitz_app起動。受け取るのは出力先と符号値、返すのは
//! 圧縮前のHDR画像である。
//!
//! 間接照明の検収世界を使うのは、金属と誘電体の板が同じ画面に並ぶ唯一の世界だからである。金属の板は拡散の
//! 取り分が0であるため拡散間接照度を持たず、局所可視度を掛けても1ビットも動かない。同じ画面に動く板と動かない板が
//! 揃っていることが、掛かる先が拡散間接だけであることの反証になる。
//!
//! シーンパスのクリア色を黒へ落とすのは、シーン画素段を1度も通らない背景を「局所可視度に不変な画素」として
//! 数えないためである。`--auto-exposure-probe`はクリア色を線形RGBへ置き換える起動指定であり、この実行では
//! 自動露出の集計を1つも読まない。
//!
//! 圧縮前のHDRを読むのは、明るさの圧縮とsRGB符号化を通った後の8ビット値から通す前の値を復元できないためである。
//! 項ごとの比は線形の放射輝度でしか成り立たない。
//!
//! **ゲーム時計を止める。** 既定の時計は実時間で進むため、止めないと12フレームの間に太陽が動き、遠方環境を
//! 焼き直す鍵が実行ごとに違う値で止まる。3枚は局所可視度だけが違う対でなければならず、間接照明の焼き上がりが
//! 1枚だけずれると、その差が局所可視度の効きとして数えられる。2026-08-05に3枚のうち1枚だけが別の焼き上がりに
//! なる実行が実際に出た(板の領域171091画素が別の実行の同条件と食い違った)。

use std::path::Path;
use std::process::Command;

use super::hdr_image::{self, HDR画像};

const シーン名: &str = "indirect_probe";
const アセットルート: &str = "target/runtime_assets";
/// 描くフレーム数。間接照明の焼き上げが定常へ入る本数であり、`indirect-probe`の各条件と揃えてある。
const フレーム数: &str = "12";
/// 昼。太陽が高く、板が直接光と鏡面間接の両方を受ける時刻である。夜で測ると直接光が0になり、
/// 直接光が不変であることを1つも見られない。
const 昼の一日内秒: &str = "43200";
/// 時計を止める倍率。3枚が同じ太陽の位置と同じ遠方環境の焼き上がりを共有するために要る。
const 時計を止める倍率: &str = "0";

pub(super) fn 描画する(出力先: &Path, 符号値: u8) -> Result<HDR画像, String> {
    let ダンプ先 = 出力先.join(format!("ao{符号値}"));
    let 出力 = Command::new("cargo")
        .args(["run", "-p", "blitz_app", "--", "--scene", シーン名])
        .args(["--asset-root", アセットルート])
        .args(["--frames", フレーム数])
        .args(["--time-of-day", 昼の一日内秒, "--time-scale", 時計を止める倍率])
        .args(["--no-sky", "--auto-exposure-probe", "0,0,0"])
        .args(["--local-visibility-fixed", &符号値.to_string()])
        .arg("--dump-hdr-frame")
        .arg(&ダンプ先)
        .output()
        .map_err(|誤り| format!("blitz_appを起動できなかった(局所可視度{符号値}): {誤り}"))?;
    let 標準出力 = String::from_utf8_lossy(&出力.stdout).into_owned();
    if !出力.status.success() {
        print!("{標準出力}");
        eprintln!("{}", String::from_utf8_lossy(&出力.stderr));
        return Err(format!("blitz_appが{}で失敗した(局所可視度{符号値})", 出力.status));
    }
    crate::validation_count::零件数を確かめる(&標準出力, &format!("局所可視度{符号値}"))?;
    hdr_image::読み込む(&ダンプ先)
}
