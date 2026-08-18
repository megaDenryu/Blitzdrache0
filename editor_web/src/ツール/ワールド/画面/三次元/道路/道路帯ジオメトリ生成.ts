import { Vector3, CatmullRomCurve3 } from 'three'
import type { 道路スプライン, 高さ場 } from '../../../編集モデル/index.ts'

export interface 道路帯幾何データ {
    readonly 頂点配列: Float32Array
    readonly UV配列: Float32Array
    readonly 添字配列: Uint32Array
}

// 道路スプラインの制御点列から帯状メッシュの頂点・UV・インデックスを生成する。
export function 道路帯幾何データを生成する(
    スプライン: 道路スプライン,
    全幅メートル: number,
    細分割数: number,
    高さ場モデル: 高さ場,
    Yオフセット: number,
): 道路帯幾何データ | null {
    if (スプライン.制御点列.length < 2) return null

    const 頂点列 = スプライン.制御点列.map((p) => new Vector3(p.x, p.y, p.z))
    const 曲線 = new CatmullRomCurve3(頂点列, false, 'centripetal')
    const 標本点列 = 曲線.getSpacedPoints(細分割数)

    const 横分割数 = 4
    const 頂点数 = (細分割数 + 1) * (横分割数 + 1)
    const 頂点配列 = new Float32Array(頂点数 * 3)
    const UV配列 = new Float32Array(頂点数 * 2)

    let 頂点添字 = 0
    for (let i = 0; i <= 細分割数; i++) {
        const t = i / 細分割数
        const pt = 標本点列[i] ?? new Vector3()
        const 接線 = 曲線.getTangentAt(t).normalize()
        const 横ベクトル = new Vector3(-接線.z, 0, 接線.x).normalize()

        for (let j = 0; j <= 横分割数; j++) {
            const 横比率 = j / 横分割数 - 0.5
            const wx = pt.x + 横ベクトル.x * (全幅メートル * 横比率)
            const wz = pt.z + 横ベクトル.z * (全幅メートル * 横比率)
            const wy = 高さ場モデル.標本高さを取得する(wx, wz) + Yオフセット

            頂点配列[頂点添字 * 3 + 0] = wx
            頂点配列[頂点添字 * 3 + 1] = wy
            頂点配列[頂点添字 * 3 + 2] = wz

            UV配列[頂点添字 * 2 + 0] = j / 横分割数
            UV配列[頂点添字 * 2 + 1] = t * 20

            頂点添字++
        }
    }

    const 面数 = 細分割数 * 横分割数
    const 添字配列 = new Uint32Array(面数 * 6)
    let 添字位置 = 0

    for (let i = 0; i < 細分割数; i++) {
        for (let j = 0; j < 横分割数; j++) {
            const 行1 = i * (横分割数 + 1) + j
            const 行2 = (i + 1) * (横分割数 + 1) + j

            添字配列[添字位置++] = 行1
            添字配列[添字位置++] = 行2
            添字配列[添字位置++] = 行1 + 1

            添字配列[添字位置++] = 行1 + 1
            添字配列[添字位置++] = 行2
            添字配列[添字位置++] = 行2 + 1
        }
    }

    return { 頂点配列, UV配列, 添字配列 }
}
