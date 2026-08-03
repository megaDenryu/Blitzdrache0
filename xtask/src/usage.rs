//! `cargo xtask`の人間向けコマンド一覧の表示。担当するのは並べる順序と枠組みだけであり、説明文そのものは
//! `command_catalog`が、どのコマンドをどの実装へ割り当てるかは`dispatch`が持つ。
//! 引数なしで呼ばれたときと未知のコマンドのときに表示する。物量と裁定材料の計測の入口は`measurement`が持つ。

mod command_catalog;
mod measurement;

pub(crate) fn 使い方を表示する() {
    println!("使い方: cargo xtask <コマンド>");
    println!();
    println!("コマンド一覧:");
    for 説明 in command_catalog::コマンドの説明一覧 {
        println!("{説明}");
    }
    measurement::計測の入口を表示する();
    println!(
        "  streaming-bench [フレーム数]  固定経路でチャンクを読み込みながら、予算を十分に取った反復のRAM・VRAM推移と、縮退させた読込・解除順の再現を測る"
    );
}
