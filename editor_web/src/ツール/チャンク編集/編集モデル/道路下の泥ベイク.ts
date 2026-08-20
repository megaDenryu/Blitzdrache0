import type { チャンクの道路 } from '../../../生成/編集資源契約.ts'
import { CatmullRomCurve3, Vector3 } from 'three'

// 道路メッシュの直下領域(半幅+2m以内)を泥100%として自動ベイクする計算工程。分岐した道も
// 泥になるよう、チャンクの全ての道路をまとめて受け取り、どれか1本でも近ければ泥に塗る。
// 地表材質が材質データ・解像度・一辺のメートル・格子間隔を保持した上で自身のメソッド
// (道路下を泥へベイクする)から呼ぶモジュール私有ヘルパーであり、地表材質以外から呼んでは
// ならない。単独では公開しない(編集モデル/index.tsのバレルへは載せない)。

// 道路1本ぶんの、泥に塗る範囲を表す標本点列と半幅の組。
interface 泥に塗る帯 {
    readonly 標本点列: ReadonlyArray<Vector3>
    readonly 泥半幅: number
}

function 泥に塗る帯を作る(道路: チャンクの道路): 泥に塗る帯 | null {
    if (道路.制御点列.length < 2) return null
    const 頂点列 = 道路.制御点列.map((p) => new Vector3(p.x, p.y, p.z))
    const 曲線 = new CatmullRomCurve3(頂点列, false, 'centripetal')
    return {
        標本点列: 曲線.getSpacedPoints(道路.細分割数 * 3),
        泥半幅: 道路.全幅メートル * 0.5 + 2.0,
    }
}

function 帯のどれかの下にあるか(帯一覧: ReadonlyArray<泥に塗る帯>, wx: number, wz: number): boolean {
    for (const 帯 of 帯一覧) {
        for (const p of 帯.標本点列) {
            if (Math.hypot(wx - p.x, wz - p.z) <= 帯.泥半幅) return true
        }
    }
    return false
}

export function 道路下を泥へベイクする処理(
    材質データ: Uint8Array,
    解像度: number,
    一辺のメートル: number,
    格子間隔: number,
    道路一覧: ReadonlyArray<チャンクの道路>,
): void {
    const 帯一覧 = 道路一覧.map(泥に塗る帯を作る).filter((帯): 帯 is 泥に塗る帯 => 帯 !== null)
    if (帯一覧.length === 0) return
    for (let gz = 0; gz < 解像度; gz++) {
        for (let gx = 0; gx < 解像度; gx++) {
            const wx = gx * 格子間隔 - 一辺のメートル / 2
            const wz = gz * 格子間隔 - 一辺のメートル / 2
            if (帯のどれかの下にあるか(帯一覧, wx, wz)) {
                const 画素先頭 = (gz * 解像度 + gx) * 4
                材質データ[画素先頭 + 0] = 0
                材質データ[画素先頭 + 1] = 255
                材質データ[画素先頭 + 2] = 0
                材質データ[画素先頭 + 3] = 0
            }
        }
    }
}
