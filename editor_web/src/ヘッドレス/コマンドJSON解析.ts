import type { 編集コマンド } from '../生成/編集資源契約.ts'
import { 配列か, オブジェクトか, 文字列か, 長整数か } from '../境界/通信/契約検証/オブジェクト判定.ts'

// 制約: Rust側契約で現在u64型を持つのは「散布の設定.乱数の種」のみである。
// 正規表現置換はフィールド名「"乱数の種"」固定としている。
// 将来u64フィールドが追加された場合は、本正規表現の更新と契約検証側の対応が必要となる。

function 一意トークンを生成する(): string {
    return `__BIGINT_SEED_${Math.random().toString(36).slice(2)}_${Date.now()}__`
}

// JSON文字列から編集コマンド列を読み取り、2^53を超える乱数の種をBigIntとして復元する。
export function コマンドJSON文字列を復元する(json文字列: string): readonly 編集コマンド[] {
    const トークン = 一意トークンを生成する()
    const 退避済みJSON = json文字列.replace(
        /"乱数の種"\s*:\s*(-?\d+)/g,
        `"乱数の種": "${トークン}:$1"`,
    )
    const 解析結果: unknown = JSON.parse(退避済みJSON)
    const 復元結果 = トークン付きBigIntを復元する(解析結果, トークン)
    if (!配列か(復元結果)) {
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
    if (!オブジェクトか(値)) return false
    return 文字列か(値['種類']) && '値' in 値
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

