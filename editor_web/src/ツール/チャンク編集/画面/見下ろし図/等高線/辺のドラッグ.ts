import type { 平面の位置, 等高線 } from '../../../../../生成/編集資源契約.ts'
import type { ワールドXZ } from '../見下ろし図の視点.ts'
import type { ドラッグの結果 } from './頂点のドラッグ.ts'
import { 二点の距離, type 辺の当たり } from './等高線の当たり判定.ts'

// クリックとドラッグを分ける移動量(画素)。粗マスのポインタ操作係の同じ判断に揃える。
const クリックとみなす移動画素の上限 = 4

// 押した位置から今の位置までの差分を、辺の両端の頂点だけへ加えた頂点列を作る純粋関数。
export function 辺の差分を加えた頂点列(頂点列: readonly 平面の位置[], 当たり: 辺の当たり, 差分: 平面の位置): 平面の位置[] {
    return 頂点列.map((頂点, 添字) =>
        添字 === 当たり.頂点の添字甲 || 添字 === 当たり.頂点の添字乙
            ? { x: 頂点.x + 差分.x, z: 頂点.z + 差分.z }
            : { x: 頂点.x, z: 頂点.z },
    )
}

// 既存の等高線の辺を押してから離すまでの写し。下書きの線は離すまで変えず、押した位置からの差分を
// 辺の両端の頂点へ加えて動かし、離したときに1つのコマンドへまとめる。頂点のドラッグの辺版であり、
// 触れるのは動かしている線の写しと当たり、押した位置・最後の位置だけである。
export class 辺のドラッグ {
    private _当たり: 辺の当たり | null = null
    private _元の線: 等高線 | null = null
    private _押した位置: ワールドXZ | null = null
    private _最後の位置: ワールドXZ | null = null
    private _画素毎メートル: number | null = null
    private _写し: 等高線 | null = null

    public 始める(当たり: 辺の当たり, 線: 等高線, 押した位置: ワールドXZ, 画素毎メートル: number): void {
        this._当たり = 当たり
        this._元の線 = 線
        this._押した位置 = 押した位置
        this._最後の位置 = 押した位置
        this._画素毎メートル = 画素毎メートル
        this._写し = { 高さメートル: 線.高さメートル, 頂点列: 線.頂点列.map((p) => ({ x: p.x, z: p.z })), 閉じている: 線.閉じている }
    }

    // 動かしていれば両端の頂点を置き直してtrueを返す。
    public 動かす(位置: ワールドXZ): boolean {
        if (this._当たり === null || this._元の線 === null || this._押した位置 === null) return false
        const 差分 = { x: 位置.x - this._押した位置.x, z: 位置.z - this._押した位置.z }
        this._写し = { ...this._元の線, 頂点列: 辺の差分を加えた頂点列(this._元の線.頂点列, this._当たり, 差分) }
        this._最後の位置 = 位置
        return true
    }

    // 押した位置から最後の位置までの画素での移動が閾値を超えていれば動いたことにする。
    // 閾値以内ならクリックによる選択だけとみなし、呼び出し側は動いたがfalseのときコマンドを積まない。
    public 終える(): ドラッグの結果 | null {
        if (
            this._当たり === null ||
            this._元の線 === null ||
            this._押した位置 === null ||
            this._最後の位置 === null ||
            this._画素毎メートル === null
        ) {
            return null
        }
        const 動いた距離メートル = 二点の距離(this._押した位置, this._最後の位置)
        const 結果: ドラッグの結果 = {
            線の添字: this._当たり.線の添字,
            線: this._写し ?? this._元の線,
            動いた: 動いた距離メートル > クリックとみなす移動画素の上限 / this._画素毎メートル,
        }
        this._当たり = null
        this._元の線 = null
        this._押した位置 = null
        this._最後の位置 = null
        this._画素毎メートル = null
        this._写し = null
        return 結果
    }

    public 写しで置き換える(一覧: readonly 等高線[]): readonly 等高線[] {
        if (this._当たり === null || this._写し === null) return 一覧
        const 写し = this._写し
        return 一覧.map((線, 添字) => (添字 === this._当たり?.線の添字 ? 写し : 線))
    }
}
