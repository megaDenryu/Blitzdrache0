import type { 建物外形カタログ } from '../../生成/編集資源契約.ts'
import type { 読込結果 } from './サーバー通信結果.ts'
import { JSONを取得する } from './通信要求.ts'
import { 建物外形カタログの形か } from './契約検証/建物外形カタログ検証.ts'
import { 建物外形カタログパスを組み立てる } from './実サーバー接続の経路.ts'

// 起動時に生成された版付きカタログを読む1つの要求。形式版が食い違うカタログは検証が拒み、
// 編集画面が古い外形で建物を描くことを防ぐ。
export function 建物外形カタログを読む(基底URL: string): Promise<読込結果<建物外形カタログ>> {
    return JSONを取得する(建物外形カタログパスを組み立てる(基底URL), (テキスト) => JSON.parse(テキスト), 建物外形カタログの形か)
}
