import type { チャンク座標, チャンク構造 } from '../../生成/編集資源契約.ts'
import type { 読込結果, 保存結果 } from './サーバー通信結果.ts'
import { JSONを取得する, JSONを送信する, バイナリを取得する, バイナリを送信する } from './通信要求.ts'
import { チャンク構造の形か } from './契約検証/チャンク構造検証.ts'
import { JSON文字列からチャンク構造を復元する, チャンク構造をJSON文字列へ直列化する } from './チャンク構造直列化.ts'
import { チャンクパスを組み立てる } from './実サーバー接続の経路.ts'

// チャンクの構造・高さ格子・材質重みの6つの要求。構造だけが長整数を持つため、
// 直列化と復元をこの束が受け持つ(生のJSON.parseでは乱数の種が数値へ落ちる)。
export function 構造を読む(基底URL: string, 座標: チャンク座標): Promise<読込結果<チャンク構造>> {
    return JSONを取得する(
        チャンクパスを組み立てる(基底URL, 座標, '構造'),
        (テキスト) => JSON文字列からチャンク構造を復元する(テキスト),
        チャンク構造の形か,
    )
}

export function 構造を保存する(基底URL: string, 座標: チャンク座標, 構造: チャンク構造): Promise<保存結果> {
    return JSONを送信する(チャンクパスを組み立てる(基底URL, 座標, '構造'), チャンク構造をJSON文字列へ直列化する(構造))
}

export function 高さ格子を読む(基底URL: string, 座標: チャンク座標): Promise<読込結果<ArrayBufferLike>> {
    return バイナリを取得する(チャンクパスを組み立てる(基底URL, 座標, '高さ格子'))
}

export function 高さ格子を保存する(基底URL: string, 座標: チャンク座標, バイト列: ArrayBufferLike): Promise<保存結果> {
    return バイナリを送信する(チャンクパスを組み立てる(基底URL, 座標, '高さ格子'), バイト列)
}

export function 材質重みを読む(基底URL: string, 座標: チャンク座標): Promise<読込結果<ArrayBufferLike>> {
    return バイナリを取得する(チャンクパスを組み立てる(基底URL, 座標, '材質重み'))
}

export function 材質重みを保存する(基底URL: string, 座標: チャンク座標, バイト列: ArrayBufferLike): Promise<保存結果> {
    return バイナリを送信する(チャンクパスを組み立てる(基底URL, 座標, '材質重み'), バイト列)
}
