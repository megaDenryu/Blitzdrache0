const 入力要素タグ名一覧 = new Set<string>(['INPUT', 'TEXTAREA'])

// フォーカスがinput/textarea要素にあるかを調べる。WASDQEのカメラ移動とAltのモード循環は
// 文字入力中に誤発火してはならないため、キー処理の入口で必ずこの判定を経由する。
export function 入力欄がフォーカス中か(): boolean {
    const 対象 = document.activeElement
    return 対象 !== null && 入力要素タグ名一覧.has(対象.tagName)
}
