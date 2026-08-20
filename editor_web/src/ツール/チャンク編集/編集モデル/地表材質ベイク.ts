import type { チャンクの道路 } from '../../../生成/編集資源契約.ts'
import type { 高さ場 } from './高さ場.ts'
import { 合計255へ正規化する } from './地表材質正規化.ts'
import { CatmullRomCurve3, Vector3 } from 'three'

// 地表材質の急勾配ベイク・道路下ベイクの計算工程。地表材質が材質データ・解像度・一辺のメートル・
// 格子間隔を保持した上で自身のメソッド(急勾配を岩肌へベイクする・道路下を泥へベイクする)から呼ぶ
// モジュール私有ヘルパーであり、地表材質以外から呼んではならない。単独では公開しない
// (編集モデル/index.tsのバレルへは載せない)。

// 勾配角が30度を超える崖面を岩肌テクスチャとして自動ベイクする。
export function 急勾配を岩肌へベイクする処理(
    材質データ: Uint8Array,
    解像度: number,
    一辺のメートル: number,
    格子間隔: number,
    対象高さ場: 高さ場,
): void {
    for (let gz = 0; gz < 解像度; gz++) {
        for (let gx = 0; gx < 解像度; gx++) {
            const wx = gx * 格子間隔 - 一辺のメートル / 2
            const wz = gz * 格子間隔 - 一辺のメートル / 2
            const 法線 = 対象高さ場.標本法線を取得する(wx, wz)
            const コサイン角 = Math.max(-1, Math.min(1, 法線.y))
            const 勾配角度 = Math.acos(コサイン角) * (180 / Math.PI)
            if (勾配角度 > 30) {
                const 岩の重み比率 = Math.min(1.0, (勾配角度 - 30) / 25)
                const 画素先頭 = (gz * 解像度 + gx) * 4
                const 現在草 = (材質データ[画素先頭 + 0] ?? 0) * (1.0 - 岩の重み比率)
                const 現在泥 = (材質データ[画素先頭 + 1] ?? 0) * (1.0 - 岩の重み比率)
                const 現在砂 = (材質データ[画素先頭 + 3] ?? 0) * (1.0 - 岩の重み比率)
                const 新岩 = Math.floor(岩の重み比率 * 255)
                const [草, 泥, 岩, 砂] = 合計255へ正規化する(現在草, 現在泥, 新岩, 現在砂)
                材質データ[画素先頭 + 0] = 草
                材質データ[画素先頭 + 1] = 泥
                材質データ[画素先頭 + 2] = 岩
                材質データ[画素先頭 + 3] = 砂
            }
        }
    }
}

// 標高がしきい値を下回るほど泥の重みを穏やかに増やす。全面置換の道路下ベイクと違い、
// 既存の重みへ比率で混ぜ込むだけに留める(崖の底が低地であっても、この後に重ねる
// 急勾配ベイクが岩肌へ上書きできる余地を残すため)。
export function 低地を泥へベイクする処理(
    材質データ: Uint8Array,
    解像度: number,
    一辺のメートル: number,
    格子間隔: number,
    対象高さ場: 高さ場,
): void {
    const しきい値メートル = -2.5
    const 移行幅メートル = 5.0
    const 泥重み上限比率 = 0.55
    for (let gz = 0; gz < 解像度; gz++) {
        for (let gx = 0; gx < 解像度; gx++) {
            const wx = gx * 格子間隔 - 一辺のメートル / 2
            const wz = gz * 格子間隔 - 一辺のメートル / 2
            const 標高 = 対象高さ場.標本高さを取得する(wx, wz)
            const 泥重み比率 = Math.min(泥重み上限比率, Math.max(0, (しきい値メートル - 標高) / 移行幅メートル))
            if (泥重み比率 > 0) {
                const 画素先頭 = (gz * 解像度 + gx) * 4
                const 現在草 = (材質データ[画素先頭 + 0] ?? 0) * (1.0 - 泥重み比率)
                const 追加泥 = Math.floor(泥重み比率 * 255)
                const 新泥 = (材質データ[画素先頭 + 1] ?? 0) + 追加泥
                const 現在岩 = 材質データ[画素先頭 + 2] ?? 0
                const 現在砂 = 材質データ[画素先頭 + 3] ?? 0
                const [草, 泥, 岩, 砂] = 合計255へ正規化する(現在草, 新泥, 現在岩, 現在砂)
                材質データ[画素先頭 + 0] = 草
                材質データ[画素先頭 + 1] = 泥
                材質データ[画素先頭 + 2] = 岩
                材質データ[画素先頭 + 3] = 砂
            }
        }
    }
}

// 道路メッシュの直下領域（半幅+2m以内）を泥100%として自動ベイクする。
export function 道路下を泥へベイクする処理(
    材質データ: Uint8Array,
    解像度: number,
    一辺のメートル: number,
    格子間隔: number,
    道路: チャンクの道路,
): void {
    if (道路.制御点列.length < 2) return
    const 頂点列 = 道路.制御点列.map((p) => new Vector3(p.x, p.y, p.z))
    const 曲線 = new CatmullRomCurve3(頂点列, false, 'centripetal')
    const 標本点列 = 曲線.getSpacedPoints(道路.細分割数 * 3)
    const 泥半幅 = 道路.全幅メートル * 0.5 + 2.0
    for (let gz = 0; gz < 解像度; gz++) {
        for (let gx = 0; gx < 解像度; gx++) {
            const wx = gx * 格子間隔 - 一辺のメートル / 2
            const wz = gz * 格子間隔 - 一辺のメートル / 2
            let 最小距離 = Number.POSITIVE_INFINITY
            for (const p of 標本点列) {
                const 距離 = Math.hypot(wx - p.x, wz - p.z)
                if (距離 < 最小距離) 最小距離 = 距離
            }
            if (最小距離 <= 泥半幅) {
                const 画素先頭 = (gz * 解像度 + gx) * 4
                材質データ[画素先頭 + 0] = 0
                材質データ[画素先頭 + 1] = 255
                材質データ[画素先頭 + 2] = 0
                材質データ[画素先頭 + 3] = 0
            }
        }
    }
}
