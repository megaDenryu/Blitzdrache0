import type { 部品交差情報 } from 'SengenThree'
import type { 大域編集画面部品 } from './画面/index.ts'
import type { 大域編集状態 } from './大域編集状態.ts'
import type { 大域編集操作サービス } from './大域編集操作サービス.ts'
import type { 大域編集同期サービス } from './大域編集同期サービス.ts'

// 広域道路作成・ノード選択のクリック入力を処理する。
export class 大域編集クリックハンドラ {
    public constructor(
        private readonly _状態: 大域編集状態,
        private readonly _部品: 大域編集画面部品,
        private readonly _操作: 大域編集操作サービス,
        private readonly _同期: 大域編集同期サービス,
    ) {}

    public クリック処理(最前面当たり: 部品交差情報 | null, 当たり一覧: readonly 部品交差情報[]): void {
        const ビュー = this._部品.三次元ビュー

        if (this._状態.モード === '広域道路作成' && 最前面当たり && 最前面当たり.部品 === ビュー.地形) {
            const pt = 最前面当たり.交差点
            this._操作.コマンドを実行する({
                種類: '道路点を追加する',
                値: {
                    対象: { 種類: '広域' },
                    位置: { x: pt.x, y: pt.y + 0.5, z: pt.z },
                },
            })
            return
        }

        // 選択モードの主作業も広域道路編集モードと同じ「道路ノードを選択する」を再利用する。
        if (this._状態.モード === '広域道路編集' || this._状態.モード === '選択') {
            this._道路ノードを選択する(当たり一覧)
        }
    }

    private _道路ノードを選択する(当たり一覧: readonly 部品交差情報[]): boolean {
        const ビュー = this._部品.三次元ビュー
        for (const 当たり of 当たり一覧) {
            if (当たり.部品 === ビュー.道路ノード) {
                const ノード一覧 = ビュー.道路ノード.ノードメッシュ一覧
                for (let i = 0; i < ノード一覧.length; i++) {
                    if (ノード一覧[i] === 当たり.原初交差情報.object) {
                        this._状態.選択中ノード添字 = i
                        this._同期.道路を同期する()
                        this._同期.UIを同期する()
                        return true
                    }
                }
            }
        }
        return false
    }
}
