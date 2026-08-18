import * as fs from 'node:fs'
import { 実サーバー接続 } from '../境界/通信/index.ts'
import { コマンドJSON文字列を復元する } from './コマンドJSON解析.ts'
import { 編集コマンドを一括適用する } from './編集コマンド一括適用処理.ts'

// Node環境で実行され、引数のコマンドJSONファイルを読み込んで7901番サーバーへ適用・保存する入口。
async function main(): Promise<void> {
    const コマンドファイルパス = process.argv[2]
    if (コマンドファイルパス === undefined || コマンドファイルパス.trim() === '') {
        throw new Error('使用方法: node --experimental-transform-types src/ヘッドレス/編集コマンド一括適用.ts <コマンドJSONパス>')
    }

    const json文字列 = fs.readFileSync(コマンドファイルパス, 'utf8')
    const コマンド列 = コマンドJSON文字列を復元する(json文字列)

    const 保管庫 = new 実サーバー接続('http://127.0.0.1:7901')
    const 適用件数 = await 編集コマンドを一括適用する(保管庫, コマンド列)
    console.log(`編集コマンドを一括適用した: ${適用件数}件`)
}

main().catch((原因: unknown) => {
    const メッセージ = 原因 instanceof Error ? 原因.message : String(原因)
    console.error(`ヘッドレス適用失敗: ${メッセージ}`)
    process.exit(1)
})
