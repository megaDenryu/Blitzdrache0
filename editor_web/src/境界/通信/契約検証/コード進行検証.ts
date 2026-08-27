import type { 和音, コード進行, コード進行参照, 和音の種類 } from '../../../生成/編集資源契約.ts'
import { オブジェクトか, 数値か, 文字列か, 配列か } from './オブジェクト判定.ts'

export function 和音の種類か(値: unknown): 値 is 和音の種類 {
    return 値 === '長三和音'
        || 値 === '短三和音'
        || 値 === '長七の和音'
        || 値 === '短七の和音'
        || 値 === '属七の和音'
        || 値 === '四度掛留の和音'
        || 値 === '減三和音'
        || 値 === '増三和音'
}

export function 和音の形か(値: unknown): 値 is 和音 {
    if (!オブジェクトか(値)) return false
    return 数値か(値['根音']) && 和音の種類か(値['種類']) && 数値か(値['続くステップ数'])
}

export function コード進行の形か(値: unknown): 値 is コード進行 {
    if (!オブジェクトか(値)) return false
    return 文字列か(値['名前']) && 配列か(値['和音一覧']) && 値['和音一覧'].every(和音の形か)
}

export function コード進行参照の形か(値: unknown): 値 is コード進行参照 {
    if (!オブジェクトか(値)) return false
    if (値['種類'] === '既定の進行') {
        return 文字列か(値['識別子'])
    }
    if (値['種類'] === '独自の進行') {
        return 文字列か(値['名前'])
    }
    return false
}
