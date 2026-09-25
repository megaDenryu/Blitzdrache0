// つまみが持つ文字列を数値として読む。範囲入力の値は文字列でしか取り出せないため、変換はここ1箇所に閉じる。
// 読めない文字列は黙って既定値へ落とさず、配線の誤りとして明示の失敗にする。
export function つまみの文字列を数値として読む(文字列: string): number {
    const 数値 = Number.parseFloat(文字列)
    if (!Number.isFinite(数値)) {
        throw new Error(`つまみの値を数値として読めません: ${文字列}`)
    }
    return 数値
}
