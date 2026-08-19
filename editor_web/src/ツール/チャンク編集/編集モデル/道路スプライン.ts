import type { 位置3次元, チャンクの道路, 広域道路 } from '../../../生成/編集資源契約.ts'
import type { 高さ場 } from './高さ場.ts'
import { CatmullRomCurve3, Vector3 } from 'three'

// 道路スプラインの制御点列操作および切土盛土の造成処理を管理する。
export class 道路スプライン {
    public readonly 制御点列: Array<位置3次元>
    public 全幅メートル: number
    public 散布除外バッファメートル: number
    public 細分割数: number

    public constructor(設定: チャンクの道路 | 広域道路) {
        this.制御点列 = 設定.制御点列.map((p: 位置3次元) => ({ x: p.x, y: p.y, z: p.z }))
        this.全幅メートル = 設定.全幅メートル
        this.細分割数 = 設定.細分割数
        if ('散布除外バッファメートル' in 設定) {
            this.散布除外バッファメートル = 設定.散布除外バッファメートル
        } else {
            this.散布除外バッファメートル = 設定.全幅メートル * 1.5
        }
    }

    public 点を追加する(位置: 位置3次元): void {
        this.制御点列.push({ x: 位置.x, y: 位置.y, z: 位置.z })
    }

    public 点を移動する(添字: number, 新しい位置: 位置3次元): void {
        if (添字 < 0 || 添字 >= this.制御点列.length) {
            throw new Error(`添字が範囲外: 添字=${添字}, 長さ=${this.制御点列.length}`)
        }
        this.制御点列[添字] = { x: 新しい位置.x, y: 新しい位置.y, z: 新しい位置.z }
    }

    public 点を削除する(添字: number): 位置3次元 {
        if (添字 < 0 || 添字 >= this.制御点列.length) {
            throw new Error(`添字が範囲外: 添字=${添字}, 長さ=${this.制御点列.length}`)
        }
        const 削除点 = this.制御点列.splice(添字, 1)[0]!
        return 削除点
    }

    // 削除の差し戻しなど、末尾以外の添字へ点を復元するときに使う。
    public 点を添字へ挿入する(添字: number, 位置: 位置3次元): void {
        if (添字 < 0 || 添字 > this.制御点列.length) {
            throw new Error(`添字が範囲外: 添字=${添字}, 長さ=${this.制御点列.length}`)
        }
        this.制御点列.splice(添字, 0, { x: 位置.x, y: 位置.y, z: 位置.z })
    }

    // Catmull-Rom曲線から等間隔に標本点列を計算する。
    public 標本点列を計算する(標本数: number): Array<Vector3> {
        if (this.制御点列.length < 2) return []
        const 頂点列 = this.制御点列.map((p) => new Vector3(p.x, p.y, p.z))
        const 曲線 = new CatmullRomCurve3(頂点列, false, 'centripetal')
        return 曲線.getSpacedPoints(標本数)
    }

    // 道路に合わせて高さ場を切土・盛土する。
    public 道路に合わせて切土盛土する(対象高さ場: 高さ場): void {
        if (this.制御点列.length < 2) return
        const 標本点列 = this.標本点列を計算する(this.細分割数 * 3)
        const 道路半幅 = this.全幅メートル * 0.5
        const なじみ距離 = Math.max(道路半幅 + 0.001, this.散布除外バッファメートル)

        for (let gz = 0; gz < 対象高さ場.解像度; gz++) {
            for (let gx = 0; gx < 対象高さ場.解像度; gx++) {
                const wx = gx * 対象高さ場.格子間隔 - 対象高さ場.一辺のメートル / 2
                const wz = gz * 対象高さ場.格子間隔 - 対象高さ場.一辺のメートル / 2
                const 格子添字 = gz * 対象高さ場.解像度 + gx
                const 現在高さ = 対象高さ場.格子データ[格子添字] ?? 0

                let 最小距離 = Number.POSITIVE_INFINITY
                let 目標高さ = 現在高さ
                for (const p of 標本点列) {
                    const 距離 = Math.hypot(wx - p.x, wz - p.z)
                    if (距離 < 最小距離) {
                        最小距離 = 距離
                        目標高さ = p.y
                    }
                }

                if (最小距離 <= 道路半幅) {
                    対象高さ場.標高を格子添字で設定する(格子添字, 目標高さ)
                } else if (最小距離 < なじみ距離) {
                    const t = (最小距離 - 道路半幅) / (なじみ距離 - 道路半幅)
                    const スムースステップ = t * t * (3 - 2 * t)
                    対象高さ場.標高を格子添字で設定する(格子添字, 目標高さ * (1 - スムースステップ) + 現在高さ * スムースステップ)
                }
            }
        }
    }
}
