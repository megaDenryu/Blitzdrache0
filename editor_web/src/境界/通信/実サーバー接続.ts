import type { 大域世界構造, チャンク座標, チャンク構造 } from '../../生成/編集資源契約.ts'
import type { プロジェクト保管庫接続 } from './プロジェクト保管庫接続.ts'
import type { 読込結果, 保存結果 } from './サーバー通信結果.ts'
import {
    JSONをGETで取得する,
    バイナリをGETで取得する,
    JSONをPUTで送信する,
    バイナリをPUTで送信する,
} from './HTTP通信共通.ts'
import { 大域世界構造の形か } from './契約検証/大域世界構造検証.ts'
import { チャンク構造の形か } from './契約検証/チャンク構造検証.ts'
import { チャンク構造をJSON文字列へ直列化する, JSON文字列からチャンク構造を復元する } from './チャンク構造直列化.ts'

// crates/editor_server（127.0.0.1:7901）への実通信を行う境界実装。
// ブラウザではViteプロキシ越しの相対/api経路を用い、Nodeヘッドレスでは基底URLを受け取って接続する。
export class 実サーバー接続 implements プロジェクト保管庫接続 {
    private readonly _基底URL: string

    public constructor(基底URL: string = '') {
        this._基底URL = 基底URL
    }

    public async 大域世界の構造を読む(): Promise<読込結果<大域世界構造>> {
        return JSONをGETで取得する(
            `${this._基底URL}/api/大域世界/構造`,
            (テキスト) => JSON.parse(テキスト),
            大域世界構造の形か,
        )
    }

    public async 大域世界の構造を保存する(構造: 大域世界構造): Promise<保存結果> {
        return JSONをPUTで送信する(
            `${this._基底URL}/api/大域世界/構造`,
            JSON.stringify(構造),
        )
    }

    public async 大域世界の高さ格子を読む(): Promise<読込結果<ArrayBufferLike>> {
        return バイナリをGETで取得する(`${this._基底URL}/api/大域世界/高さ格子`)
    }

    public async 大域世界の高さ格子を保存する(バイト列: ArrayBufferLike): Promise<保存結果> {
        return バイナリをPUTで送信する(
            `${this._基底URL}/api/大域世界/高さ格子`,
            バイト列,
        )
    }

    public async チャンクの構造を読む(座標: チャンク座標): Promise<読込結果<チャンク構造>> {
        return JSONをGETで取得する(
            `${this._基底URL}/api/チャンク/${座標.x}/${座標.z}/構造`,
            (テキスト) => JSON文字列からチャンク構造を復元する(テキスト),
            チャンク構造の形か,
        )
    }

    public async チャンクの構造を保存する(座標: チャンク座標, 構造: チャンク構造): Promise<保存結果> {
        return JSONをPUTで送信する(
            `${this._基底URL}/api/チャンク/${座標.x}/${座標.z}/構造`,
            チャンク構造をJSON文字列へ直列化する(構造),
        )
    }

    public async チャンクの高さ格子を読む(座標: チャンク座標): Promise<読込結果<ArrayBufferLike>> {
        return バイナリをGETで取得する(`${this._基底URL}/api/チャンク/${座標.x}/${座標.z}/高さ格子`)
    }

    public async チャンクの高さ格子を保存する(座標: チャンク座標, バイト列: ArrayBufferLike): Promise<保存結果> {
        return バイナリをPUTで送信する(
            `${this._基底URL}/api/チャンク/${座標.x}/${座標.z}/高さ格子`,
            バイト列,
        )
    }

    public async チャンクの材質重みを読む(座標: チャンク座標): Promise<読込結果<ArrayBufferLike>> {
        return バイナリをGETで取得する(`${this._基底URL}/api/チャンク/${座標.x}/${座標.z}/材質重み`)
    }

    public async チャンクの材質重みを保存する(座標: チャンク座標, バイト列: ArrayBufferLike): Promise<保存結果> {
        return バイナリをPUTで送信する(
            `${this._基底URL}/api/チャンク/${座標.x}/${座標.z}/材質重み`,
            バイト列,
        )
    }
}
