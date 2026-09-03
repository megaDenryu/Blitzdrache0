import type { ワールドXZ } from '../見下ろし図の視点.ts'

// 大升の(列,行)。列はワールドのXが増える向き、行はZが増える向き(見下ろし図の下向き)に数える。
export interface 大升の番地 {
    readonly 列: number
    readonly 行: number
}

// 大升の格子の寸法。チャンクの一辺と、高さ格子の升目の一辺(格子間隔)と、大升が升目を縦横いくつまとめるかから決まる。
export interface 大升の格子 {
    readonly 一辺のメートル: number
    readonly 格子間隔: number
    readonly 大升の一辺の升目数: number
}

// 大升1つの一辺の長さ(メートル)。
export function 大升の一辺のメートル(格子: 大升の格子): number {
    return 格子.格子間隔 * 格子.大升の一辺の升目数
}

// チャンクの一辺に並ぶ大升の数。一辺が解像度を割り切らない設定は編集モデルが拒むため、ここでは切り上げない。
export function 一辺に並ぶ大升の数(格子: 大升の格子): number {
    return Math.floor(格子.一辺のメートル / 大升の一辺のメートル(格子))
}

// ワールドXZ(チャンク中心が原点)からその点を含む大升の番地を求める。チャンクの外ならnullを返す。
// 境界の上の点は右(下)の大升に入れ、チャンクの東端・南端だけは最後の大升へ入れる。
export function ワールドから大升へ(位置: ワールドXZ, 格子: 大升の格子): 大升の番地 | null {
    const 半分 = 格子.一辺のメートル / 2
    if (位置.x < -半分 || 位置.x > 半分 || 位置.z < -半分 || 位置.z > 半分) return null
    const 一辺 = 大升の一辺のメートル(格子)
    const 数 = 一辺に並ぶ大升の数(格子)
    const 列 = Math.min(数 - 1, Math.floor((位置.x + 半分) / 一辺))
    const 行 = Math.min(数 - 1, Math.floor((位置.z + 半分) / 一辺))
    return { 列, 行 }
}

// 大升の北西(x・zが小さい側)の角のワールドXZ。
export function 大升の北西の角(番地: 大升の番地, 格子: 大升の格子): ワールドXZ {
    const 半分 = 格子.一辺のメートル / 2
    const 一辺 = 大升の一辺のメートル(格子)
    return { x: -半分 + 番地.列 * 一辺, z: -半分 + 番地.行 * 一辺 }
}

export function 同じ大升か(甲: 大升の番地, 乙: 大升の番地): boolean {
    return 甲.列 === 乙.列 && 甲.行 === 乙.行
}
