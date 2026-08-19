import type { 読込結果, 保存結果 } from './サーバー通信結果.ts'
import { 読込成功, 読込無し, 読込失敗, 保存成功, 保存失敗 } from './サーバー通信結果.ts'
import { 応答からエラーを読み取る, 通信例外をエラーへ変換する } from './サーバー応答解析.ts'

// バイナリ本文(octet-stream)のREST往復(GET/PUT)を安全に実行する共通手続き。
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
