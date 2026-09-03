import type { 等高線 } from '../../../../../生成/編集資源契約.ts'
import type { ワールドXZ, 見下ろし図の視点 } from '../見下ろし図の視点.ts'
import type { 見下ろし図の編集状態 } from '../見下ろし図の編集状態.ts'
import { 当たりの半径画素 } from '../重ね描きの配色.ts'
import { 描いている途中の等高線 } from './描いている途中の等高線.ts'
import { 始点で閉じるか, 線分の当たりを探す, 頂点の当たりを探す } from './等高線の当たり判定.ts'
import { 頂点のドラッグ } from './頂点のドラッグ.ts'
import type { 等高線の重ね描きの入力 } from './等高線の重ね描き.ts'

// 等高線の操作が相手にするもの。下書きの読み出しとコマンドの積み方は配線が決める。
export interface I等高線の操作の相手 {
    readonly 等高線一覧を読む: () => readonly 等高線[]
    readonly 視点を読む: () => 見下ろし図の視点 | null
    readonly 等高線を追加する: (線: 等高線) => void
    readonly 等高線を変更する: (添字: number, 線: 等高線) => void
    readonly 等高線を削除する: (添字: number) => void
    readonly 再描画する: () => void
}

// 等高線モードの左ボタンとDeleteキー。左クリックで頂点を置き、ダブルクリックで線を終え、始点をクリックして閉じる。
// 描いていないときは、頂点の近くを押すとドラッグで動かし、線分の近くをクリックすると選ぶ。
// 1本描き終える・1回動かし終えるまでが1コマンドである(設計正本の判断2、操作契約)。
export class 等高線のポインタ操作係 {
    public readonly 途中の線: 描いている途中の等高線 = new 描いている途中の等高線()
    private readonly _ドラッグ: 頂点のドラッグ = new 頂点のドラッグ()

    public constructor(
        private readonly _状態: 見下ろし図の編集状態,
        private readonly _相手: I等高線の操作の相手,
    ) {}

    // ダブルクリックの2回目の押下は、1回目が置いた頂点の上で起きる。頂点を足さずにそこで線を終える。
    public 押された(位置: ワールドXZ, 原初事象: PointerEvent): void {
        const 半径 = this._当たりの半径メートル()
        if (半径 === null) return
        if (原初事象.detail >= 2) {
            this._確定する(false)
            return
        }
        if (this.途中の線.描いているか) {
            if (始点で閉じるか(this.途中の線.頂点列, 位置, 半径)) this._確定する(true)
            else this.途中の線.頂点を足す(位置)
            this._相手.再描画する()
            return
        }
        const 一覧 = this._相手.等高線一覧を読む()
        const 頂点 = 頂点の当たりを探す(一覧, 位置, 半径)
        if (頂点 !== null) {
            const 線 = 一覧[頂点.線の添字]
            if (線 !== undefined) this._ドラッグ.始める(頂点, 線)
            this._状態.選択中の等高線の添字 = 頂点.線の添字
            this._相手.再描画する()
            return
        }
        const 線の添字 = 線分の当たりを探す(一覧, 位置, 半径)
        if (線の添字 !== null) {
            this._状態.選択中の等高線の添字 = 線の添字
        } else {
            this._状態.選択中の等高線の添字 = null
            this.途中の線.頂点を足す(位置)
        }
        this._相手.再描画する()
    }

    public 動かされた(位置: ワールドXZ, 押下中: boolean): void {
        if (!押下中 || !this._ドラッグ.動かす(位置)) return
        this._相手.再描画する()
    }

    public 離された(): void {
        const 結果 = this._ドラッグ.終える()
        if (結果 === null) return
        if (結果.動いた) this._相手.等高線を変更する(結果.線の添字, 結果.線)
        this._相手.再描画する()
    }

    public 選択中の線を削除する(): void {
        const 添字 = this._状態.選択中の等高線の添字
        if (添字 === null) return
        this._状態.選択中の等高線の添字 = null
        this._相手.等高線を削除する(添字)
    }

    // ドラッグ中の線はまだ下書きに無いため、下書きの一覧のその線だけを動かしている写しへ置き換えて描く。
    public 重ね描きの入力を作る(): 等高線の重ね描きの入力 {
        return {
            等高線一覧: this._ドラッグ.写しで置き換える(this._相手.等高線一覧を読む()),
            選択中の添字: this._状態.選択中の等高線の添字,
            描いている途中の頂点列: this.途中の線.頂点列,
        }
    }

    private _確定する(閉じている: boolean): void {
        const 線 = this.途中の線.確定する(this._状態.新しい等高線の高さメートル, 閉じている)
        if (線 !== null) this._相手.等高線を追加する(線)
    }

    private _当たりの半径メートル(): number | null {
        const 視点 = this._相手.視点を読む()
        return 視点 === null ? null : 当たりの半径画素 / 視点.画素毎メートル
    }
}
