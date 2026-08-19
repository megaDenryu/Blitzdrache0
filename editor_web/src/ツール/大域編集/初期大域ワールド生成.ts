import type { 大域世界構造 } from '../../生成/編集資源契約.ts'
import { ワールド編集状態 } from '../チャンク編集/編集モデル/index.ts'

// 1024m四方・4x4チャンク・513x513大域ハイトマップの初期編集状態を構築する。
export function 初期大域ワールド状態を生成する(): ワールド編集状態 {
    const 大域構造: 大域世界構造 = {
        区画割り: {
            一辺のメートル: 1024,
            軸あたりチャンク数: 4,
            チャンクあたり格子解像度: 128,
        },
        広域道路: {
            制御点列: [],
            全幅メートル: 12.0,
            細分割数: 120,
        },
    }

    const 解像度 = 513
    const 一辺 = 1024
    const 格子間隔 = 一辺 / (解像度 - 1)
    const 半分 = 一辺 / 2
    const 高さデータ = new Float32Array(解像度 * 解像度)

    for (let gz = 0; gz < 解像度; gz++) {
        for (let gx = 0; gx < 解像度; gx++) {
            const wx = gx * 格子間隔 - 半分
            const wz = gz * 格子間隔 - 半分

            let h = Math.sin(wx * 0.005) * Math.cos(wz * 0.005) * 35.0
            h += Math.sin(wx * 0.012 + 0.5) * Math.sin(wz * 0.012) * 15.0
            h += Math.sin(wx * 0.03) * Math.cos(wz * 0.03) * 4.0

            高さデータ[gz * 解像度 + gx] = h
        }
    }

    const 状態 = new ワールド編集状態(大域構造, 高さデータ)

    const 初期道路点列 = [
        { x: -380, z: -300 },
        { x: -120, z: -50 },
        { x: 150, z: 100 },
        { x: 400, z: 350 },
    ]

    for (const 点 of 初期道路点列) {
        const y = 状態.大域高さ場.標本高さを取得する(点.x, 点.z) + 1.0
        状態.広域道路.点を追加する({ x: 点.x, y, z: 点.z })
    }

    return 状態
}
