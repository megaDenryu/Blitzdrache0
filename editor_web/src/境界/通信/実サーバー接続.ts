import type {
    大域世界構造,
    チャンク座標,
    チャンク構造,
    マテリアル台帳,
    建物の格子,
    建物の格子の一覧項目,
    建物外形カタログ,
} from '../../生成/編集資源契約.ts'
import type { 建物外形カタログ接続 } from './建物外形カタログ接続.ts'
import type { 建物の格子接続 } from './建物の格子接続.ts'
import type { プロジェクト保管庫接続 } from './プロジェクト保管庫接続.ts'
import type { ソースアセット書き出し接続 } from './ソースアセット書き出し接続.ts'
import type { 読込結果, 保存結果, 書き出し結果 } from './サーバー通信結果.ts'
import { 書き出しを要求する } from './通信要求.ts'
import * as 建物外形カタログ要求 from './建物外形カタログ要求.ts'
import * as 大域世界要求 from './大域世界要求.ts'
import * as チャンク資源要求 from './チャンク資源要求.ts'
import * as マテリアル台帳要求 from './マテリアル台帳要求.ts'
import * as 建物の格子要求 from './建物の格子要求.ts'
import { 書き出しパスを組み立てる } from './実サーバー接続の経路.ts'

// crates/editor_server（127.0.0.1:7901）への実通信を行う境界実装。
// ブラウザではViteプロキシ越しの相対/api経路を用い、Nodeヘッドレスでは基底URLを受け取って接続する。
// 資源ごとの経路と検証の組み合わせは`*要求.ts`の各束が持ち、この型は基底URLを保持して束ねるだけである。
export class 実サーバー接続 implements プロジェクト保管庫接続, ソースアセット書き出し接続, 建物外形カタログ接続, 建物の格子接続 {
    private readonly _基底URL: string

    public constructor(基底URL: string = '') {
        this._基底URL = 基底URL
    }

    public 大域世界の構造を読む(): Promise<読込結果<大域世界構造>> {
        return 大域世界要求.構造を読む(this._基底URL)
    }

    public 大域世界の構造を保存する(構造: 大域世界構造): Promise<保存結果> {
        return 大域世界要求.構造を保存する(this._基底URL, 構造)
    }

    public 大域世界の高さ格子を読む(): Promise<読込結果<ArrayBufferLike>> {
        return 大域世界要求.高さ格子を読む(this._基底URL)
    }

    public 大域世界の高さ格子を保存する(バイト列: ArrayBufferLike): Promise<保存結果> {
        return 大域世界要求.高さ格子を保存する(this._基底URL, バイト列)
    }

    public チャンクの構造を読む(座標: チャンク座標): Promise<読込結果<チャンク構造>> {
        return チャンク資源要求.構造を読む(this._基底URL, 座標)
    }

    public チャンクの構造を保存する(座標: チャンク座標, 構造: チャンク構造): Promise<保存結果> {
        return チャンク資源要求.構造を保存する(this._基底URL, 座標, 構造)
    }

    public チャンクの高さ格子を読む(座標: チャンク座標): Promise<読込結果<ArrayBufferLike>> {
        return チャンク資源要求.高さ格子を読む(this._基底URL, 座標)
    }

    public チャンクの高さ格子を保存する(座標: チャンク座標, バイト列: ArrayBufferLike): Promise<保存結果> {
        return チャンク資源要求.高さ格子を保存する(this._基底URL, 座標, バイト列)
    }

    public チャンクの材質重みを読む(座標: チャンク座標): Promise<読込結果<ArrayBufferLike>> {
        return チャンク資源要求.材質重みを読む(this._基底URL, 座標)
    }

    public チャンクの材質重みを保存する(座標: チャンク座標, バイト列: ArrayBufferLike): Promise<保存結果> {
        return チャンク資源要求.材質重みを保存する(this._基底URL, 座標, バイト列)
    }

    public マテリアル台帳を読む(): Promise<読込結果<マテリアル台帳>> {
        return マテリアル台帳要求.マテリアル台帳を読む(this._基底URL)
    }

    public マテリアル台帳を保存する(台帳: マテリアル台帳): Promise<保存結果> {
        return マテリアル台帳要求.マテリアル台帳を保存する(this._基底URL, 台帳)
    }

    public 建物一覧を読む(): Promise<読込結果<建物の格子の一覧項目[]>> {
        return 建物の格子要求.建物一覧を読む(this._基底URL)
    }

    public 建物の格子を読む(建物定義ID: string): Promise<読込結果<建物の格子>> {
        return 建物の格子要求.建物の格子を読む(this._基底URL, 建物定義ID)
    }

    public 建物の格子を保存する(格子: 建物の格子): Promise<保存結果> {
        return 建物の格子要求.建物の格子を保存する(this._基底URL, 格子)
    }

    public 建物外形カタログを読む(): Promise<読込結果<建物外形カタログ>> {
        return 建物外形カタログ要求.建物外形カタログを読む(this._基底URL)
    }

    public ソースアセットへ書き出す(世界名?: string): Promise<書き出し結果> {
        return 書き出しを要求する(書き出しパスを組み立てる(this._基底URL), 世界名)
    }
}
