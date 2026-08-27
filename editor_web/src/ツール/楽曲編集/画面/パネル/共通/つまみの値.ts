// つまみが持つ綴りを数値として読む。範囲入力の値は文字列でしか取り出せないため、変換はここ1箇所に閉じる。
// 読めない綴りは黙って既定値へ落とさず、配線の誤りとして明示の失敗にする。
export function つまみの綴りを数値として読む(綴り: string): number {
    const 数値 = Number.parseFloat(綴り)
    if (!Number.isFinite(数値)) {
        throw new Error(`つまみの値を数値として読めません: ${綴り}`)
    }
    return 数値
}
