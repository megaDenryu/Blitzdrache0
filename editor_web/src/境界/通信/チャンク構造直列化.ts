import type { チャンク構造 } from '../../生成/編集資源契約.ts'
import { 配列か, オブジェクトか, 長整数か } from './契約検証/オブジェクト判定.ts'

// 制約: Rust側契約で現在u64型を持つのは「散布の設定.乱数の種」のみである。
// 正規表現置換はフィールド名「"乱数の種"」固定としている。
// 将来u64フィールドが追加された場合は、本正規表現の更新と契約検証側の対応が必要となる。

function 一意トークンを生成する(): string {
    return `__BIGINT_SEED_${Math.random().toString(36).slice(2)}_${Date.now()}__`
}

// チャンク構造の乱数の種（u64相当のBigInt）を53ビット制限で丸めずにJSON数値リテラルとして出力する。
export function チャンク構造をJSON文字列へ直列化する(構造: チャンク構造): string {
    const トークン = 一意トークンを生成する()
    const json文字列 = JSON.stringify(構造, (キー: string, 値: unknown) => {
        if (typeof 値 === 'bigint') {
            if (キー !== '乱数の種') {
                throw new Error(`予期しないbigintフィールド「${キー}」が直列化に渡された`)
            }
            return `${トークン}:${値.toString()}`
        }
        return 値
    })
    return json文字列.replace(new RegExp(`"${トークン}:(-?\\d+)"`, 'g'), '$1')
}

// 2^53を超える数値リテラルがJSON.parseで精度落ちする前に文字列へ退避し、BigIntとして復元する。
export function JSON文字列からチャンク構造を復元する(json文字列: string): unknown {
    const トークン = 一意トークンを生成する()
    const 退避済みJSON = json文字列.replace(
        /"乱数の種"\s*:\s*(-?\d+)/g,
        `"乱数の種": "${トークン}:$1"`,
    )
    const 解析結果: unknown = JSON.parse(退避済みJSON)
    return トークン付きBigIntを復元する(解析結果, トークン)
}

function トークン付きBigIntを復元する(値: unknown, トークン: string): unknown {
    if (配列か(値)) {
        return 値.map((要素) => トークン付きBigIntを復元する(要素, トークン))
    }
    if (オブジェクトか(値)) {
        const 結果: Record<string, unknown> = {}
        for (const [キー, 項目] of Object.entries(値)) {
            if (キー === '乱数の種') {
                if (typeof 項目 === 'string' && 項目.startsWith(`${トークン}:`)) {
                    結果[キー] = BigInt(項目.slice(`${トークン}:`.length))
                } else if (typeof 項目 === 'number') {
                    結果[キー] = BigInt(項目)
                } else if (長整数か(項目)) {
                    結果[キー] = 項目
                } else {
                    結果[キー] = 項目
                }
            } else {
                結果[キー] = トークン付きBigIntを復元する(項目, トークン)
            }
        }
        return 結果
    }
    return 値
}

