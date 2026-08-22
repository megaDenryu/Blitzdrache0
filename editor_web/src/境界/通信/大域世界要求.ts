import type { 大域世界構造 } from '../../生成/編集資源契約.ts'
import type { 読込結果, 保存結果 } from './サーバー通信結果.ts'
import { JSONを取得する, JSONを送信する, バイナリを取得する, バイナリを送信する } from './通信要求.ts'
import { 大域世界構造の形か } from './契約検証/大域世界構造検証.ts'
import { 大域世界パスを組み立てる } from './実サーバー接続の経路.ts'

// 大域世界の構造と高さ格子の4つの要求。経路と検証の組み合わせをここへ寄せ、
// 実サーバー接続は資源ごとの要求を束ねる窓口に留める(マテリアル台帳要求.tsと同じ分け方)。
export function 構造を読む(基底URL: string): Promise<読込結果<大域世界構造>> {
    return JSONを取得する(大域世界パスを組み立てる(基底URL, '構造'), (テキスト) => JSON.parse(テキスト), 大域世界構造の形か)
}

export function 構造を保存する(基底URL: string, 構造: 大域世界構造): Promise<保存結果> {
    return JSONを送信する(大域世界パスを組み立てる(基底URL, '構造'), JSON.stringify(構造))
}

export function 高さ格子を読む(基底URL: string): Promise<読込結果<ArrayBufferLike>> {
    return バイナリを取得する(大域世界パスを組み立てる(基底URL, '高さ格子'))
}

export function 高さ格子を保存する(基底URL: string, バイト列: ArrayBufferLike): Promise<保存結果> {
    return バイナリを送信する(大域世界パスを組み立てる(基底URL, '高さ格子'), バイト列)
}
