// 契約検証のための共通型ガード関数群。
// anyやasキャストを一切使わずに値の型を安全に絞り込む。

export function オブジェクトか(値: unknown): 値 is Record<string, unknown> {
    return typeof 値 === 'object' && 値 !== null && !Array.isArray(値)
}

export function 文字列か(値: unknown): 値 is string {
    return typeof 値 === 'string'
}

export function 数値か(値: unknown): 値 is number {
    return typeof 値 === 'number' && Number.isFinite(値)
}

export function 長整数か(値: unknown): 値 is bigint {
    return typeof 値 === 'bigint'
}

export function 配列か(値: unknown): 値 is readonly unknown[] {
    return Array.isArray(値)
}

