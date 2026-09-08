import type { パターン, 曲の節 } from '../../../生成/編集資源契約.ts'
import { オブジェクトか, 数値か, 文字列か, 配列か } from './オブジェクト判定.ts'
import { コード進行参照の形か } from './コード進行検証.ts'

export function パターンの形か(値: unknown): 値 is パターン {
    if (!オブジェクトか(値)) return false
    return 文字列か(値['名乗り'])
        && 文字列か(値['表示名'])
        && 数値か(値['小節数'])
        && コード進行参照の形か(値['進行の参照'])
        && 配列か(値['格子'])
        && 値['格子'].every(トラック格子の形か)
}

export function トラック格子の形か(値: unknown): boolean {
    if (!オブジェクトか(値) || !配列か(値['行一覧'])) return false
    return 値['行一覧'].every((行) => 配列か(行) && 行.every(数値か))
}

export function 曲の節の形か(値: unknown): 値 is 曲の節 {
    if (!オブジェクトか(値)) return false
    return 文字列か(値['パターンの名乗り']) && 数値か(値['繰り返し回数'])
}
