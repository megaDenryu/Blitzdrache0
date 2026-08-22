//! 8km×10km世界をkm級の固定経路で走らせ、OW4と同じ計器を採る入口。

mod arguments;
pub(crate) mod launch;

use std::path::Path;
use std::process::ExitCode;

pub(crate) use arguments::大規模世界の計測指定;

pub(crate) fn 大規模世界の固定経路を計測する(引数一覧: &[String]) -> ExitCode {
    let 指定 = match arguments::引数を読む(引数一覧) {
        Ok(指定) => 指定,
        Err(理由) => {
            eprintln!("[xtask] large-world-benchの引数が不正: {理由}");
            return ExitCode::FAILURE;
        }
    };
    if 指定.計画だけ {
        let シェーダー = crate::shader_copy::シェーダーの入口のファイル::コピー先の中の場所(Path::new("shaders"));
        let 引数 = launch::起動引数を作る(&指定, &シェーダー);
        println!("[xtask] large-world-bench計画: {}", 引数.join(" "));
        return ExitCode::SUCCESS;
    }
    match crate::ow4_bench::大規模世界を測る(&指定) {
        Ok(()) => {
            println!("[xtask] large-world-bench成功: validation 0件、3回の計測完了");
            ExitCode::SUCCESS
        }
        Err(理由) => {
            eprintln!("[xtask] large-world-bench失敗: {理由}");
            ExitCode::FAILURE
        }
    }
}
