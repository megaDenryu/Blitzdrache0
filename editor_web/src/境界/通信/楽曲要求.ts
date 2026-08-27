import type { 楽曲 } from '../../生成/編集資源契約.ts'
import type { 楽曲ID } from '../楽曲ID.ts'
import { 楽曲IDを生成する } from '../楽曲ID.ts'
import type { 読込結果, 保存結果 } from './サーバー通信結果.ts'
import { JSONを取得する, JSONを送信する } from './通信要求.ts'
import { 楽曲一覧の形か, 楽曲の形か } from './契約検証/楽曲検証.ts'
import { 楽曲一覧パスを組み立てる, 楽曲パスを組み立てる } from './実サーバー接続の経路.ts'

// 楽曲の3つの要求（一覧・読込・保存）を実行する。
export async function 楽曲一覧を読む(基底URL: string): Promise<読込結果<楽曲ID[]>> {
    const 結果 = await JSONを取得する(楽曲一覧パスを組み立てる(基底URL), (テキスト) => JSON.parse(テキスト), 楽曲一覧の形か)
    if (結果.種別 === '失敗') return 結果
    if (結果.種別 === '無し') return 結果
    return { 種別: '成功', 値: 結果.値.map(楽曲IDを生成する) }
}

export function 楽曲を読む(基底URL: string, 楽曲ID: 楽曲ID): Promise<読込結果<楽曲>> {
    return JSONを取得する(楽曲パスを組み立てる(基底URL, 楽曲ID), (テキスト) => JSON.parse(テキスト), 楽曲の形か)
}

export function 楽曲を保存する(基底URL: string, 楽曲: 楽曲): Promise<保存結果> {
    return JSONを送信する(楽曲パスを組み立てる(基底URL, 楽曲.名乗り), JSON.stringify(楽曲))
}
