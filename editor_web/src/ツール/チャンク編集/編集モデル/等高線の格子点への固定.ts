import type { 等高線, 平面の位置 } from '../../../生成/編集資源契約.ts'

// 等高線から高さ場を生成する計算の最初の工程。等高線の各線分を格子へラスタライズし、線が通る格子点を
// その線の高さに固定する。受け取るのは等高線一覧と格子の形、返すのは固定した格子点の印と高さである。
// 線分は格子間隔の半分以下の刻みで標本し、標本ごとに最寄りの格子点を固定する。刻みを格子間隔の半分に
// 抑えるのは、隣り合う標本の最寄り格子点が4近傍で繋がり、線が格子の上で途切れないためである。
// 参照: `_doc/設計/見下ろし図による地形編集.md`「判断3」。

export type 固定された格子点 = {
    readonly 固定の印: Uint8Array
    readonly 固定の高さ: Float32Array
}

export function 等高線を格子点へ固定する(
    等高線一覧: ReadonlyArray<等高線>,
    解像度: number,
    一辺のメートル: number,
): 固定された格子点 {
    const 格子間隔 = 一辺のメートル / (解像度 - 1)
    const 固定の印 = new Uint8Array(解像度 * 解像度)
    const 固定の高さ = new Float32Array(解像度 * 解像度)
    const 格子点を固定する = (位置: 平面の位置, 高さ: number): void => {
        const gx = Math.round((位置.x + 一辺のメートル / 2) / 格子間隔)
        const gz = Math.round((位置.z + 一辺のメートル / 2) / 格子間隔)
        if (gx < 0 || gx >= 解像度 || gz < 0 || gz >= 解像度) return
        const 添字 = gz * 解像度 + gx
        固定の印[添字] = 1
        固定の高さ[添字] = 高さ
    }
    for (const 線 of 等高線一覧) {
        for (const [始点, 終点] of 線分の列(線)) {
            線分を標本する(始点, 終点, 格子間隔 / 2, (位置) => 格子点を固定する(位置, 線.高さメートル))
        }
        if (線.頂点列.length === 1 && 線.頂点列[0] !== undefined) {
            格子点を固定する(線.頂点列[0], 線.高さメートル)
        }
    }
    return { 固定の印, 固定の高さ }
}

// 閉じた等高線は最後の頂点から最初の頂点へも線分を張る。
function 線分の列(線: 等高線): Array<[平面の位置, 平面の位置]> {
    const 列: Array<[平面の位置, 平面の位置]> = []
    for (let i = 0; i + 1 < 線.頂点列.length; i++) {
        const 始点 = 線.頂点列[i]
        const 終点 = 線.頂点列[i + 1]
        if (始点 !== undefined && 終点 !== undefined) 列.push([始点, 終点])
    }
    const 最初 = 線.頂点列[0]
    const 最後 = 線.頂点列[線.頂点列.length - 1]
    if (線.閉じている && 線.頂点列.length >= 3 && 最初 !== undefined && 最後 !== undefined) {
        列.push([最後, 最初])
    }
    return 列
}

function 線分を標本する(
    始点: 平面の位置,
    終点: 平面の位置,
    刻み: number,
    標本ごとの処理: (位置: 平面の位置) => void,
): void {
    const 長さ = Math.hypot(終点.x - 始点.x, 終点.z - 始点.z)
    const 分割数 = Math.max(1, Math.ceil(長さ / 刻み))
    for (let i = 0; i <= 分割数; i++) {
        const t = i / 分割数
        標本ごとの処理({ x: 始点.x + (終点.x - 始点.x) * t, z: 始点.z + (終点.z - 始点.z) * t })
    }
}
