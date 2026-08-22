import type { 大域世界構造, チャンク座標, チャンク構造, マテリアル台帳, 建物の格子, 建物の格子の一覧項目, 建物外形カタログ } from '../../生成/編集資源契約.ts'
import type { 建物外形カタログ接続 } from './建物外形カタログ接続.ts'
import type { 建物の格子接続 } from './建物の格子接続.ts'
import type { 建物定義ID } from '../建物定義ID.ts'
import type { 読込結果, 保存結果 } from './サーバー通信結果.ts'
import type { プロジェクト保管庫接続 } from './プロジェクト保管庫接続.ts'
import { 保存状態サービス } from './保存状態サービス.ts'
import { 保存状態を通知する往復 } from './保存状態を通知する往復.ts'
import { 補助の口を解決する, type 補助の口 } from './補助の口の解決.ts'

// 保管庫通信を包み、大域世界およびチャンクの読込・保存の成否を単一の保存状態サービスへ通知するデコレータ。
// 通知の要る往復は`保存状態を通知する往復`が、接続が持たないことのある補助の口の解決は
// `補助の口の解決`が受け持ち、この型は口をひとまとめに見せる窓口に留まる。
export class 状態通知付き保管庫接続 implements プロジェクト保管庫接続, 建物外形カタログ接続, 建物の格子接続 {
    public readonly 通知: 保存状態サービス
    private readonly _補助の口: 補助の口
    private readonly _通知つき往復: 保存状態を通知する往復

    public constructor(
        private readonly _内側保管庫: プロジェクト保管庫接続,
        通知?: 保存状態サービス,
    ) {
        this.通知 = 通知 ?? new 保存状態サービス()
        this._補助の口 = 補助の口を解決する(_内側保管庫)
        this._通知つき往復 = new 保存状態を通知する往復(_内側保管庫, this._補助の口, this.通知)
    }

    public 大域世界の構造を読む(): Promise<読込結果<大域世界構造>> {
        return this._通知つき往復.大域世界の構造を読む()
    }

    public 大域世界の構造を保存する(構造: 大域世界構造): Promise<保存結果> {
        return this._通知つき往復.大域世界の構造を保存する(構造)
    }

    public 大域世界の高さ格子を読む(): Promise<読込結果<ArrayBufferLike>> {
        return this._内側保管庫.大域世界の高さ格子を読む()
    }

    public 大域世界の高さ格子を保存する(バイト列: ArrayBufferLike): Promise<保存結果> {
        return this._内側保管庫.大域世界の高さ格子を保存する(バイト列)
    }

    public チャンクの構造を読む(座標: チャンク座標): Promise<読込結果<チャンク構造>> {
        return this._通知つき往復.チャンクの構造を読む(座標)
    }

    public チャンクの構造を保存する(座標: チャンク座標, 構造: チャンク構造): Promise<保存結果> {
        return this._通知つき往復.チャンクの構造を保存する(座標, 構造)
    }

    public チャンクの高さ格子を読む(座標: チャンク座標): Promise<読込結果<ArrayBufferLike>> {
        return this._内側保管庫.チャンクの高さ格子を読む(座標)
    }

    public チャンクの高さ格子を保存する(座標: チャンク座標, バイト列: ArrayBufferLike): Promise<保存結果> {
        return this._内側保管庫.チャンクの高さ格子を保存する(座標, バイト列)
    }

    public チャンクの材質重みを読む(座標: チャンク座標): Promise<読込結果<ArrayBufferLike>> {
        return this._内側保管庫.チャンクの材質重みを読む(座標)
    }

    public チャンクの材質重みを保存する(座標: チャンク座標, バイト列: ArrayBufferLike): Promise<保存結果> {
        return this._内側保管庫.チャンクの材質重みを保存する(座標, バイト列)
    }

    public マテリアル台帳を読む(): Promise<読込結果<マテリアル台帳>> {
        return this._内側保管庫.マテリアル台帳を読む()
    }

    public マテリアル台帳を保存する(台帳: マテリアル台帳): Promise<保存結果> {
        return this._内側保管庫.マテリアル台帳を保存する(台帳)
    }

    public 建物外形カタログを読む(): Promise<読込結果<建物外形カタログ>> {
        return this._通知つき往復.建物外形カタログを読む()
    }

    public 建物一覧を読む(): Promise<読込結果<建物の格子の一覧項目[]>> {
        return this._補助の口.建物一覧を読む()
    }

    public 建物の格子を読む(建物定義ID: 建物定義ID): Promise<読込結果<建物の格子>> {
        return this._補助の口.建物の格子を読む(建物定義ID)
    }

    // 建物の格子の保存は、通ればサーバー側で建物外形カタログが組み直される。失敗の文面は
    // 建物のタブが自分で出すため、ここでは大域の保存状態へ混ぜない。
    public 建物の格子を保存する(格子: 建物の格子): Promise<保存結果> {
        return this._補助の口.建物の格子を保存する(格子)
    }
}
