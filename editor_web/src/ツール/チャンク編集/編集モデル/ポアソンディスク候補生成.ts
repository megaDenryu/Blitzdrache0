import type { 決定的乱数生成器 } from './乱数.ts'

export type 散布点 = { readonly x: number, readonly z: number }

// 領域内に最小間隔を保ったポアソンディスク散布候補点列を決定的に生成する。
export function ポアソンディスク候補を生成する(
    最小間隔: number,
    一辺: number,
    最大数: number,
    乱数: 決定的乱数生成器,
): Array<散布点> {
    const 点列: Array<散布点> = []
    const k = 30
    const セル寸法 = 最小間隔 / Math.SQRT2
    const 格子数 = Math.ceil(一辺 / セル寸法)
    const セル格子 = new Int32Array(格子数 * 格子数).fill(-1)
    const 生成待ち一覧: Array<number> = []

    const 点を追加する = (x: number, z: number): void => {
        const 添字 = 点列.length
        点列.push({ x, z })
        const gx = Math.floor((x + 一辺 / 2) / セル寸法)
        const gz = Math.floor((z + 一辺 / 2) / セル寸法)
        if (gx >= 0 && gx < 格子数 && gz >= 0 && gz < 格子数) {
            セル格子[gz * 格子数 + gx] = 添字
        }
        生成待ち一覧.push(添字)
    }

    点を追加する(0, 0)
    while (生成待ち一覧.length > 0 && 点列.length < 最大数) {
        const 待ち添字 = Math.floor(乱数.次の実数() * 生成待ち一覧.length)
        const 中心点 = 点列[生成待ち一覧[待ち添字]!]!
        let 候補追加成功 = false
        for (let i = 0; i < k; i++) {
            const 角度 = 乱数.次の実数() * Math.PI * 2
            const 距離 = 最小間隔 * (1 + 乱数.次の実数())
            const nx = 中心点.x + Math.cos(角度) * 距離
            const nz = 中心点.z + Math.sin(角度) * 距離
            if (nx < -一辺 / 2 + 5 || nx > 一辺 / 2 - 5 || nz < -一辺 / 2 + 5 || nz > 一辺 / 2 - 5) continue
            const gx = Math.floor((nx + 一辺 / 2) / セル寸法)
            const gz = Math.floor((nz + 一辺 / 2) / セル寸法)
            let 衝突なし = true
            for (let dy = -2; dy <= 2; dy++) {
                for (let dx = -2; dx <= 2; dx++) {
                    const cx = gx + dx
                    const cz = gz + dy
                    if (cx >= 0 && cx < 格子数 && cz >= 0 && cz < 格子数) {
                        const 近傍添字 = セル格子[cz * 格子数 + cx]!
                        if (近傍添字 >= 0) {
                            const 近傍点 = 点列[近傍添字]!
                            if (Math.hypot(nx - 近傍点.x, nz - 近傍点.z) < 最小間隔) {
                                衝突なし = false
                                break
                            }
                        }
                    }
                }
                if (!衝突なし) break
            }
            if (衝突なし) {
                点を追加する(nx, nz)
                候補追加成功 = true
                break
            }
        }
        if (!候補追加成功) {
            生成待ち一覧.splice(待ち添字, 1)
        }
    }
    return 点列
}
