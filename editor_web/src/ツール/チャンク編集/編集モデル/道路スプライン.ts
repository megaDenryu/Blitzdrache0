import type { 位置3次元, チャンクの道路, 広域道路 } from '../../../生成/編集資源契約.ts'
import type { 高さ場 } from './高さ場.ts'
import { 道路に合わせて切土盛土する処理 } from './道路の切土盛土.ts'
import { CatmullRomCurve3, Vector3 } from 'three'

// 1本の道路の制御点列と、その道路自身の全幅・散布除外バッファ・細分割数を保持する編集モデル。
// 道路一覧の中の1本ぶんであり、他の道路とは値を共有しない。
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

    public チャンクの道路として取り出す(): チャンクの道路 {
        return {
            制御点列: this.制御点列.map((p: 位置3次元) => ({ x: p.x, y: p.y, z: p.z })),
            全幅メートル: this.全幅メートル,
            散布除外バッファメートル: this.散布除外バッファメートル,
            細分割数: this.細分割数,
        }
    }

    public 広域道路として取り出す(): 広域道路 {
        return {
            制御点列: this.制御点列.map((p: 位置3次元) => ({ x: p.x, y: p.y, z: p.z })),
            全幅メートル: this.全幅メートル,
            細分割数: this.細分割数,
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
        const 道路半幅 = this.全幅メートル * 0.5
        道路に合わせて切土盛土する処理(
            this.標本点列を計算する(this.細分割数 * 3),
            道路半幅,
            Math.max(道路半幅 + 0.001, this.散布除外バッファメートル),
            対象高さ場,
        )
    }
}
