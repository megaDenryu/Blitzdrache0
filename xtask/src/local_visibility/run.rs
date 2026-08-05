//! 1つの形ぶんのblitz_app起動。受け取るのは形の語、返すのは標準出力である。
//!
//! 目視見本の庭を使うのは、この世界だけが局所可視性補正を宣言しており、深度プリパスと2本のコンピュートが
//! 本番と同じ順序で積まれるためである。合成深度は深度プリパスの後で深度画像を丸ごと書き換えるため、
//! 世界が描くジオメトリは局所可視度に1つも寄与しない。世界の選択が決めるのは経路であって入力ではない。
//!
//! フレーム数を12に留めるのは、局所可視度が深度だけから決まり、空と間接照明の焼き上げの進み具合に
//! 依存しないためである。

use std::process::Command;

/// 描くフレーム数。最終フレームで注入と読み戻しが2回起こる。
const フレーム数: &str = "12";
/// 正午。時刻は局所可視度に効かないが、実行条件を1つに固定して読み手が条件を推測しなくてよいようにする。
const 一日内秒: &str = "43200";
/// 時計を止める倍率。局所可視度は深度だけから決まるため時刻に依らないが、既定の時計が実時間で進むと
/// 遠方環境の焼き直しがフレームごとに起こりうる。止めるほうが実行の条件が1つ減る。
const 時計を止める倍率: &str = "0";

pub(super) fn 描画する(形: &str) -> Result<String, String> {
    let 出力 = Command::new("cargo")
        .args(["run", "-p", "blitz_app", "--", "--scene", crate::visual_sample_world::シーン名])
        .args(["--asset-root", crate::visual_sample_world::アセットルート])
        .args(["--frames", フレーム数])
        .args(["--time-of-day", 一日内秒, "--time-scale", 時計を止める倍率])
        .args(["--local-visibility-shape", 形])
        .output()
        .map_err(|誤り| format!("blitz_appを起動できなかった({形}): {誤り}"))?;
    let 標準出力 = String::from_utf8_lossy(&出力.stdout).into_owned();
    if !出力.status.success() {
        print!("{標準出力}");
        eprintln!("{}", String::from_utf8_lossy(&出力.stderr));
        return Err(format!("blitz_appが{}で失敗した({形})", 出力.status));
    }
    crate::validation_count::零件数を確かめる(&標準出力, 形)?;
    Ok(標準出力)
}
