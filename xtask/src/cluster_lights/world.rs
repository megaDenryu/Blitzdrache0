//! 第4段階の検収が使う2つの世界の起動材料。担当するのは、世界ごとのシーン名とアセットルートと共通の起動引数を
//! 1箇所へ置くことと、実行時形式を作って実在を確かめることである。
//!
//! 共通の起動引数を世界と一緒に持つのは、光の件数を振った実行どうしが同じ条件で比べられなければならないためである。
//! 条件の指定が実行ごとに散ると、件数の違いと条件の違いが値の上で混ざる。
//!
//! 時間再構成と自動露出を外すのは、どちらもフレームをまたいで値を持ち越すためである。持ち越しがあると、
//! 同一起動の中で撮った2枚が一致することを読み戻しの決定性の根拠にできない。

pub(super) const 夜のシーン: &str = "terrain_night_lights";
pub(super) const 夜のアセットルート: &str = "target/night_lights_assets";
const 夜の実行時形式: &str = "target/night_lights_assets/terrain_night_lights.blitzasset";

pub(super) const 屋内のシーン: &str = "prop_stone_hut_interior";
pub(super) const 屋内のアセットルート: &str = "target/stone_hut_assets";
const 屋内の実行時形式: &str = "target/stone_hut_assets/prop_stone_hut_interior.blitzasset";

/// 夜の一日内秒。`cargo xtask cloth-night`と`cargo xtask sky-time`が使う代表時刻と同じ値である。
pub(super) const 夜の一日内秒: &str = "75600";

/// 絵を撮る実行の枚数。静止した世界であり、間接照明の焼き上げが済むだけの枚数があれば足りる。
/// 読み戻しの決定性も屋内の明暗の対照もGPU時間を読まないため、窓が満ちるまで回す必要が無い。
pub(super) const 絵の枚数: &str = "12";

/// GPU時間を採る実行の枚数。パス別GPU時間の窓は直近60標本であり(正本は`crates/blitz_render/src/gpu_pass_timing.rs`の`窓の標本数`)、
/// 読み取りは1フレームに1回、進行中フレーム数ぶん遅れて始まる。90枚なら窓は必ず満ち、窓の先頭は30枚目より後になるため、
/// 間接照明と大気の焼き上げが済んだ定常状態だけが窓に入る。窓が満ちたことは入口が標本数で必ず確かめる。
pub(super) const 計測の枚数: &str = "90";

pub(super) fn 夜の世界を用意する() -> Result<(), String> {
    if !crate::gen_source_assets::生成する() || !crate::compile_assets::夜の多光源世界を既定で生成する() {
        return Err("夜の多光源の検収世界のアセット生成に失敗した".to_string());
    }
    実在を確かめる(夜の実行時形式, "夜の多光源")
}

pub(super) fn 屋内の世界を用意する() -> Result<(), String> {
    if !crate::gen_source_assets::生成する() || !crate::compile_assets::屋内の多光源世界を既定で生成する() {
        return Err("屋内の多光源の検収世界のアセット生成に失敗した".to_string());
    }
    実在を確かめる(屋内の実行時形式, "屋内の多光源")
}

/// 外部のアセットリポジトリが無い環境では、小屋の原型が解決できずコンパイルが失敗する。失敗を見落として
/// 起動へ進むと、カタログ未登録という遠い場所の失敗に化けるため、ここで名指しして落とす。
fn 実在を確かめる(パス: &str, 世界名: &str) -> Result<(), String> {
    if std::path::Path::new(パス).is_file() {
        return Ok(());
    }
    Err(format!(
        "{世界名}の実行時形式{パス}が作られていない(外部のアセットリポジトリが見つからなかった可能性が高い。compile-assetsの出力に理由が出る)"
    ))
}
