import type { ワールド編集状態 } from './編集モデル/index.ts'
import type { チャンク編集画面部品 } from './画面/index.ts'
import type { チャンク編集状態 } from './チャンク編集状態.ts'
import type { チャンク編集同期サービス } from './チャンク編集同期サービス.ts'
import type { 当たりの記録 } from './道路点編集の相手.ts'

// 建物をクリックで選ぶ操作サービス。当たった物体の親をたどって、どの建物のグループに属するかを見る。
// ポインタの振り分けからは、この口(選ぶ)だけを求める。
export interface 建物を選ぶ相手 {
    選ぶ(当たり一覧: readonly 当たりの記録[]): boolean
}

export class 建物のクリック選択 implements 建物を選ぶ相手 {
    public constructor(
        private readonly _モデル: ワールド編集状態,
        private readonly _状態: チャンク編集状態,
        private readonly _部品: チャンク編集画面部品,
        private readonly _同期: チャンク編集同期サービス,
    ) {}

    public 選ぶ(当たり一覧: readonly 当たりの記録[]): boolean {
        const ビュー = this._部品.三次元ビュー
        const チャンク = this._モデル.チャンクを取得する(this._状態.対象チャンク座標)
        for (const 建物 of チャンク.建物.一覧を取得する()) {
            const グループ = ビュー.建物.識別子からグループを取得する(建物.識別子)
            if (グループ === undefined || グループ === null) continue
            const 一致 = 当たり一覧.some((当たり) => {
                let 親 = 当たり.原初交差情報.object.parent
                while (親 !== null) {
                    if (親 === グループ) return true
                    親 = 親.parent
                }
                return false
            })
            if (一致) {
                this._状態.選択中建物識別子 = 建物.識別子
                this._同期.UIを同期する()
                return true
            }
        }
        return false
    }
}
