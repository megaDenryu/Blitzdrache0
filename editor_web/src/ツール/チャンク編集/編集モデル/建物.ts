import type { 建物の配置, 位置3次元 } from '../../../生成/編集資源契約.ts'
import type { 高さ場 } from './高さ場.ts'

// 建物およびPOIの配置・移動・削除および基礎平坦化の造成処理を管理する。
export class 建物の管理 {
    private readonly 建物マップ: Map<string, 建物の配置>

    public constructor(初期建物一覧?: ReadonlyArray<建物の配置>) {
        this.建物マップ = new Map<string, 建物の配置>()
        if (初期建物一覧 !== undefined) {
            for (const 建物 of 初期建物一覧) {
                this.建物マップ.set(建物.識別子, { ...建物 })
            }
        }
    }

    public 一覧を取得する(): Array<建物の配置> {
        return Array.from(this.建物マップ.values()).map((b) => ({ ...b }))
    }

    public 取得する(識別子: string): 建物の配置 {
        const 建物 = this.建物マップ.get(識別子)
        if (建物 === undefined) {
            throw new Error(`建物が見つからない: 識別子=${識別子}`)
        }
        return { ...建物 }
    }

    public 配置する(建物: 建物の配置): void {
        if (this.建物マップ.has(建物.識別子)) {
            throw new Error(`建物識別子が重複している: ${建物.識別子}`)
        }
        this.建物マップ.set(建物.識別子, { ...建物 })
    }

    public 移動する(識別子: string, 新しい位置: 位置3次元, 新しい向きラジアン: number): void {
        const 建物 = this.建物マップ.get(識別子)
        if (建物 === undefined) {
            throw new Error(`移動対象の建物が見つからない: 識別子=${識別子}`)
        }
        建物.位置 = { x: 新しい位置.x, y: 新しい位置.y, z: 新しい位置.z }
        建物.向きラジアン = 新しい向きラジアン
    }

    public 削除する(識別子: string): 建物の配置 {
        const 建物 = this.建物マップ.get(識別子)
        if (建物 === undefined) {
            throw new Error(`削除対象の建物が見つからない: 識別子=${識別子}`)
        }
        this.建物マップ.delete(識別子)
        return { ...建物 }
    }

    // 建物の基礎半径・なじみ半径に合わせて高さ場を平坦化する。
    public 基礎を平坦化する(識別子: string, 対象高さ場: 高さ場): void {
        const 建物 = this.建物マップ.get(識別子)
        if (建物 === undefined) {
            throw new Error(`平坦化対象の建物が見つからない: 識別子=${識別子}`)
        }
        const bx = 建物.位置.x
        const bz = 建物.位置.z
        const 基礎高さ = 建物.位置.y
        const 基礎半径 = 建物.基礎半径メートル
        const なじみ半径 = Math.max(基礎半径 + 0.001, 建物.なじみ半径メートル)

        for (let gz = 0; gz < 対象高さ場.解像度; gz++) {
            for (let gx = 0; gx < 対象高さ場.解像度; gx++) {
                const wx = gx * 対象高さ場.格子間隔 - 対象高さ場.一辺のメートル / 2
                const wz = gz * 対象高さ場.格子間隔 - 対象高さ場.一辺のメートル / 2
                const 距離 = Math.hypot(wx - bx, wz - bz)
                const 格子添字 = gz * 対象高さ場.解像度 + gx
                const 現在高さ = 対象高さ場.格子データ[格子添字] ?? 0

                if (距離 <= 基礎半径) {
                    対象高さ場.標高を格子添字で設定する(格子添字, 基礎高さ)
                } else if (距離 < なじみ半径) {
                    const t = (距離 - 基礎半径) / (なじみ半径 - 基礎半径)
                    const スムースステップ = t * t * (3 - 2 * t)
                    対象高さ場.標高を格子添字で設定する(格子添字, 基礎高さ * (1 - スムースステップ) + 現在高さ * スムースステップ)
                }
            }
        }
    }
}
