import type { 部品交差情報 } from 'SengenThree'
import type { ワールド編集状態 } from './編集モデル/index.ts'
import type { チャンク編集画面部品 } from './画面/index.ts'
import type { チャンク編集状態 } from './チャンク編集状態.ts'
import type { チャンク編集操作サービス } from './チャンク編集操作サービス.ts'
import type { チャンク編集同期サービス } from './チャンク編集同期サービス.ts'

// クリック時の道路点追加・道路ノード選択・建物選択を処理する。
export class チャンク編集クリックハンドラ {
    public constructor(
        private readonly _モデル: ワールド編集状態,
        private readonly _状態: チャンク編集状態,
        private readonly _部品: チャンク編集画面部品,
        private readonly _操作: チャンク編集操作サービス,
        private readonly _同期: チャンク編集同期サービス,
    ) {}

    public クリック処理(最前面当たり: 部品交差情報 | null, 当たり一覧: readonly 部品交差情報[]): void {
        const ビュー = this._部品.三次元ビュー

        if (this._状態.モード === '道作成' && 最前面当たり && 最前面当たり.部品 === ビュー.地形) {
            const pt = 最前面当たり.交差点
            this._操作.コマンドを実行する({
                種類: '道路点を追加する',
                値: {
                    対象: { 種類: 'チャンク', 値: { ...this._状態.対象チャンク座標 } },
                    位置: { x: pt.x, y: pt.y, z: pt.z },
                },
            })
        } else if (this._状態.モード === '道編集') {
            for (const 当たり of 当たり一覧) {
                if (当たり.部品 === ビュー.道路ノード) {
                    const ノード一覧 = ビュー.道路ノード.ノードメッシュ一覧
                    for (let i = 0; i < ノード一覧.length; i++) {
                        if (ノード一覧[i] === 当たり.原初交差情報.object) {
                            this._状態.選択中ノード添字 = i
                            this._同期.道路を同期する()
                            this._同期.UIを同期する()
                            return
                        }
                    }
                }
            }
        } else if (this._状態.モード === '建物') {
            const チャンク = this._モデル.チャンクを取得する(this._状態.対象チャンク座標)
            for (const 建物 of チャンク.建物.一覧を取得する()) {
                const グループ = ビュー.建物.識別子からグループを取得する(建物.識別子)
                if (グループ) {
                    const 一致 = 当たり一覧.some((hit) => {
                        let 親 = hit.原初交差情報.object.parent
                        while (親 !== null) {
                            if (親 === グループ) return true
                            親 = 親.parent
                        }
                        return false
                    })
                    if (一致) {
                        this._状態.選択中建物識別子 = 建物.識別子
                        this._同期.UIを同期する()
                        return
                    }
                }
            }
        }
    }
}
