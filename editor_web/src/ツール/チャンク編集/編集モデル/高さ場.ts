import type { 位置3次元, 造成筆致 } from '../../../生成/編集資源契約.ts'
import { 単一通過点の造成筆致を適用する } from './造成筆致適用.ts'

// Float32格子による高さ場の幾何学的データと造成処理を管理する。
export class 高さ場 {
    public readonly 解像度: number
    public readonly 一辺のメートル: number
    public readonly 格子間隔: number
    public readonly 格子データ: Float32Array

    public constructor(解像度: number, 一辺のメートル: number, 初期データ?: Float32Array) {
        if (解像度 < 2) throw new Error(`解像度は2以上でなければならない: ${解像度}`)
        if (一辺のメートル <= 0) throw new Error(`一辺のメートルは正の数でなければならない: ${一辺のメートル}`)
        this.解像度 = 解像度
        this.一辺のメートル = 一辺のメートル
        this.格子間隔 = 一辺のメートル / (解像度 - 1)
        const 必要要素数 = 解像度 * 解像度
        if (初期データ !== undefined) {
            if (初期データ.length !== 必要要素数) {
                throw new Error(`初期データの要素数が不正: 期待=${必要要素数}, 実際=${初期データ.length}`)
            }
            this.格子データ = new Float32Array(初期データ)
        } else {
            this.格子データ = new Float32Array(必要要素数)
        }
    }

    public 複製する(): 高さ場 {
        return new 高さ場(this.解像度, this.一辺のメートル, this.格子データ)
    }

    // 格子データ全体を差し戻し・上書き用の別データへ置き換える。要素数の不一致は例外にする。
    public 格子データを置き換える(データ: Float32Array): void {
        if (データ.length !== this.格子データ.length) {
            throw new Error(`置き換える格子データの要素数が不正: 期待=${this.格子データ.length}, 実際=${データ.length}`)
        }
        this.格子データ.set(データ)
    }

    // 単一の格子点の標高を設定する。範囲外添字は例外にする。
    public 標高を格子添字で設定する(格子添字: number, 標高: number): void {
        if (格子添字 < 0 || 格子添字 >= this.格子データ.length) {
            throw new Error(`格子添字が範囲外: 添字=${格子添字}, 要素数=${this.格子データ.length}`)
        }
        this.格子データ[格子添字] = 標高
    }

    // 双線形補間により任意のワールド座標における標本高さを計算する。
    public 標本高さを取得する(ワールドX: number, ワールドZ: number): number {
        const gx = (ワールドX + this.一辺のメートル / 2) / this.格子間隔
        const gz = (ワールドZ + this.一辺のメートル / 2) / this.格子間隔
        if (gx < 0 || gx >= this.解像度 - 1 || gz < 0 || gz >= this.解像度 - 1) return 0
        const x0 = Math.floor(gx)
        const z0 = Math.floor(gz)
        const fx = gx - x0
        const fz = gz - z0
        const h00 = this.格子データ[z0 * this.解像度 + x0] ?? 0
        const h10 = this.格子データ[z0 * this.解像度 + (x0 + 1)] ?? 0
        const h01 = this.格子データ[(z0 + 1) * this.解像度 + x0] ?? 0
        const h11 = this.格子データ[(z0 + 1) * this.解像度 + (x0 + 1)] ?? 0
        return (1 - fx) * (1 - fz) * h00 + fx * (1 - fz) * h10 + (1 - fx) * fz * h01 + fx * fz * h11
    }

    // 中心差分法により任意のワールド座標における法線ベクトルを正規化して計算する。
    public 標本法線を取得する(ワールドX: number, ワールドZ: number): 位置3次元 {
        const eps = 0.5
        const hL = this.標本高さを取得する(ワールドX - eps, ワールドZ)
        const hR = this.標本高さを取得する(ワールドX + eps, ワールドZ)
        const hD = this.標本高さを取得する(ワールドX, ワールドZ - eps)
        const hU = this.標本高さを取得する(ワールドX, ワールドZ + eps)
        const vx = hL - hR
        const vy = 2.0 * eps
        const vz = hD - hU
        const 長さ = Math.hypot(vx, vy, vz)
        if (長さ === 0) return { x: 0, y: 1, z: 0 }
        return { x: vx / 長さ, y: vy / 長さ, z: vz / 長さ }
    }

    // 造成筆致を通過点列に沿って順に適用する。
    public 造成筆致を適用する(筆致: 造成筆致): void {
        for (const 通過点 of 筆致.通過点列) {
            単一通過点の造成筆致を適用する(
                this.格子データ,
                this.解像度,
                this.一辺のメートル,
                this.格子間隔,
                通過点,
                筆致,
            )
        }
    }
}
