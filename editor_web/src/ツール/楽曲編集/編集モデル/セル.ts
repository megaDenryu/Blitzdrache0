// 格子のセルを表す判別共用体。
// 0=無し, 1=進行に従う打点, 2=進行に従う継続, 3=進行の外の打点, 4=進行の外の継続。

export type セル =
    | { readonly 種類: '打点なし' }
    | { readonly 種類: '音の始まり', readonly 進行に従うか: boolean }
    | { readonly 種類: '音の継続', readonly 進行に従うか: boolean }

// 判別共用体のセルを契約の数値へ変換する。
export function セルを数値へ変換する(セル: セル): number {
    switch (セル.種類) {
        case '打点なし':
            return 0
        case '音の始まり':
            return セル.進行に従うか ? 1 : 3
        case '音の継続':
            return セル.進行に従うか ? 2 : 4
    }
}

// 契約の数値を判別共用体のセルへ変換する。範囲外の値は明示の失敗にする。
export function 数値からセルへ変換する(値: number): セル {
    switch (値) {
        case 0:
            return { 種類: '打点なし' }
        case 1:
            return { 種類: '音の始まり', 進行に従うか: true }
        case 2:
            return { 種類: '音の継続', 進行に従うか: true }
        case 3:
            return { 種類: '音の始まり', 進行に従うか: false }
        case 4:
            return { 種類: '音の継続', 進行に従うか: false }
    }
    throw new Error(`不正なセルの数値です: ${値}`)
}
