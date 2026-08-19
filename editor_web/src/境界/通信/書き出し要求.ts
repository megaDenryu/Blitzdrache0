import type { 書き出し結果 } from './サーバー通信結果.ts'
import { 書き出し成功, 書き出し失敗 } from './サーバー通信結果.ts'
import { 応答からエラーを読み取る, 通信例外をエラーへ変換する } from './サーバー応答解析.ts'
import { 書き出し応答の形か } from './契約検証/書き出し応答検証.ts'

// ソースアセット書き出しのPOST要求。世界名が省略なら要求本体を空にする(サーバー側が既定の世界名を補う)。
export async function 書き出しを要求する(通信先: string, 世界名?: string): Promise<書き出し結果> {
    try {
        const 応答 = await fetch(通信先, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(世界名 === undefined ? {} : { 出力先の世界名: 世界名 }),
        })
        if (!応答.ok) {
            return 書き出し失敗(await 応答からエラーを読み取る(応答))
        }
        const 本文: unknown = await 応答.json()
        if (!書き出し応答の形か(本文)) {
            return 書き出し失敗({ 種別: '応答形式不正', 説明: `${通信先} の応答が型契約と一致しなかった` })
        }
        return 書き出し成功(本文.書いたファイル数, 本文.出力先)
    } catch (原因) {
        return 書き出し失敗(通信例外をエラーへ変換する(原因))
    }
}
