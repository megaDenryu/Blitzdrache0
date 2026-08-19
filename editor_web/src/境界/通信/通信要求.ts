import type { 読込結果, 保存結果, 書き出し結果 } from './サーバー通信結果.ts'
import { 読込成功, 読込無し, 読込失敗, 保存成功, 保存失敗, 書き出し成功, 書き出し失敗 } from './サーバー通信結果.ts'
import { 応答からエラーを読み取る, 通信例外をエラーへ変換する } from './サーバー応答解析.ts'
import { 書き出し応答の形か } from './契約検証/書き出し応答検証.ts'

// JSONおよびバイナリ（octet-stream）のREST通信を安全に実行する共通手続き群。
export async function JSONを取得する<T>(
    通信先: string,
    解析処理: (本文: string) => unknown,
    型ガード: (値: unknown) => 値 is T,
): Promise<読込結果<T>> {
    try {
        const 応答 = await fetch(通信先)
        if (!応答.ok) {
            return 読込失敗(await 応答からエラーを読み取る(応答))
        }
        const テキスト = await 応答.text()
        if (テキスト.trim() === '' || テキスト.trim() === 'null') {
            return 読込無し()
        }
        const 解析結果 = 解析処理(テキスト)
        if (解析結果 === null) {
            return 読込無し()
        }
        if (!型ガード(解析結果)) {
            return 読込失敗({ 種別: '応答形式不正', 説明: `${通信先} の応答が型契約と一致しなかった` })
        }
        return 読込成功(解析結果)
    } catch (原因) {
        return 読込失敗(通信例外をエラーへ変換する(原因))
    }
}

export async function バイナリを取得する(通信先: string): Promise<読込結果<ArrayBufferLike>> {
    try {
        const 応答 = await fetch(通信先)
        if (応答.status === 204) {
            return 読込無し()
        }
        if (!応答.ok) {
            return 読込失敗(await 応答からエラーを読み取る(応答))
        }
        const バッファ = await 応答.arrayBuffer()
        return 読込成功(バッファ)
    } catch (原因) {
        return 読込失敗(通信例外をエラーへ変換する(原因))
    }
}

export async function JSONを送信する(通信先: string, JSON文字列: string): Promise<保存結果> {
    try {
        const 応答 = await fetch(通信先, {
            method: 'PUT',
            headers: { 'Content-Type': 'application/json' },
            body: JSON文字列,
        })
        if (!応答.ok) {
            return 保存失敗(await 応答からエラーを読み取る(応答))
        }
        return 保存成功()
    } catch (原因) {
        return 保存失敗(通信例外をエラーへ変換する(原因))
    }
}

export async function バイナリを送信する(通信先: string, バイト列: ArrayBufferLike): Promise<保存結果> {
    try {
        const 応答 = await fetch(通信先, {
            method: 'PUT',
            headers: { 'Content-Type': 'application/octet-stream' },
            body: バイト列 instanceof ArrayBuffer ? バイト列 : new Uint8Array(バイト列).slice().buffer,
        })
        if (!応答.ok) {
            return 保存失敗(await 応答からエラーを読み取る(応答))
        }
        return 保存成功()
    } catch (原因) {
        return 保存失敗(通信例外をエラーへ変換する(原因))
    }
}

// 世界名が省略なら要求本体を空にする(サーバー側が既定の世界名を補う)。
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
