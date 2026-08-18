import type { サーバーエラー } from './サーバー通信結果.ts'
import { isObject, isString } from './契約検証/オブジェクト判定.ts'

// crates/editor_server の失敗応答は `{ "種別": "...", "説明": "..." }` の形で返される。
export async function 応答からエラーを読み取る(応答: Response): Promise<サーバーエラー> {
    const 本文: unknown = await 応答.json().catch(() => null)
    if (サーバーエラーの形か(本文)) {
        return 本文
    }
    return {
        種別: 'サーバーエラー応答の形式不正',
        説明: `状態コード ${応答.status} の応答を解釈できなかった`,
    }
}

export function 通信例外をエラーへ変換する(原因: unknown): サーバーエラー {
    const 説明 = 原因 instanceof Error ? 原因.message : String(原因)
    return { 種別: '通信失敗', 説明 }
}

function サーバーエラーの形か(値: unknown): 値 is サーバーエラー {
    if (!isObject(値)) {
        return false
    }
    return isString(値['種別']) && isString(値['説明'])
}
