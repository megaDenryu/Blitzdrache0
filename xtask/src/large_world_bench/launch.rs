//! 計測条件をblitz_appの起動引数へ写す、実計測と非GPU計画表示の共通境界。

use std::path::Path;

use super::大規模世界の計測指定;

#[cfg(test)]
mod tests;

pub(crate) fn 起動引数を作る(指定: &大規模世界の計測指定, シェーダー入口: &Path) -> Vec<String> {
    let mut 引数 = vec![
        "--scene".to_string(),
        指定.シーン.綴り().to_string(),
        "--benchmark-frames".to_string(),
        指定.フレーム数.to_string(),
    ];
    引数.extend(ストリーミング計測の起動引数を作る(指定, シェーダー入口));
    引数.extend(["--asset-root".to_string(), 指定.アセットルート.display().to_string()]);
    引数
}

pub(crate) fn ストリーミング計測の起動引数を作る(
    指定: &大規模世界の計測指定, シェーダー入口: &Path
) -> Vec<String> {
    let mut 引数 = vec!["--streaming".to_string(), "--streaming-route".to_string()];
    値を足す(&mut 引数, "--streaming-preload-radius", 指定.先読み半径);
    値を足す(&mut 引数, "--streaming-ram-limit", 指定.ram上限);
    値を足す(&mut 引数, "--streaming-vram-limit", 指定.vram上限);
    値を足す(&mut 引数, "--streaming-loader-workers", 指定.ワーカー本数);
    値を足す(&mut 引数, "--streaming-request-capacity", 指定.要求容量);
    値を足す(&mut 引数, "--streaming-completion-capacity", 指定.完了容量);
    値を足す(&mut 引数, "--streaming-route-start-east-meters", 指定.始点東);
    値を足す(&mut 引数, "--streaming-route-start-south-meters", 指定.始点南);
    値を足す(&mut 引数, "--streaming-route-end-east-meters", 指定.終点東);
    値を足す(&mut 引数, "--streaming-route-end-south-meters", 指定.終点南);
    値を足す(&mut 引数, "--streaming-route-meters-per-frame", 指定.一フレーム移動量);
    引数.extend(
        [
            "--report-streaming-summary",
            "--report-memory",
            "--report-draw-issue",
            "--report-instance-sections",
            "--report-gpu-times",
            "--report-frame-times",
            "--no-taa",
        ]
        .map(str::to_string),
    );
    引数.extend(["--shader-source".to_string(), シェーダー入口.display().to_string()]);
    引数
}

fn 値を足す(引数: &mut Vec<String>, 名前: &str, 値: impl ToString) {
    引数.extend([名前.to_string(), 値.to_string()]);
}
