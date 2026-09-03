import type { 等高線, 平面の位置 } from '../../../../../生成/編集資源契約.ts'

// 等高線の頂点への当たり。どの線の何番目の頂点かを指す。
export interface 頂点の当たり {
    readonly 線の添字: number
    readonly 頂点の添字: number
}

// 見下ろし図の上のワールドXZで、等高線の頂点と線分への当たりを求める純粋関数の集まり。
// 当たりの半径はメートルで受け取る。呼び出し側が画素の閾値を視点の倍率で割ってメートルへ直す。
// ズームに応じて当たりの大きさが画面上で一定になるようにするためである。

export function 二点の距離(甲: 平面の位置, 乙: 平面の位置): number {
    return Math.hypot(甲.x - 乙.x, 甲.z - 乙.z)
}

// 点から線分への最短距離。線分が1点に潰れているときは端点への距離になる。
export function 点と線分の距離(点: 平面の位置, 端甲: 平面の位置, 端乙: 平面の位置): number {
    const dx = 端乙.x - 端甲.x
    const dz = 端乙.z - 端甲.z
    const 長さの二乗 = dx * dx + dz * dz
    if (長さの二乗 === 0) return 二点の距離(点, 端甲)
    const t = Math.min(1, Math.max(0, ((点.x - 端甲.x) * dx + (点.z - 端甲.z) * dz) / 長さの二乗))
    return 二点の距離(点, { x: 端甲.x + dx * t, z: 端甲.z + dz * t })
}

// 位置に最も近い頂点を全等高線から探す。半径の外ならnullを返す。
export function 頂点の当たりを探す(一覧: readonly 等高線[], 位置: 平面の位置, 半径メートル: number): 頂点の当たり | null {
    let 最近: 頂点の当たり | null = null
    let 最近の距離 = 半径メートル
    一覧.forEach((線, 線の添字) => {
        線.頂点列.forEach((頂点, 頂点の添字) => {
            const 距離 = 二点の距離(位置, 頂点)
            if (距離 <= 最近の距離) {
                最近の距離 = 距離
                最近 = { 線の添字, 頂点の添字 }
            }
        })
    })
    return 最近
}

// 線の線分の列。閉じている線は末尾から先頭へ戻る線分を含む。頂点1つの線は線分を持たない。
export function 線分の列を作る(線: 等高線): ReadonlyArray<readonly [平面の位置, 平面の位置]> {
    const 列: Array<readonly [平面の位置, 平面の位置]> = []
    for (let i = 0; i + 1 < 線.頂点列.length; i++) {
        const 甲 = 線.頂点列[i]
        const 乙 = 線.頂点列[i + 1]
        if (甲 !== undefined && 乙 !== undefined) 列.push([甲, 乙])
    }
    const 先頭 = 線.頂点列[0]
    const 末尾 = 線.頂点列[線.頂点列.length - 1]
    if (線.閉じている && 線.頂点列.length >= 3 && 先頭 !== undefined && 末尾 !== undefined) 列.push([末尾, 先頭])
    return 列
}

// 等高線の辺(線分)への当たり。どの線の、頂点列の何番目と何番目を結ぶ辺かを指す。
// 辺のドラッグはこの2つの頂点添字へ同じ差分を加える。
export interface 辺の当たり {
    readonly 線の添字: number
    readonly 頂点の添字甲: number
    readonly 頂点の添字乙: number
}

// 線の頂点添字の隣接対の列を作る。線分の列を作るの添字版で、辺のドラッグが動かす2頂点を特定するために使う。
function 頂点添字の対の列を作る(線: 等高線): ReadonlyArray<readonly [number, number]> {
    const 列: Array<readonly [number, number]> = []
    for (let i = 0; i + 1 < 線.頂点列.length; i++) 列.push([i, i + 1])
    if (線.閉じている && 線.頂点列.length >= 3) 列.push([線.頂点列.length - 1, 0])
    return 列
}

// 位置に最も近い辺を全等高線から探す。半径の外ならnullを返す。
export function 辺の当たりを探す(一覧: readonly 等高線[], 位置: 平面の位置, 半径メートル: number): 辺の当たり | null {
    let 最近: 辺の当たり | null = null
    let 最近の距離 = 半径メートル
    一覧.forEach((線, 線の添字) => {
        頂点添字の対の列を作る(線).forEach(([甲添字, 乙添字]) => {
            const 甲 = 線.頂点列[甲添字]
            const 乙 = 線.頂点列[乙添字]
            if (甲 === undefined || 乙 === undefined) return
            const 距離 = 点と線分の距離(位置, 甲, 乙)
            if (距離 <= 最近の距離) {
                最近の距離 = 距離
                最近 = { 線の添字, 頂点の添字甲: 甲添字, 頂点の添字乙: 乙添字 }
            }
        })
    })
    return 最近
}

// 描いている途中の線の始点をクリックして閉じる判定。頂点が3つ未満では閉じても面にならないため閉じない。
export function 始点で閉じるか(頂点列: readonly 平面の位置[], 位置: 平面の位置, 半径メートル: number): boolean {
    const 始点 = 頂点列[0]
    return 頂点列.length >= 3 && 始点 !== undefined && 二点の距離(始点, 位置) <= 半径メートル
}
