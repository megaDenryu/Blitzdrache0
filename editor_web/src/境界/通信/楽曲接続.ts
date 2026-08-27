import type { 楽曲 } from '../../生成/編集資源契約.ts'
import type { 楽曲ID } from '../楽曲ID.ts'
import type { 読込結果, 保存結果 } from './サーバー通信結果.ts'

// 楽曲エディターが編集する楽曲データを読み書きする境界。
export interface 楽曲接続 {
    楽曲一覧を読む(): Promise<読込結果<楽曲ID[]>>
    楽曲を読む(楽曲ID: 楽曲ID): Promise<読込結果<楽曲>>
    楽曲を保存する(楽曲: 楽曲): Promise<保存結果>
}

export function 楽曲接続か(値: object): 値 is 楽曲接続 {
    return '楽曲を保存する' in 値 && typeof 値.楽曲を保存する === 'function'
}

// 楽曲の口を持たない接続へ包んだときの失敗。
export function 楽曲を提供しない読込失敗<結果>(): 読込結果<結果> {
    return { 種別: '失敗', エラー: { 種別: '未対応', 説明: 'この接続は楽曲を提供しない' } }
}

export function 楽曲を提供しない保存失敗(): 保存結果 {
    return { 種別: '失敗', エラー: { 種別: '未対応', 説明: 'この接続は楽曲を提供しない' } }
}
