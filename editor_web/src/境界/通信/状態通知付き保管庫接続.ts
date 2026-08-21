import type { 大域世界構造, チャンク座標, チャンク構造, マテリアル台帳, 建物外形カタログ } from '../../生成/編集資源契約.ts'
import { 建物外形カタログ接続か, 建物外形カタログを提供しない失敗, type 建物外形カタログ接続 } from './建物外形カタログ接続.ts'
import type { 読込結果, 保存結果 } from './サーバー通信結果.ts'
import type { プロジェクト保管庫接続 } from './プロジェクト保管庫接続.ts'
import { 保存状態サービス } from './保存状態サービス.ts'
import { 読込結果から保存状態を作る, 保存結果から保存状態を作る } from './通信結果から保存状態を作る.ts'

// 保管庫通信を包み、大域世界およびチャンクの読込・保存の成否を単一の保存状態サービスへ通知するデコレータ。
export class 状態通知付き保管庫接続 implements プロジェクト保管庫接続, 建物外形カタログ接続 {
    public readonly 通知: 保存状態サービス

    public constructor(
        private readonly _内側保管庫: プロジェクト保管庫接続,
        通知?: 保存状態サービス,
    ) {
        this.通知 = 通知 ?? new 保存状態サービス()
    }

    public async 大域世界の構造を読む(): Promise<読込結果<大域世界構造>> {
        const 結果 = await this._内側保管庫.大域世界の構造を読む()
        const 状態 = 読込結果から保存状態を作る(結果)
        this.通知.大域状態を更新する(状態.文言, 状態.エラーか)
        return 結果
    }

    public async 建物外形カタログを読む(): Promise<読込結果<建物外形カタログ>> {
        const 結果 = 建物外形カタログ接続か(this._内側保管庫)
            ? await this._内側保管庫.建物外形カタログを読む()
            : 建物外形カタログを提供しない失敗()
        if (結果.種別 === '失敗') {
            this.通知.大域状態を更新する(`建物外形カタログ読込失敗: ${結果.エラー.種別} ${結果.エラー.説明}`, true)
        }
        return 結果
    }

    public async 大域世界の構造を保存する(構造: 大域世界構造): Promise<保存結果> {
        const 結果 = await this._内側保管庫.大域世界の構造を保存する(構造)
        const 状態 = 保存結果から保存状態を作る(結果)
        this.通知.大域状態を更新する(状態.文言, 状態.エラーか)
        return 結果
    }

    public 大域世界の高さ格子を読む(): Promise<読込結果<ArrayBufferLike>> {
        return this._内側保管庫.大域世界の高さ格子を読む()
    }

    public 大域世界の高さ格子を保存する(バイト列: ArrayBufferLike): Promise<保存結果> {
        return this._内側保管庫.大域世界の高さ格子を保存する(バイト列)
    }

    public async チャンクの構造を読む(座標: チャンク座標): Promise<読込結果<チャンク構造>> {
        const 結果 = await this._内側保管庫.チャンクの構造を読む(座標)
        const 状態 = 読込結果から保存状態を作る(結果)
        this.通知.チャンク状態を更新する(座標, 状態.文言, 状態.エラーか)
        return 結果
    }

    public async チャンクの構造を保存する(座標: チャンク座標, 構造: チャンク構造): Promise<保存結果> {
        const 結果 = await this._内側保管庫.チャンクの構造を保存する(座標, 構造)
        const 状態 = 保存結果から保存状態を作る(結果)
        this.通知.チャンク状態を更新する(座標, 状態.文言, 状態.エラーか)
        return 結果
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
}
