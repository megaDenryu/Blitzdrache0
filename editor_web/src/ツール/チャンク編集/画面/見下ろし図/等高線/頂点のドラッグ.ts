import type { 等高線 } from '../../../../../生成/編集資源契約.ts'
import type { ワールドXZ } from '../見下ろし図の視点.ts'
import type { 頂点の当たり } from './等高線の当たり判定.ts'

export interface ドラッグの結果 {
    readonly 線の添字: number
    readonly 線: 等高線
    readonly 動いた: boolean
}

// 既存の等高線の頂点を押してから離すまでの写し。下書きの線は離すまで変えず、離したときに1つのコマンドへまとめる。
// 触れるのは動かしている線の写しとその添字だけである。
export class 頂点のドラッグ {
    private _当たり: 頂点の当たり | null = null
    private _写し: 等高線 | null = null
    private _動いた: boolean = false

    public 始める(当たり: 頂点の当たり, 線: 等高線): void {
        this._当たり = 当たり
        this._写し = { 高さメートル: 線.高さメートル, 頂点列: 線.頂点列.map((p) => ({ x: p.x, z: p.z })), 閉じている: 線.閉じている }
        this._動いた = false
    }

    // 動かしていれば頂点を置き直してtrueを返す。
    public 動かす(位置: ワールドXZ): boolean {
        if (this._当たり === null || this._写し === null) return false
        this._写し.頂点列[this._当たり.頂点の添字] = { x: 位置.x, z: 位置.z }
        this._動いた = true
        return true
    }

    public 終える(): ドラッグの結果 | null {
        if (this._当たり === null || this._写し === null) return null
        const 結果: ドラッグの結果 = { 線の添字: this._当たり.線の添字, 線: this._写し, 動いた: this._動いた }
        this._当たり = null
        this._写し = null
        this._動いた = false
        return 結果
    }

    public 写しで置き換える(一覧: readonly 等高線[]): readonly 等高線[] {
        if (this._当たり === null || this._写し === null) return 一覧
        const 写し = this._写し
        return 一覧.map((線, 添字) => (添字 === this._当たり?.線の添字 ? 写し : 線))
    }
}
