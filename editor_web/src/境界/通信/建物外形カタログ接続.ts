import type { 建物外形カタログ } from '../../生成/編集資源契約.ts'
import type { 読込結果 } from './サーバー通信結果.ts'

// 保存資源とは別に、起動時生成された建物外形カタログを読む境界。
export interface 建物外形カタログ接続 {
    建物外形カタログを読む(): Promise<読込結果<建物外形カタログ>>
}

export function 建物外形カタログ接続か(値: object): 値 is 建物外形カタログ接続 {
    return '建物外形カタログを読む' in 値 && typeof 値.建物外形カタログを読む === 'function'
}
