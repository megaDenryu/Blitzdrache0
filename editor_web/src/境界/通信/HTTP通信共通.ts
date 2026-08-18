import type { 読込結果, 保存結果 } from './サーバー通信結果.ts'
import { 読込成功, 読込無し, 読込失敗, 保存成功, 保存失敗 } from './サーバー通信結果.ts'
import { 応答からエラーを読み取る, 通信例外をエラーへ変換する } from './サーバー応答解析.ts'

// JSONおよびバイナリ（octet-stream）のREST通信を安全に実行する共通手続き群。
export async function JSONをGETで取得する<T>(
    URL: string,
    パース処理: (本文: string) => unknown,
    型ガード: (値: unknown) => 値 is T,
): Promise<読込結果<T>> {
    try {
        const 応答 = await fetch(URL)
        if (!応答.ok) {
            return 読込失敗(await 応答からエラーを読み取る(応答))
        }
        const テキスト = await 応答.text()
        if (テキスト.trim() === '' || テキスト.trim() === 'null') {
            return 読込無し()
        }
        const パース結果 = パース処理(テキスト)
        if (パース結果 === null) {
            return 読込無し()
        }
        if (!型ガード(パース結果)) {
            return 読込失敗({ 種別: '応答形式不正', 説明: `${URL} の応答が型契約と一致しなかった` })
        }
        return 読込成功(パース結果)
    } catch (原因) {
        return 読込失敗(通信例外をエラーへ変換する(原因))
    }
}

export async function バイナリをGETで取得する(URL: string): Promise<読込結果<ArrayBufferLike>> {
    try {
        const 応答 = await fetch(URL)
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

export async function JSONをPUTで送信する(URL: string, json文字列: string): Promise<保存結果> {
    try {
        const 応答 = await fetch(URL, {
            method: 'PUT',
            headers: { 'Content-Type': 'application/json' },
            body: json文字列,
        })
        if (!応答.ok) {
            return 保存失敗(await 応答からエラーを読み取る(応答))
        }
        return 保存成功()
    } catch (原因) {
        return 保存失敗(通信例外をエラーへ変換する(原因))
    }
}

export async function バイナリをPUTで送信する(URL: string, バイト列: ArrayBufferLike): Promise<保存結果> {
    try {
        const 応答 = await fetch(URL, {
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
