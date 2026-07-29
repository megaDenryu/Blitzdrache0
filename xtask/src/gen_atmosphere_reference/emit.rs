//! 焼いた本文へ出典の見出しを付けてファイルへ書き出す工程。
//! 見出しをここで付けるのは、生成物だけを見た人が出どころと再生成の手順へ辿り着けるようにするためである。
//! 見出しへ焼くリビジョンは固定値でなく実測値である。固定値を書くと、照合を外したときに表示だけが正しく残る。

use std::path::Path;

use super::revision::実測リビジョン;

const 出力パス: &str = "crates/blitz_render/data/bruneton_reference.txt";

const 各行の説明: &str = "\
# 各行は種別と数値の並びである。数値は17桁で書き、f64として厳密に読み戻せる。
# param <名前> <値>                                       焼いた条件の大気パラメータ
# density <高度> <レイリー> <ミー> <吸収>                  規格化密度
# horizon <半径> <天頂余弦>                                地平線の向き
# top_distance <半径> <天頂余弦> <距離>                    大気上端までの距離
# bottom_distance <半径> <天頂余弦> <距離>                 地表までの距離
# optical_length <半径> <天頂余弦> <レイリー> <ミー> <吸収> 上端までの光学距離
# transmittance <半径> <天頂余弦> <赤> <緑> <青>           上端までの透過率
# phase <散乱角余弦> <レイリー> <ミー>                     位相関数の値
";

pub(super) fn 書き出す(本文: &str, リビジョン: &実測リビジョン) -> Result<String, String> {
    let 行数 = 本文.lines().filter(|行| !行.trim().is_empty()).count();
    if 行数 == 0 {
        return Err("焼き出しが1行も出さなかった".to_string());
    }
    let 全体 = format!("{}{本文}", 見出し(リビジョン));
    let パス = Path::new(出力パス);
    if let Some(親) = パス.parent() {
        std::fs::create_dir_all(親).map_err(|誤り| format!("{}を作れない: {誤り}", 親.display()))?;
    }
    std::fs::write(パス, 全体).map_err(|誤り| format!("{出力パス}へ書けない: {誤り}"))?;
    Ok(format!("{出力パス}へ{行数}件のレコードを書いた"))
}

fn 見出し(リビジョン: &実測リビジョン) -> String {
    format!(
        "# Bruneton 2017実装のCPU参照が出した大気の物理量の期待値。手で編集しない。\n\
         # 再生成: cargo xtask gen-atmosphere-reference <precomputed_atmospheric_scatteringの作業コピー>\n\
         # 出典・ライセンス・構築手順: xtask/src/gen_atmosphere_reference.rs の冒頭\n\
         # 焼いた作業コピーのリビジョン: precomputed_atmospheric_scattering {}\n\
         # 焼いた作業コピーのリビジョン: external/dimensional_types {}\n\
         #\n\
         {各行の説明}",
        リビジョン.参照実装, リビジョン.部分モジュール
    )
}
