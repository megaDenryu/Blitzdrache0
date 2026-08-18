import type { 大域世界構造, 世界の区画割り, 広域道路, 位置3次元 } from '../../../生成/編集資源契約.ts'
import { isArray, isNumber, isObject } from './オブジェクト判定.ts'

// 受信した大域世界構造のJSONが型契約に適合しているかを実行時に検査する。
export function 大域世界構造の形か(値: unknown): 値 is 大域世界構造 {
    if (!isObject(値)) return false
    return 世界の区画割りの形か(値['区画割り']) && 広域道路の形か(値['広域道路'])
}

export function 世界の区画割りの形か(値: unknown): 値 is 世界の区画割り {
    if (!isObject(値)) return false
    return (
        isNumber(値['一辺のメートル']) &&
        isNumber(値['軸あたりチャンク数']) &&
        isNumber(値['チャンクあたり格子解像度'])
    )
}

export function 広域道路の形か(値: unknown): 値 is 広域道路 {
    if (!isObject(値)) return false
    if (!isArray(値['制御点列'])) return false
    if (!値['制御点列'].every(位置3次元の形か)) return false
    return isNumber(値['全幅メートル']) && isNumber(値['細分割数'])
}

export function 位置3次元の形か(値: unknown): 値 is 位置3次元 {
    if (!isObject(値)) return false
    return isNumber(値['x']) && isNumber(値['y']) && isNumber(値['z'])
}
