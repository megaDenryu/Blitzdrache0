import type {
    チャンク構造,
    チャンクの道路,
    建物の配置,
    建物種別,
    散布の設定,
} from '../../../生成/編集資源契約.ts'
import { isArray, isBigInt, isNumber, isObject, isString } from './オブジェクト判定.ts'
import { 位置3次元の形か } from './大域世界構造検証.ts'

// 受信したチャンク構造のJSONが型契約に適合しているかを実行時に検査する。
export function チャンク構造の形か(値: unknown): 値 is チャンク構造 {
    if (!isObject(値)) return false
    return (
        チャンクの道路の形か(値['道路']) &&
        建物一覧の形か(値['建物一覧']) &&
        散布の設定の形か(値['散布'])
    )
}

export function チャンクの道路の形か(値: unknown): 値 is チャンクの道路 {
    if (!isObject(値)) return false
    if (!isArray(値['制御点列'])) return false
    if (!値['制御点列'].every(位置3次元の形か)) return false
    return (
        isNumber(値['全幅メートル']) &&
        isNumber(値['散布除外バッファメートル']) &&
        isNumber(値['細分割数'])
    )
}

export function 建物一覧の形か(値: unknown): 値 is Array<建物の配置> {
    if (!isArray(値)) return false
    return 値.every(建物の配置の形か)
}

export function 建物種別の形か(値: unknown): 値 is 建物種別 {
    return 値 === '家屋' || 値 === '塔' || 値 === '宝箱'
}

export function 建物の配置の形か(値: unknown): 値 is 建物の配置 {
    if (!isObject(値)) return false
    return (
        isString(値['識別子']) &&
        建物種別の形か(値['種別']) &&
        位置3次元の形か(値['位置']) &&
        isNumber(値['向きラジアン']) &&
        isNumber(値['基礎半径メートル']) &&
        isNumber(値['なじみ半径メートル'])
    )
}

export function 散布の設定の形か(値: unknown): 値 is 散布の設定 {
    if (!isObject(値)) return false
    return isNumber(値['最小間隔メートル']) && isBigInt(値['乱数の種'])
}
