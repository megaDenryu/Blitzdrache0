import type { 材質の筆致, チャンクの道路 } from '../../../生成/編集資源契約.ts'
import type { 高さ場 } from './高さ場.ts'
import { 材質の筆致を塗る処理 } from './材質の筆致の塗り.ts'
import { 急勾配を岩肌へベイクする処理 } from './急勾配の岩肌ベイク.ts'
import { 低地を泥へベイクする処理 } from './低地の泥ベイク.ts'
import { 道路下の泥の帯を焼き直す処理 } from './道路下の泥ベイク.ts'
import { 道路の泥の被覆の記録, type 道路の泥の被覆の記録の写し } from './道路の泥の被覆の記録.ts'

// 4層の地表材質（草・泥・岩・砂）の重みをu8の各画素合計255で管理する。
export class 地表材質 {
    public readonly 解像度: number
    public readonly 一辺のメートル: number
    public readonly 格子間隔: number
    public readonly 材質データ: Uint8Array
    private readonly _道路の泥の被覆の記録: 道路の泥の被覆の記録

    public constructor(解像度: number, 一辺のメートル: number, 初期データ?: Uint8Array) {
        if (解像度 < 2) throw new Error(`解像度は2以上でなければならない: ${解像度}`)
        if (一辺のメートル <= 0) throw new Error(`一辺のメートルは正の数でなければならない: ${一辺のメートル}`)
        this.解像度 = 解像度
        this.一辺のメートル = 一辺のメートル
        this.格子間隔 = 一辺のメートル / (解像度 - 1)
        const 必要バイト数 = 解像度 * 解像度 * 4
        if (初期データ !== undefined) {
            if (初期データ.length !== 必要バイト数) {
                throw new Error(`初期データの要素数が不正: 期待=${必要バイト数}, 実際=${初期データ.length}`)
            }
            this.材質データ = new Uint8Array(初期データ)
        } else {
            this.材質データ = new Uint8Array(必要バイト数)
            for (let i = 0; i < 解像度 * 解像度; i++) {
                this.材質データ[i * 4 + 0] = 255
            }
        }
        this._道路の泥の被覆の記録 = new 道路の泥の被覆の記録(解像度 * 解像度)
    }

    public 複製する(): 地表材質 {
        const 複製 = new 地表材質(this.解像度, this.一辺のメートル, this.材質データ)
        複製.道路の泥の被覆の記録を写しから戻す(this.道路の泥の被覆の記録の写しを取る())
        return 複製
    }

    // 材質データ全体を差し戻し用の別データへ置き換える。要素数の不一致は例外にする。
    public 材質データを置き換える(データ: Uint8Array): void {
        if (データ.length !== this.材質データ.length) {
            throw new Error(`置き換える材質データの要素数が不正: 期待=${this.材質データ.length}, 実際=${データ.length}`)
        }
        this.材質データ.set(データ)
    }

    // 材質筆致を通過点列に沿って適用する。
    public 材質筆致を適用する(筆致: 材質の筆致): void {
        材質の筆致を塗る処理(this.材質データ, this.解像度, this.一辺のメートル, this.格子間隔, 筆致)
    }

    public 急勾配を岩肌へベイクする(対象高さ場: 高さ場): void {
        急勾配を岩肌へベイクする処理(this.材質データ, this.解像度, this.一辺のメートル, this.格子間隔, 対象高さ場)
    }

    // 前回の帯を覆う前の材質へ戻してから、今の道路の直下を泥100%へ塗り直す。
    public 道路下の泥の帯を焼き直す(道路一覧: ReadonlyArray<チャンクの道路>): void {
        道路下の泥の帯を焼き直す処理(
            this.材質データ,
            this.解像度,
            this.一辺のメートル,
            this.格子間隔,
            道路一覧,
            this._道路の泥の被覆の記録,
        )
    }

    // 焼き直しの差し戻しは、材質データと被覆の記録の両方を焼き直す前へ戻して初めて成り立つ。
    public 道路の泥の被覆の記録の写しを取る(): 道路の泥の被覆の記録の写し {
        return this._道路の泥の被覆の記録.写しを取る()
    }

    public 道路の泥の被覆の記録を写しから戻す(写し: 道路の泥の被覆の記録の写し): void {
        this._道路の泥の被覆の記録.写しから戻す(写し)
    }

    public 低地を泥へベイクする(対象高さ場: 高さ場): void {
        低地を泥へベイクする処理(this.材質データ, this.解像度, this.一辺のメートル, this.格子間隔, 対象高さ場)
    }
}
