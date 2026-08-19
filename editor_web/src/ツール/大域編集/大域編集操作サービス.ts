import type { 編集コマンド } from '../../生成/編集資源契約.ts'
import type { ワールド編集状態 } from '../チャンク編集/編集モデル/index.ts'
import { 編集コマンドを適用する, 差し戻しを適用する } from '../チャンク編集/操作コマンド/index.ts'
import type { 大域編集状態 } from './大域編集状態.ts'
import type { 大域編集同期サービス } from './大域編集同期サービス.ts'

// 大域操作コマンドの適用・取り消しおよび全道路消去等を担当するサービス。
export class 大域編集操作サービス {
    public constructor(
        private readonly _モデル: ワールド編集状態,
        private readonly _UI状態: 大域編集状態,
        private readonly _同期: 大域編集同期サービス,
    ) {}

    public コマンドを実行する(コマンド: 編集コマンド): void {
        const 差し戻し = 編集コマンドを適用する(this._モデル, コマンド)
        this._UI状態.取り消し断片を積む(差し戻し)
        this._同期.全体を同期する()
    }

    public 取り消す(): void {
        const 差し戻し = this._UI状態.取り消し断片を取り出す()
        if (差し戻し !== undefined) {
            差し戻しを適用する(this._モデル, 差し戻し)
            this._同期.全体を同期する()
        }
    }

    public 道路を全消去する(): void {
        while (this._モデル.広域道路.制御点列.length > 0) {
            this.コマンドを実行する({
                種類: '道路点を削除する',
                値: {
                    対象: { 種類: '広域' },
                    添字: 0,
                },
            })
        }
        this._UI状態.選択中ノード添字 = null
        this._同期.道路を同期する()
        this._同期.UIを同期する()
    }
}
