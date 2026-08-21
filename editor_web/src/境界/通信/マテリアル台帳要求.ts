import type { マテリアル台帳 } from '../../生成/編集資源契約.ts'
import type { 読込結果, 保存結果 } from './サーバー通信結果.ts'
import { JSONを取得する, JSONを送信する } from './通信要求.ts'
import { マテリアル台帳の形か } from './契約検証/マテリアル台帳検証.ts'

// マテリアル台帳資源のAPI経路の綴りと読み書き手続きをこの1箇所へ集約し、実サーバー接続からは
// 委譲するだけにする(呼び出し連鎖の中の独立した工程の分離。参照: CLAUDE.md「切り出しの根拠義務」第5項)。
const マテリアル台帳パス = '/api/マテリアル台帳'

export async function マテリアル台帳を読む(基底URL: string): Promise<読込結果<マテリアル台帳>> {
    return JSONを取得する(`${基底URL}${マテリアル台帳パス}`, (テキスト) => JSON.parse(テキスト), マテリアル台帳の形か)
}

export async function マテリアル台帳を保存する(基底URL: string, 台帳: マテリアル台帳): Promise<保存結果> {
    return JSONを送信する(`${基底URL}${マテリアル台帳パス}`, JSON.stringify(台帳))
}
