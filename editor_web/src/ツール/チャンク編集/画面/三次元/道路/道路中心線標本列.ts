import { CatmullRomCurve3, Vector3 } from 'three'
import type { 道路スプライン } from '../../../編集モデル/index.ts'

// 帯の幅は水平面(XZ)の上だけで張り、高さは組み立ての最後に高さ場から取り直す。
// そのため中心線の標本は水平の2成分だけを持つ。
export interface 水平位置 {
    readonly x: number
    readonly z: number
}

// 中心線標本とは、道路の中心線上の1点と、そこでの単位長さの進行方向および横方向を
// 組にしたもののことである。横方向は進行方向を左へ直角に回した向きであり、
// 帯の列はこの向きに沿って左端から右端まで並ぶ。
export interface 中心線標本 {
    readonly 中心位置: 水平位置
    readonly 進行方向: 水平位置
    readonly 横方向: 水平位置
}

const 標本数の下限 = 24
const 制御点1つあたりの標本数の下限 = 24

// 曲がりのきつい区間で標本が粗いと、折れ角が1つの標本の中に押し込まれて帯の形が崩れる。
// 制御点の数に比例した下限を課し、利用者が細分割数を下げても密度が不足しないようにする。
export function 中心線の標本数を決める(制御点の数: number, 細分割数: number): number {
    const 下限 = Math.max(標本数の下限, (制御点の数 - 1) * 制御点1つあたりの標本数の下限)
    return Math.max(細分割数, 下限)
}

function 単位方向を求める(始点: 水平位置, 終点: 水平位置, 代替: 水平位置): 水平位置 {
    const 差X = 終点.x - 始点.x
    const 差Z = 終点.z - 始点.z
    const 長さ = Math.hypot(差X, 差Z)
    if (!Number.isFinite(長さ) || 長さ < 1e-9) return 代替
    return { x: 差X / 長さ, z: 差Z / 長さ }
}

// 進行方向は曲線の接線ではなく、前後の弦の向きをならして決める。道路は上下にも起伏するため、
// 三次元の接線を水平へ落とすと、急な坂の頂点で水平成分が反転して弦と逆向きになることがある。
// 弦からならした向きなら、真後ろへ折り返さない限り弦と同じ側を向く。
function 前後の弦をならす(手前の弦: 水平位置, 次の弦: 水平位置): 水平位置 {
    const 和X = 手前の弦.x + 次の弦.x
    const 和Z = 手前の弦.z + 次の弦.z
    const 長さ = Math.hypot(和X, 和Z)
    if (長さ < 1e-6) return 次の弦
    return { x: 和X / 長さ, z: 和Z / 長さ }
}

// 制御点列のCatmull-Rom曲線を等間隔に標本化し、各標本の向きまで決めた列を返す。
export function 中心線標本列を作る(スプライン: 道路スプライン, 標本数: number): 中心線標本[] {
    const 頂点列 = スプライン.制御点列.map((点) => new Vector3(点.x, 点.y, 点.z))
    const 曲線 = new CatmullRomCurve3(頂点列, false, 'centripetal')
    // 注意: 等間隔の標本化はthree.jsが内部に持つ弧長の対応表を通る。既定の分割数は200であり、
    // これを標本数が上回ると隣り合う標本が同じ位置に重なって帯が崩れる。
    曲線.arcLengthDivisions = Math.max(200, 標本数 * 4)
    const 中心位置列: 水平位置[] = 曲線.getSpacedPoints(標本数).map((点) => ({ x: 点.x, z: 点.z }))

    const 弦方向列: 水平位置[] = []
    let 直前の弦方向: 水平位置 = { x: 1, z: 0 }
    for (let 行 = 0; 行 + 1 < 中心位置列.length; 行++) {
        直前の弦方向 = 単位方向を求める(
            中心位置列[行] ?? { x: 0, z: 0 },
            中心位置列[行 + 1] ?? { x: 0, z: 0 },
            直前の弦方向,
        )
        弦方向列.push(直前の弦方向)
    }

    return 中心位置列.map((中心位置, 行) => {
        const 手前の弦 = 弦方向列[行 - 1] ?? 弦方向列[行] ?? 直前の弦方向
        const 次の弦 = 弦方向列[行] ?? 弦方向列[行 - 1] ?? 直前の弦方向
        const 進行方向 = 前後の弦をならす(手前の弦, 次の弦)
        return { 中心位置, 進行方向, 横方向: { x: -進行方向.z, z: 進行方向.x } }
    })
}
