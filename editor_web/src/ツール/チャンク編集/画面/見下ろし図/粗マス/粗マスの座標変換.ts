import type { ワールドXZ } from '../見下ろし図の視点.ts'

// 粗マスの(列,行)。列はワールドのXが増える向き、行はZが増える向き(見下ろし図の下向き)に数える。
export interface 粗マスの番地 {
    readonly 列: number
    readonly 行: number
}

// 粗マスの格子の寸法。チャンクの一辺と、高さ格子の升目の一辺(格子間隔)と、粗マスが升目を縦横いくつまとめるかから決まる。
// 値は不変であり、ワールド座標との相互変換をメソッドで持つ(参照: `見下ろし図の視点`と同じ様式)。
export class 粗マスの格子 {
    private constructor(
        public readonly 一辺のメートル: number,
        public readonly 格子間隔: number,
        public readonly 粗マスの一辺の升目数: number,
    ) {}

    public static 生成する(一辺のメートル: number, 格子間隔: number, 粗マスの一辺の升目数: number): 粗マスの格子 {
        if (一辺のメートル <= 0) throw new Error(`一辺のメートルは正の数でなければならない: ${一辺のメートル}`)
        if (格子間隔 <= 0) throw new Error(`格子間隔は正の数でなければならない: ${格子間隔}`)
        if (粗マスの一辺の升目数 <= 0) throw new Error(`粗マスの一辺の升目数は正の数でなければならない: ${粗マスの一辺の升目数}`)
        return new 粗マスの格子(一辺のメートル, 格子間隔, 粗マスの一辺の升目数)
    }

    // 粗マス1つの一辺の長さ(メートル)。
    public 粗マスの一辺のメートル(): number {
        return this.格子間隔 * this.粗マスの一辺の升目数
    }

    // チャンクの一辺に並ぶ粗マスの数。一辺が解像度を割り切らない設定は編集モデルが拒むため、ここでは切り上げない。
    public 一辺に並ぶ粗マスの数(): number {
        return Math.floor(this.一辺のメートル / this.粗マスの一辺のメートル())
    }

    // ワールドXZ(チャンク中心が原点)からその点を含む粗マスの番地を求める。チャンクの外ならnullを返す。
    // 境界の上の点は右(下)の粗マスに入れ、チャンクの東端・南端だけは最後の粗マスへ入れる。
    public ワールドから粗マスへ(位置: ワールドXZ): 粗マスの番地 | null {
        const 半分 = this.一辺のメートル / 2
        if (位置.x < -半分 || 位置.x > 半分 || 位置.z < -半分 || 位置.z > 半分) return null
        const 一辺 = this.粗マスの一辺のメートル()
        const 数 = this.一辺に並ぶ粗マスの数()
        const 列 = Math.min(数 - 1, Math.floor((位置.x + 半分) / 一辺))
        const 行 = Math.min(数 - 1, Math.floor((位置.z + 半分) / 一辺))
        return { 列, 行 }
    }

    // 粗マスの北西(x・zが小さい側)の角のワールドXZ。
    public 粗マスの北西の角(番地: 粗マスの番地): ワールドXZ {
        const 半分 = this.一辺のメートル / 2
        const 一辺 = this.粗マスの一辺のメートル()
        return { x: -半分 + 番地.列 * 一辺, z: -半分 + 番地.行 * 一辺 }
    }
}

export function 同じ粗マスか(甲: 粗マスの番地, 乙: 粗マスの番地): boolean {
    return 甲.列 === 乙.列 && 甲.行 === 乙.行
}
