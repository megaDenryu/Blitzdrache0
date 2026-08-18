// 契約検証のための共通型ガード関数群。
// anyやasキャストを一切使わずに値の型を安全に絞り込む。

export function isObject(値: unknown): 値 is Record<string, unknown> {
    return typeof 値 === 'object' && 値 !== null && !Array.isArray(値)
}

export function isString(値: unknown): 値 is string {
    return typeof 値 === 'string'
}

export function isNumber(値: unknown): 値 is number {
    return typeof 値 === 'number' && Number.isFinite(値)
}

export function isBigInt(値: unknown): 値 is bigint {
    return typeof 値 === 'bigint'
}

export function isArray(値: unknown): 値 is readonly unknown[] {
    return Array.isArray(値)
}
