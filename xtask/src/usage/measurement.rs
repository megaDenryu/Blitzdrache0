//! 物量と裁定材料の計測の入口の一覧。担当するのは説明文だけである。
//! 一覧を分けて持つのは、計測の入口が条件と軸の説明を伴って長くなり、検収の入口の並びの見通しを潰すためである。

pub(super) fn 計測の入口を表示する() {
    println!(
        "  ow4-bench [チャンクあたり個体数...] [--production-draw] [--time-of-day 秒] [シャドウ計測指定]  植生の密度だけを変えた25チャンク世界を各3回走らせ、CPU区間・GPU時間・計数・会計・プロセス実測を採る(既定は400・4000・40000体)。シャドウ計測指定は--shadow-resolution・--caster-margin・--max-shadow-distance・--shadow-caster-range・--camera-yaw・--camera-nudge・--no-instance-shadow・--no-shadow-casters・--no-instance-lod・--report-caster-distanceであり、そのままblitz_appへ渡る"
    );
    println!(
        "  shadow-probe <resolution|casters|margin|camera|vertex|sun|distance|range> [--rounds N] [--chunk-instances N] [--time-of-day 秒]  律速切り分けの計測バッチ。1つの軸の条件を子プロセスとして交互に起動し、距離区分別GPU時間の中央値とp95・投入インデックス数・可視数・その実行の太陽高度と方位を条件ごとにまとめて出す(判定はしない。周回数は条件数の倍数へ切り上げる。生値と実行ログは軸ごとにtarget/shadow_probe/<軸名>/へ残す。vertexは外形と配置を固定して原型のトポロジー量だけを変えた診断世界の対、sunは南中対称の時刻3対6条件で、均衡を保つ最小は既定の6周回36実行である。distanceは最大影距離を既定の225と裁定前の300・200・175メートルへ振る4条件、rangeは影の視距離を切断なし・225・200・175メートルへ振る4条件であり、どちらの水準もキャスター距離分布から導いた)"
    );
    println!(
        "  shadow-loss [--layout terrain|range] <--max-shadow-distance メートル | --shadow-caster-range メートル> [--final-color]  同じ構図を基準(現行設定)と候補で1回ずつ描く。候補の軸はちょうど1つだけ受ける(0個・2個・同じ軸2回は失敗し、rangeは影の視距離だけを受ける)。既定はシーンの画素段の診断出力(多段影の評価直後の影可視度と固定25メートル刻みの受光距離帯)から受光距離帯ごとの影の欠落と余分と影可視度の弱まり・強まりを数えて差分画像を書き出す(どこまでの欠落を許すかは判定しない。ただしrangeは遠方キャスターと近距離対照だけを置いた負の対照の世界であり、遠方キャスターの影が落ちる領域と近距離対照の影が落ちる領域を別々に数えて前者が消え後者が残ることを判定する)。--final-colorは診断を付けず空とポスト処理も外さない本番の見た目で2枚をPNGへ撮るだけの様式であり、比較も判定もしない(オーナーが見た目で選ぶための絵である)"
    );
}
