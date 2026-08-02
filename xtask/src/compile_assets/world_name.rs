//! プロセス境界で世界を指す綴りの台帳。担当するのは綴りそのものを1箇所に置くことだけであり、
//! どの世界をどの出力ルートへ焼くかは呼び出し側が決める。
//! 綴りはアセットコンパイラ側(`crates/blitz_asset_compiler/examples/compile_assets/world/argument_name.rs`)にも
//! 同じものがあり、食い違えば実行時アセット生成が「未知の世界名である」で失敗する。

pub const 板の世界: &str = "chunk_world";
pub const 地形の世界: &str = "terrain_world";
pub const 植生の世界: &str = "vegetation_world";
pub const 見本の集落の世界: &str = "village_world";

/// 頂点処理量の係数を同定する計測専用の世界。地形の代表世界と同じ地面と配置を持ち、同居植生の原型だけが違う。
/// 粗い側は代表世界と同じ原型を、細かい側は面を格子へ細分化した原型を読む。
/// 参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「シャドウ性能の是正(フェーズ2性能課題、2026-08-03着手)」
pub const 頂点診断の粗い世界: &str = "vertex_diag_coarse_world";
pub const 頂点診断の細かい世界: &str = "vertex_diag_fine_world";
