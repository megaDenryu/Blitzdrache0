import type { 地表材質層, 材質の筆致 } from '../../../生成/編集資源契約.ts'
import { 合計255へ正規化する } from './地表材質正規化.ts'

// 材質の筆致を通過点列に沿って塗る計算工程。地表材質が材質データ・解像度・一辺のメートル・
// 格子間隔を保持した上で自身のメソッド(材質筆致を適用する)から呼ぶモジュール私有ヘルパーであり、
// 地表材質以外から呼んではならない。単独では公開しない(編集モデル/index.tsのバレルへは載せない)。

function 層を添字に変換する(層: 地表材質層): number {
    switch (層) {
        case '草': return 0
        case '泥': return 1
        case '岩': return 2
        case '砂': return 3
    }
    const 未知の層: never = 層
    throw new Error(`未知の地表材質層: ${JSON.stringify(未知の層)}`)
}

export function 材質の筆致を塗る処理(
    材質データ: Uint8Array,
    解像度: number,
    一辺のメートル: number,
    格子間隔: number,
    筆致: 材質の筆致,
): void {
    const 層添字 = 層を添字に変換する(筆致.層)
    for (const 通過点 of 筆致.通過点列) {
        for (let gz = 0; gz < 解像度; gz++) {
            for (let gx = 0; gx < 解像度; gx++) {
                const wx = gx * 格子間隔 - 一辺のメートル / 2
                const wz = gz * 格子間隔 - 一辺のメートル / 2
                const 距離 = Math.hypot(wx - 通過点.x, wz - 通過点.z)
                if (距離 < 筆致.半径メートル) {
                    const コサイン減衰 = Math.cos((距離 / 筆致.半径メートル) * (Math.PI / 2)) * 筆致.流量
                    const 画素先頭 = (gz * 解像度 + gx) * 4
                    const 現在草 = 材質データ[画素先頭 + 0] ?? 0
                    const 現在泥 = 材質データ[画素先頭 + 1] ?? 0
                    const 現在岩 = 材質データ[画素先頭 + 2] ?? 0
                    const 現在砂 = 材質データ[画素先頭 + 3] ?? 0
                    const 加算値 = Math.floor(コサイン減衰 * 255)
                    const 新草 = 層添字 === 0 ? Math.min(255, 現在草 + 加算値) : 現在草
                    const 新泥 = 層添字 === 1 ? Math.min(255, 現在泥 + 加算値) : 現在泥
                    const 新岩 = 層添字 === 2 ? Math.min(255, 現在岩 + 加算値) : 現在岩
                    const 新砂 = 層添字 === 3 ? Math.min(255, 現在砂 + 加算値) : 現在砂
                    const [正規化草, 正規化泥, 正規化岩, 正規化砂] = 合計255へ正規化する(新草, 新泥, 新岩, 新砂)
                    材質データ[画素先頭 + 0] = 正規化草
                    材質データ[画素先頭 + 1] = 正規化泥
                    材質データ[画素先頭 + 2] = 正規化岩
                    材質データ[画素先頭 + 3] = 正規化砂
                }
            }
        }
    }
}
