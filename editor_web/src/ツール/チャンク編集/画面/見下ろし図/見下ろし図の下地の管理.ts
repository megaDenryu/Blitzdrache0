import type { 高さ場, 地表材質 } from '../../編集モデル/index.ts'
import { 既定の三次元の配色を作る } from '../三次元/三次元の配色.ts'
import { 下地の画像を作る } from './下地の描画.ts'
import type { 下地の配色 } from './下地の配色.ts'
import type { 見下ろし図の視点 } from './見下ろし図の視点.ts'

// チャンクの外枠の線。下地の縁と重なって読めなくならないよう明るい色で1本引く。
const 外枠の色 = 'rgba(255, 255, 255, 0.7)'
const 外枠の太さ = 1

// 見下ろし図の下地(高さ場と地表材質から作った画像)の保持と描画。触れるのは最後に受け取った地形と配色、
// 作り直しが要るかの印、描くための裏のキャンバスだけである。
// 見下ろし図部品が「いつ作り直すか」を決め、この型は「何から作りどう描くか」だけを持つ。
// 材質の識別色はマテリアル台帳から後で注入されるため、届くまでは三次元と同じ既定の値で描く。
export class 見下ろし図の下地の管理 {
    private _高さ場: 高さ場 | null = null
    private _地表材質: 地表材質 | null = null
    private _配色: 下地の配色
    private _作り直しが要る: boolean = false
    private _下地キャンバス: HTMLCanvasElement | null = null

    public constructor() {
        const 既定 = 既定の三次元の配色を作る()
        this._配色 = {
            材質色: { 草: '#2d5a27', 泥: '#5c4033', 岩: '#64748b', 砂: '#d4b483' },
            標高低色: 既定.標高低色,
            標高中色: 既定.標高中色,
            標高高色: 既定.標高高色,
        }
    }

    public get 配色(): 下地の配色 {
        return this._配色
    }

    public get 一辺のメートル(): number | null {
        return this._高さ場?.一辺のメートル ?? null
    }

    public 地形を受け取る(高さ場モデル: 高さ場, 地表材質モデル: 地表材質): void {
        this._高さ場 = 高さ場モデル
        this._地表材質 = 地表材質モデル
        this._作り直しが要る = true
    }

    public 配色を設定する(配色: Partial<下地の配色>): void {
        this._配色 = { ...this._配色, ...配色 }
        this._作り直しが要る = true
    }

    // 作り直しが要るときだけ画像を作り、作ったかどうかを返す。地形をまだ受け取っていないときは何もしない。
    public 作り直す(): boolean {
        if (!this._作り直しが要る || this._高さ場 === null || this._地表材質 === null) return false
        const 画像 = 下地の画像を作る(this._高さ場, this._地表材質, this._配色)
        const キャンバス = this._下地キャンバス ?? document.createElement('canvas')
        キャンバス.width = 画像.幅
        キャンバス.height = 画像.高さ
        const 文脈 = キャンバス.getContext('2d')
        if (文脈 === null) throw new Error('下地のcanvas要素から2Dの描画文脈を取得できない')
        文脈.putImageData(new ImageData(画像.画素, 画像.幅, 画像.高さ), 0, 0)
        this._下地キャンバス = キャンバス
        this._作り直しが要る = false
        return true
    }

    // 下地の画素は格子点の上に中心が来るよう、チャンクの範囲を格子間隔の半分だけ外へ広げた矩形へ拡大して描く。
    // 格子1点が1画素の意味を保つため補間はしない。外枠はチャンクの範囲そのものに引く。
    public 描く(文脈: CanvasRenderingContext2D, 視点: 見下ろし図の視点): void {
        if (this._高さ場 === null || this._下地キャンバス === null) return
        const 半分 = this._高さ場.一辺のメートル / 2
        const 半格子 = this._高さ場.格子間隔 / 2
        const 左上 = 視点.ワールドから画素へ({ x: -半分 - 半格子, z: -半分 - 半格子 })
        const 右下 = 視点.ワールドから画素へ({ x: 半分 + 半格子, z: 半分 + 半格子 })
        文脈.imageSmoothingEnabled = false
        文脈.drawImage(this._下地キャンバス, 左上.x, 左上.y, 右下.x - 左上.x, 右下.y - 左上.y)
        const 枠の左上 = 視点.ワールドから画素へ({ x: -半分, z: -半分 })
        const 枠の右下 = 視点.ワールドから画素へ({ x: 半分, z: 半分 })
        文脈.strokeStyle = 外枠の色
        文脈.lineWidth = 外枠の太さ
        文脈.strokeRect(枠の左上.x, 枠の左上.y, 枠の右下.x - 枠の左上.x, 枠の右下.y - 枠の左上.y)
    }
}
