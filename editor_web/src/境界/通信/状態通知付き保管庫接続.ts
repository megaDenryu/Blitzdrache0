import type { 大域世界構造, チャンク座標, チャンク構造, マテリアル台帳 } from '../../生成/編集資源契約.ts'
import type { 読込結果, 保存結果 } from './サーバー通信結果.ts'
import type { プロジェクト保管庫接続 } from './プロジェクト保管庫接続.ts'
import { 保存状態サービス } from './保存状態サービス.ts'

// 保管庫通信を包み、大域世界およびチャンクの読込・保存の成否を単一の保存状態サービスへ通知するデコレータ。
export class 状態通知付き保管庫接続 implements プロジェクト保管庫接続 {
    public readonly 通知: 保存状態サービス

    public constructor(
        private readonly _内側保管庫: プロジェクト保管庫接続,
        通知?: 保存状態サービス,
    ) {
        this.通知 = 通知 ?? new 保存状態サービス()
    }

    public async 大域世界の構造を読む(): Promise<読込結果<大域世界構造>> {
        const 結果 = await this._内側保管庫.大域世界の構造を読む()
        if (結果.種別 === '成功') {
            this.通知.大域状態を更新する('読込完了(起動時)', false)
        } else if (結果.種別 === '無し') {
            this.通知.大域状態を更新する('未保存(初期生成)', false)
        } else {
            this.通知.大域状態を更新する(`読込失敗: ${結果.エラー.種別} ${結果.エラー.説明}`, true)
        }
        return 結果
    }

    public async 大域世界の構造を保存する(構造: 大域世界構造): Promise<保存結果> {
        const 結果 = await this._内側保管庫.大域世界の構造を保存する(構造)
        if (結果.種別 === '成功') {
            this.通知.大域状態を更新する('保存完了', false)
        } else {
            this.通知.大域状態を更新する(`保存失敗: ${結果.エラー.種別} ${結果.エラー.説明}`, true)
        }
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
        if (結果.種別 === '成功') {
            this.通知.チャンク状態を更新する(座標, '読込完了(起動時)', false)
        } else if (結果.種別 === '無し') {
            this.通知.チャンク状態を更新する(座標, '未保存(初期生成)', false)
        } else {
            this.通知.チャンク状態を更新する(座標, `読込失敗: ${結果.エラー.種別} ${結果.エラー.説明}`, true)
        }
        return 結果
    }

    public async チャンクの構造を保存する(座標: チャンク座標, 構造: チャンク構造): Promise<保存結果> {
        const 結果 = await this._内側保管庫.チャンクの構造を保存する(座標, 構造)
        if (結果.種別 === '成功') {
            this.通知.チャンク状態を更新する(座標, '保存完了', false)
        } else {
            this.通知.チャンク状態を更新する(座標, `保存失敗: ${結果.エラー.種別} ${結果.エラー.説明}`, true)
        }
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
