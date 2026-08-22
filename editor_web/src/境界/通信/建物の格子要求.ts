import type { 建物の格子, 建物の格子の一覧項目 } from '../../生成/編集資源契約.ts'
import type { 読込結果, 保存結果 } from './サーバー通信結果.ts'
import { JSONを取得する, JSONを送信する } from './通信要求.ts'
import { 建物の格子の一覧の形か, 建物の格子の形か } from './契約検証/建物の格子検証.ts'
import { 建物の格子パスを組み立てる, 建物一覧パスを組み立てる } from './実サーバー接続の経路.ts'

// 建物の格子の3つの要求。実サーバー接続が持つメソッドが薄くなりすぎないよう、
// 経路と検証の組み合わせをここへ寄せる(マテリアル台帳要求.tsと同じ分け方)。
export function 建物一覧を読む(基底URL: string): Promise<読込結果<建物の格子の一覧項目[]>> {
    return JSONを取得する(建物一覧パスを組み立てる(基底URL), (テキスト) => JSON.parse(テキスト), 建物の格子の一覧の形か)
}

export function 建物の格子を読む(基底URL: string, 建物定義ID: string): Promise<読込結果<建物の格子>> {
    return JSONを取得する(建物の格子パスを組み立てる(基底URL, 建物定義ID), (テキスト) => JSON.parse(テキスト), 建物の格子の形か)
}

export function 建物の格子を保存する(基底URL: string, 格子: 建物の格子): Promise<保存結果> {
    return JSONを送信する(建物の格子パスを組み立てる(基底URL, 格子.建物定義ID), JSON.stringify(格子))
}
