import type { 平面の位置 } from '../../../生成/編集資源契約.ts'
import type { 等値線の線分 } from './等値線の線分抽出.ts'

// 高さ場から等高線を導く計算の第2工程。端点を共有する線分を繋いで折れ線にする。受け取るのは線分の一覧、
// 返すのは折れ線の一覧(頂点列と、始点へ戻って閉じたかどうか)である。
// 端点の一致は、格子の辺の上で同じ補間から出た2つの座標が浮動小数の丸めで食い違わないよう、
// 0.1ミリメートルの升へ丸めた文字列で判定する。
// 参照: `_doc/設計/見下ろし図による地形編集.md`「判断5」。

export type 連結した折れ線 = {
    readonly 頂点列: Array<平面の位置>
    readonly 閉じている: boolean
}

const 端点の一致を判定する丸めの逆数 = 10000

function 端点のキー(位置: 平面の位置): string {
    return `${Math.round(位置.x * 端点の一致を判定する丸めの逆数)},${Math.round(位置.z * 端点の一致を判定する丸めの逆数)}`
}

export function 線分を折れ線へ連結する(線分一覧: ReadonlyArray<等値線の線分>): Array<連結した折れ線> {
    const 端点から線分へ = new Map<string, Array<number>>()
    線分一覧.forEach((線分, 添字) => {
        for (const キー of [端点のキー(線分.始点), 端点のキー(線分.終点)]) {
            const 一覧 = 端点から線分へ.get(キー)
            if (一覧 === undefined) 端点から線分へ.set(キー, [添字])
            else 一覧.push(添字)
        }
    })
    const 使用済み = new Uint8Array(線分一覧.length)
    const 折れ線一覧: Array<連結した折れ線> = []
    for (let 添字 = 0; 添字 < 線分一覧.length; 添字++) {
        if (使用済み[添字] === 1) continue
        const 線分 = 線分一覧[添字]
        if (線分 === undefined) continue
        使用済み[添字] = 1
        const 前方 = 一方向へ伸ばす(線分.終点, 線分.始点, 線分一覧, 端点から線分へ, 使用済み)
        const 閉じている = 前方.length > 0 && 端点のキー(前方[前方.length - 1] ?? 線分.終点) === 端点のキー(線分.始点)
        if (閉じている) {
            前方.pop()
            折れ線一覧.push({ 頂点列: [線分.始点, 線分.終点, ...前方], 閉じている: true })
            continue
        }
        const 後方 = 一方向へ伸ばす(線分.始点, 線分.終点, 線分一覧, 端点から線分へ, 使用済み)
        折れ線一覧.push({ 頂点列: [...後方.reverse(), 線分.始点, 線分.終点, ...前方], 閉じている: false })
    }
    return 折れ線一覧
}

// 先端の端点を共有する未使用の線分を辿り、先端の先に続く頂点を順に返す。先端が出発点へ戻ったら止める。
function 一方向へ伸ばす(
    先端: 平面の位置,
    出発点: 平面の位置,
    線分一覧: ReadonlyArray<等値線の線分>,
    端点から線分へ: ReadonlyMap<string, ReadonlyArray<number>>,
    使用済み: Uint8Array,
): Array<平面の位置> {
    const 続く頂点: Array<平面の位置> = []
    let 現在 = 先端
    const 出発点のキー = 端点のキー(出発点)
    while (端点のキー(現在) !== 出発点のキー) {
        const 候補 = (端点から線分へ.get(端点のキー(現在)) ?? []).find((i) => 使用済み[i] === 0)
        if (候補 === undefined) break
        const 線分 = 線分一覧[候補]
        if (線分 === undefined) break
        使用済み[候補] = 1
        現在 = 端点のキー(線分.始点) === 端点のキー(現在) ? 線分.終点 : 線分.始点
        続く頂点.push(現在)
    }
    return 続く頂点
}
