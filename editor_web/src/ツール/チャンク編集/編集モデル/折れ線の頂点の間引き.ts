import type { 平面の位置 } from '../../../生成/編集資源契約.ts'

// 高さ場から等高線を導く計算の第3工程。ダグラス・ポイカー法で、両端を結ぶ線分からの距離が許容以内の
// 頂点を取り除く。受け取るのは折れ線の頂点列と許容メートル、返すのは間引いた頂点列である。
// 再帰でなく区間の山で回すのは、頂点が数千に及ぶ長い等値線でも呼び出しの深さに依らないためである。
// 参照: `_doc/設計/見下ろし図による地形編集.md`「判断5」。
export function 折れ線の頂点を間引く(頂点列: ReadonlyArray<平面の位置>, 許容メートル: number): Array<平面の位置> {
    if (頂点列.length <= 2) return 頂点列.map((p) => ({ x: p.x, z: p.z }))
    const 残す = new Uint8Array(頂点列.length)
    残す[0] = 1
    残す[頂点列.length - 1] = 1
    const 区間の山: Array<[number, number]> = [[0, 頂点列.length - 1]]
    while (区間の山.length > 0) {
        const 区間 = 区間の山.pop()
        if (区間 === undefined) break
        const [始, 終] = 区間
        const 始点 = 頂点列[始]
        const 終点 = 頂点列[終]
        if (始点 === undefined || 終点 === undefined) continue
        let 最大距離 = 0
        let 最遠 = -1
        for (let i = 始 + 1; i < 終; i++) {
            const 点 = 頂点列[i]
            if (点 === undefined) continue
            const 距離 = 点と線分の距離(点, 始点, 終点)
            if (距離 > 最大距離) {
                最大距離 = 距離
                最遠 = i
            }
        }
        if (最遠 >= 0 && 最大距離 > 許容メートル) {
            残す[最遠] = 1
            区間の山.push([始, 最遠], [最遠, 終])
        }
    }
    return 頂点列.filter((_, i) => 残す[i] === 1).map((p) => ({ x: p.x, z: p.z }))
}

function 点と線分の距離(点: 平面の位置, 始点: 平面の位置, 終点: 平面の位置): number {
    const dx = 終点.x - 始点.x
    const dz = 終点.z - 始点.z
    const 長さの二乗 = dx * dx + dz * dz
    if (長さの二乗 === 0) return Math.hypot(点.x - 始点.x, 点.z - 始点.z)
    const t = Math.min(1, Math.max(0, ((点.x - 始点.x) * dx + (点.z - 始点.z) * dz) / 長さの二乗))
    return Math.hypot(点.x - (始点.x + dx * t), 点.z - (始点.z + dz * t))
}
