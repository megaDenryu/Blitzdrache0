import type { 大域世界構造, チャンク座標, チャンク構造, マテリアル台帳 } from '../../生成/編集資源契約.ts'
import type { プロジェクト保管庫接続 } from './プロジェクト保管庫接続.ts'
import type { ソースアセット書き出し接続 } from './ソースアセット書き出し接続.ts'
import type { 読込結果, 保存結果, 書き出し結果 } from './サーバー通信結果.ts'
import {
    JSONを取得する,
    バイナリを取得する,
    JSONを送信する,
    バイナリを送信する,
    書き出しを要求する,
} from './通信要求.ts'
import { 大域世界構造の形か } from './契約検証/大域世界構造検証.ts'
import { チャンク構造の形か } from './契約検証/チャンク構造検証.ts'
import { チャンク構造をJSON文字列へ直列化する, JSON文字列からチャンク構造を復元する } from './チャンク構造直列化.ts'
import { マテリアル台帳を読む as マテリアル台帳を要求する, マテリアル台帳を保存する as マテリアル台帳を要求で保存する } from './マテリアル台帳要求.ts'
import { 大域世界パスを組み立てる, チャンクパスを組み立てる, 書き出しパスを組み立てる } from './実サーバー接続の経路.ts'

// crates/editor_server（127.0.0.1:7901）への実通信を行う境界実装。
// ブラウザではViteプロキシ越しの相対/api経路を用い、Nodeヘッドレスでは基底URLを受け取って接続する。
export class 実サーバー接続 implements プロジェクト保管庫接続, ソースアセット書き出し接続 {
    private readonly _基底URL: string

    public constructor(基底URL: string = '') {
        this._基底URL = 基底URL
    }

    public async 大域世界の構造を読む(): Promise<読込結果<大域世界構造>> {
        return JSONを取得する(
            大域世界パスを組み立てる(this._基底URL, '構造'),
            (テキスト) => JSON.parse(テキスト),
            大域世界構造の形か,
        )
    }

    public async 大域世界の構造を保存する(構造: 大域世界構造): Promise<保存結果> {
        return JSONを送信する(大域世界パスを組み立てる(this._基底URL, '構造'), JSON.stringify(構造))
    }

    public async 大域世界の高さ格子を読む(): Promise<読込結果<ArrayBufferLike>> {
        return バイナリを取得する(大域世界パスを組み立てる(this._基底URL, '高さ格子'))
    }

    public async 大域世界の高さ格子を保存する(バイト列: ArrayBufferLike): Promise<保存結果> {
        return バイナリを送信する(大域世界パスを組み立てる(this._基底URL, '高さ格子'), バイト列)
    }

    public async チャンクの構造を読む(座標: チャンク座標): Promise<読込結果<チャンク構造>> {
        return JSONを取得する(
            チャンクパスを組み立てる(this._基底URL, 座標, '構造'),
            (テキスト) => JSON文字列からチャンク構造を復元する(テキスト),
            チャンク構造の形か,
        )
    }

    public async チャンクの構造を保存する(座標: チャンク座標, 構造: チャンク構造): Promise<保存結果> {
        return JSONを送信する(
            チャンクパスを組み立てる(this._基底URL, 座標, '構造'),
            チャンク構造をJSON文字列へ直列化する(構造),
        )
    }

    public async チャンクの高さ格子を読む(座標: チャンク座標): Promise<読込結果<ArrayBufferLike>> {
        return バイナリを取得する(チャンクパスを組み立てる(this._基底URL, 座標, '高さ格子'))
    }

    public async チャンクの高さ格子を保存する(座標: チャンク座標, バイト列: ArrayBufferLike): Promise<保存結果> {
        return バイナリを送信する(チャンクパスを組み立てる(this._基底URL, 座標, '高さ格子'), バイト列)
    }

    public async チャンクの材質重みを読む(座標: チャンク座標): Promise<読込結果<ArrayBufferLike>> {
        return バイナリを取得する(チャンクパスを組み立てる(this._基底URL, 座標, '材質重み'))
    }

    public async チャンクの材質重みを保存する(座標: チャンク座標, バイト列: ArrayBufferLike): Promise<保存結果> {
        return バイナリを送信する(チャンクパスを組み立てる(this._基底URL, 座標, '材質重み'), バイト列)
    }

    public async ソースアセットへ書き出す(世界名?: string): Promise<書き出し結果> {
        return 書き出しを要求する(書き出しパスを組み立てる(this._基底URL), 世界名)
    }

    public async マテリアル台帳を読む(): Promise<読込結果<マテリアル台帳>> {
        return マテリアル台帳を要求する(this._基底URL)
    }

    public async マテリアル台帳を保存する(台帳: マテリアル台帳): Promise<保存結果> {
        return マテリアル台帳を要求で保存する(this._基底URL, 台帳)
    }
}
