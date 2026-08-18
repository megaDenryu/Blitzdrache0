import type { 編集コマンド } from '../生成/編集資源契約.ts'
import { isArray, isObject, isString } from '../境界/通信/契約検証/オブジェクト判定.ts'

const プレースホルダー = '__BIGINT_SEED__'

// JSON文字列から編集コマンド列を読み取り、2^53を超える乱数の種をBigIntとして復元する。
export function コマンドJSON文字列を復元する(json文字列: string): readonly 編集コマンド[] {
    const 退避済みJSON = json文字列.replace(
        /"乱数の種"\s*:\s*(-?\d+)/g,
        `"乱数の種": "${プレースホルダー}:$1"`,
    )
    const パース結果: unknown = JSON.parse(退避済みJSON)
    const 復元結果 = 再帰的にBigIntを復元する(パース結果)
    if (!isArray(復元結果)) {
        throw new Error('コマンドJSONのルートは配列でなければならない')
    }
    const コマンド列: 編集コマンド[] = []
    for (const 項目 of 復元結果) {
        if (!編集コマンドの形か(項目)) {
            throw new Error(`不正な編集コマンド形式: ${JSON.stringify(項目)}`)
        }
        コマンド列.push(項目)
    }
    return コマンド列
}

function 編集コマンドの形か(値: unknown): 値 is 編集コマンド {
    if (!isObject(値)) return false
    return isString(値['種類']) && '値' in 値
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
