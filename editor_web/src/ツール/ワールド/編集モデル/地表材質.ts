import type { 地表材質層, 材質の筆致, チャンクの道路 } from '../../../生成/編集資源契約.ts'
import type { 高さ場 } from './高さ場.ts'
import { 合計255へ正規化する } from './地表材質正規化.ts'
import { 急勾配を岩肌へベイクする処理, 道路下を泥へベイクする処理 } from './地表材質ベイク.ts'

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

    public static 合計255へ正規化する(草: number, 泥: number, 岩: number, 砂: number): [number, number, number, number] {
        return 合計255へ正規化する(草, 泥, 岩, 砂)
    }

    // 材質筆致を通過点列に沿って適用する。
    public 材質筆致を適用する(筆致: 材質の筆致): void {
        const 層インデックス = this.層をインデックスに変換する(筆致.層)
        for (const 通過点 of 筆致.通過点列) {
            for (let gz = 0; gz < this.解像度; gz++) {
                for (let gx = 0; gx < this.解像度; gx++) {
                    const wx = gx * this.格子間隔 - this.一辺のメートル / 2
                    const wz = gz * this.格子間隔 - this.一辺のメートル / 2
                    const 距離 = Math.hypot(wx - 通過点.x, wz - 通過点.z)
                    if (距離 < 筆致.半径メートル) {
                        const コサイン減衰 = Math.cos((距離 / 筆致.半径メートル) * (Math.PI / 2)) * 筆致.流量
                        const ピクセル先頭 = (gz * this.解像度 + gx) * 4
                        const 現在草 = this.材質データ[ピクセル先頭 + 0] ?? 0
                        const 現在泥 = this.材質データ[ピクセル先頭 + 1] ?? 0
                        const 現在岩 = this.材質データ[ピクセル先頭 + 2] ?? 0
                        const 現在砂 = this.材質データ[ピクセル先頭 + 3] ?? 0
                        const 加算値 = Math.floor(コサイン減衰 * 255)
                        const 新草 = 層インデックス === 0 ? Math.min(255, 現在草 + 加算値) : 現在草
                        const 新泥 = 層インデックス === 1 ? Math.min(255, 現在泥 + 加算値) : 現在泥
                        const 新岩 = 層インデックス === 2 ? Math.min(255, 現在岩 + 加算値) : 現在岩
                        const 新砂 = 層インデックス === 3 ? Math.min(255, 現在砂 + 加算値) : 現在砂
                        const [正規化草, 正規化泥, 正規化岩, 正規化砂] = 合計255へ正規化する(新草, 新泥, 新岩, 新砂)
                        this.材質データ[ピクセル先頭 + 0] = 正規化草
                        this.材質データ[ピクセル先頭 + 1] = 正規化泥
                        this.材質データ[ピクセル先頭 + 2] = 正規化岩
                        this.材質データ[ピクセル先頭 + 3] = 正規化砂
                    }
                }
            }
        }
    }

    public 急勾配を岩肌へベイクする(対象高さ場: 高さ場): void {
        急勾配を岩肌へベイクする処理(this.材質データ, this.解像度, this.一辺のメートル, this.格子間隔, 対象高さ場)
    }

    public 道路下を泥へベイクする(道路: チャンクの道路): void {
        道路下を泥へベイクする処理(this.材質データ, this.解像度, this.一辺のメートル, this.格子間隔, 道路)
    }

    private 層をインデックスに変換する(層: 地表材質層): number {
        switch (層) {
            case '草': return 0
            case '泥': return 1
            case '岩': return 2
            case '砂': return 3
        }
        const 未知の層: never = 層
        throw new Error(`未知の地表材質層: ${JSON.stringify(未知の層)}`)
    }
}
