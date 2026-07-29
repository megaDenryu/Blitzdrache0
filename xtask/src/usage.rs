//! `cargo xtask`の人間向けコマンド一覧。担当するのは説明文だけであり、どのコマンドをどの実装へ割り当てるかは`main`が持つ。
//! 引数なしで呼ばれたときと未知のコマンドのときに表示する。

pub(crate) fn 使い方を表示する() {
    println!("使い方: cargo xtask <コマンド>");
    println!();
    println!("コマンド一覧:");
    println!("  verify           検証の標準列 (conform -> fmt --check -> check -> clippy -D warnings -> test) を実行する");
    println!("  conform          規約適合の機械検査 (100行制限/禁止文字列/不正allow/依存白リスト/参照パス実在/節参照実在/vulkan配下のDrop実装禁止)");
    println!("  type-metrics     型ごとのフィールド数・impl分散ファイル数・メソッド数を多い順に表示する (違反判定はしない)");
    println!("  smoke            blitz_appを--framesで自動実行し、validation件数0を終了コードで確認する");
    println!(
        "  compile-assets [ソースルート 出力ルート [世界名]]  ソースを検証して実行時形式を生成する(引数なしでchunk_worldをtarget/runtime_assetsへ、terrain_worldをtarget/terrain_assetsへ)"
    );
    println!("  watch-assets     カタログのソース依存を監視し、変更時に実行時形式を再生成する");
    println!(
        "  gen-source-assets 検証用ソースアセット(スモーク用quad・影検証シーン・板の世界25チャンク・地形世界25チャンクの高さ格子)をassets/へ生成する"
    );
    println!("  fetch-assets     標準サンプル(DamagedHelmet・Fox)をassets/samples/へ取得する(curl.exe使用)");
    println!(
        "  gen-sky-dataset <ArHosekSkyModelData_RGB.hのパス>  Hosek-Wilkie解析近似の係数データセットを公開ヘッダからリトルエンディアンf32列へ焼く(出典と書庫のSHA-256はgen_sky_dataset.rsの冒頭)"
    );
    println!(
        "  gen-atmosphere-reference <precomputed_atmospheric_scatteringの作業コピー>  Bruneton 2017実装のCPU参照から大気の物理量の期待値を焼く(出典・手順・要るツールはgen_atmosphere_reference.rsの冒頭。Windows専用)"
    );
    println!("  bench            リリース版の固定シーンを600フレーム実行し、GPU時間とCPU側フレーム間隔分布を表示する");
    println!("  bench-display-timing  benchに実表示間隔の計測を足して実行する(計測が描画ループを止めるため既存の時系列とは比較できない)");
    println!("  m10-bench        M10流体GPU試作を固定条件で実行し、検証件数とGPU時間を表示する");
    println!("  m11-soak         3600フレーム連続実行し、RAM・VRAM推移を約5秒間隔で表示する");
    println!("  object-bench     二対象の画素判定後、1・10・100対象のGPU/CPU時間とGPUメモリを計測する");
    println!("  origin-invariance 世界全体へkm級の大域平行移動を加えても読み戻し画像がバイト一致し、カメラだけを微小に動かすと変わることを確かめる");
    println!("  lod-crack        地形LODの段差0・1・最大を四方向と細粗入替でGPU描画し、内側の継ぎ目に背景色が露出しないことを確かめる");
    println!(
        "  instance-draw    植生インスタンス群を実機描画し、4個体が画面の4分割へ離れて描かれること・両パスの発行が群×段ごと1回であること・個体数を増やしてもGPU確保数が増えないことを確かめる"
    );
    println!(
        "  instance-cull    可視判定のオンとオフで植生シーンを描き、画面内の絵がバイト一致すること・視錐台外の個体が描かれなくなること・画面外の個体が落とす影が残ることを確かめる"
    );
    println!(
        "  instance-lod     段を2つ持つ原型の植生シーンを描き、同じ群で2段が同時に立つこと・段の違いが遠景の画素に出て近景は変わらないこと・ヒステリシス帯の内側の往復で段が振動しないこと・段の切替でGPU確保とディスク読込が動かないことを確かめる"
    );
    println!(
        "  instance-stream  地形と植生が同居する25チャンク世界を本番のストリーミング経路で走らせ、束の追加と解除で植生の状態が生まれて消えること・解除の後に残留がないこと・可視判定の変動がディスクI/OもGPU確保も動かさないこと・植生が地形の上に立つことを確かめる"
    );
    println!(
        "  cloth-empty      群がカメラ視錐台にもライト視錐台にも入らないシーンを布ありと布なしで描き、両パスの発行が0件のフレームでも布のシーン描画と影の記録が成立することを確かめる"
    );
    println!(
        "  cloth-night      空ありのquad世界を夜と正午で布ありと布なしで描き、布のライティングが方向光の強度と環境光係数に追従することを確かめる"
    );
    println!(
        "  cloth-shadow-order  異なる配置変換を持つ描画対象を2つ並べた影検証シーンを布ありで描き、束の中の走査順を入れ替えても布の影の画素が変わらないことを確かめる"
    );
    println!(
        "  csm-seam         低い太陽の地形世界を本番の色と帯の可視化で描き、どの帯にも影があること・境界の両側に影があること・境界の輝度段差が帯の内側の勾配を超えないことを確かめる"
    );
    println!(
        "  sky-state        代表時刻の天空状態を本番の呼び出し元越しに導出し、表を出したうえで決定性・日境界の循環・太陽方向の単位長・全出力の有限値・正午の最大と真夜中の最小を判定する(描画なし)"
    );
    println!(
        "  sky-draw         地形世界を空ありと空なしで描き、空が塗った画素・天頂と地平の色の違い・ジオメトリ画素のバイト一致・空パスのGPU時間を判定して絵をPNGへ書き出す"
    );
    println!(
        "  sky-time         空を持つ地形世界を4時刻で描き、同一時刻のバイト一致・隣り合う時刻の空色と明部と暗部の変化・夜の環境光だけの明るさ・空の非飽和・太陽円盤の高輝度画素を判定して絵をPNGへ書き出す"
    );
    println!(
        "  atmosphere-lut   透過率LUTと多重散乱LUTをウィンドウ無しのGPUで焼いて読み戻し、代表テクセルのCPU正本との一致・全要素の有限性と非負・透過率の値域・同入力2回の完全一致を判定する"
    );
    println!("  ow3-dod          原点移動・LOD継ぎ目・半径2ストリーミングを本番経路でまとめて測り、複数LOD画像をPNGへ書き出す");
    println!(
        "  ow4-bench [チャンクあたり個体数...]  植生の密度だけを変えた25チャンク世界を各3回走らせ、CPU区間・GPU時間・計数・会計・プロセス実測を採る(既定は400・4000・40000体)"
    );
    println!(
        "  streaming-bench [フレーム数]  固定経路でチャンクを読み込みながら、予算を十分に取った反復のRAM・VRAM推移と、縮退させた読込・解除順の再現を測る"
    );
}
