import type { チャンク構造 } from '../../生成/編集資源契約.ts'
import { isArray, isObject } from './契約検証/オブジェクト判定.ts'

const プレースホルダー = '__BIGINT_SEED__'

// チャンク構造の乱数の種（u64相当のBigInt）を53ビット制限で丸めずにJSON数値リテラルとして出力する。
export function チャンク構造をJSON文字列へ直列化する(構造: チャンク構造): string {
    const json文字列 = JSON.stringify(構造, (_キー: string, 値: unknown) => {
        if (typeof 値 === 'bigint') {
            return `${プレースホルダー}:${値.toString()}`
        }
        return 値
    })
    return json文字列.replace(new RegExp(`"${プレースホルダー}:(-?\\d+)"`, 'g'), '$1')
}

// 2^53を超える数値リテラルがJSON.parseで精度落ちする前に文字列へ退避し、BigIntとして復元する。
export function JSON文字列からチャンク構造を復元する(json文字列: string): unknown {
    const 退避済みJSON = json文字列.replace(
        /"乱数の種"\s*:\s*(-?\d+)/g,
        `"乱数の種": "${プレースホルダー}:$1"`,
    )
    const パース結果: unknown = JSON.parse(退避済みJSON)
    return 再帰的にBigIntを復元する(パース結果)
}

function 再帰的にBigIntを復元する(値: unknown): unknown {
    if (typeof 値 === 'string' && 値.startsWith(`${プレースホルダー}:`)) {
        const 数値部 = 値.slice(`${プレースホルダー}:`.length)
        return BigInt(数値部)
    }
    if (isArray(値)) {
        return 値.map(再帰的にBigIntを復元する)
    }
    if (isObject(値)) {
        const 結果: Record<string, unknown> = {}
        for (const [キー, 項目] of Object.entries(値)) {
            if (キー === '乱数の種' && typeof 項目 === 'number') {
                結果[キー] = BigInt(項目)
            } else {
                結果[キー] = 再帰的にBigIntを復元する(項目)
            }
        }
        return 結果
    }
    return 値
}
