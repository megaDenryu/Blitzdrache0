import { カメラ操作適用器, type カメラ操作対象ビュー } from './カメラ操作適用器.ts'
import { ポインタ押下状態 } from './ポインタ押下状態.ts'
import { ポインタ操作の割り当てを決める } from './ポインタ操作割り当て.ts'

// 押し下がっているボタンと三次元ビューを保持し、ポインタの動きを視点の操作へ回す操作サービス。
// 三次元を持つエディターのポインタ配線は、押した・動いた・離したをこの型へ渡すだけでよく、
// 右ドラッグと中ドラッグをどう扱うかを道具ごとに書かない。
// 参照: `_doc/設計/ゲーム開発用エディター基盤.md`「判断13」
export class 視点のポインタ操作係 {
    private readonly _押下 = new ポインタ押下状態()
    private readonly _適用器: カメラ操作適用器

    public constructor(ビュー: カメラ操作対象ビュー) {
        this._適用器 = new カメラ操作適用器(ビュー)
    }

    public 押された(事象: MouseEvent): void {
        this._押下.押された(事象)
    }

    public 離された(事象: MouseEvent): void {
        this._押下.離された(事象)
    }

    public get 左ボタン押下中(): boolean {
        return this._押下.左ボタン押下中
    }

    // 動きを視点の操作として使い切ったならtrueを返す。falseなら呼び出し元がその動きを編集の操作へ回す。
    // 押し下がっているボタンが無いときも移動量を進めるのは、次のドラッグの起点をいまの位置にするためである。
    public 視点の操作として動かしたか(事象: MouseEvent): boolean {
        const 移動量 = this._押下.移動量を取り出す(事象)
        const ボタン = this._押下.押しているボタン
        if (ボタン === null) return false
        return this._適用器.視点の操作として適用したか(ポインタ操作の割り当てを決める(ボタン), 移動量.横, 移動量.縦)
    }
}
