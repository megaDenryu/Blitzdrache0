import type { 材質の筆致, チャンクの道路 } from '../../../生成/編集資源契約.ts'
import type { 高さ場 } from './高さ場.ts'
import { 材質の筆致を塗る処理 } from './材質の筆致の塗り.ts'
import { 急勾配を岩肌へベイクする処理 } from './急勾配の岩肌ベイク.ts'
import { 低地を泥へベイクする処理 } from './低地の泥ベイク.ts'
import { 道路下を泥へベイクする処理 } from './道路下の泥ベイク.ts'

// 4層の地表材質（草・泥・岩・砂）の重みをu8の各画素合計255で管理する。
export class 地表材質 {
    public readonly 解像度: number
    public readonly 一辺のメートル: number
    public readonly 格子間隔: number
    public readonly 材質データ: Uint8Array

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
    }

    public 複製する(): 地表材質 {
        return new 地表材質(this.解像度, this.一辺のメートル, this.材質データ)
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

    public 道路下を泥へベイクする(道路一覧: ReadonlyArray<チャンクの道路>): void {
        道路下を泥へベイクする処理(this.材質データ, this.解像度, this.一辺のメートル, this.格子間隔, 道路一覧)
    }

    public 低地を泥へベイクする(対象高さ場: 高さ場): void {
        低地を泥へベイクする処理(this.材質データ, this.解像度, this.一辺のメートル, this.格子間隔, 対象高さ場)
    }
}
