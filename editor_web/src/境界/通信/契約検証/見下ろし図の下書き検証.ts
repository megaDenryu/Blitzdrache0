import type { 見下ろし図の下書き, 等高線, 大升の塗り, 平面の位置, 地表材質層 } from '../../../生成/編集資源契約.ts'
import { 配列か, 数値か, オブジェクトか, 文字列か } from './オブジェクト判定.ts'

// 受信したチャンク構造の中の見下ろし図の下書きが型契約に適合しているかを実行時に検査する。
// 編集サーバーは下書きを持たない旧版を空の下書きへ移行してから返すため、欠けたJSONは契約違反として拒む。
export function 見下ろし図の下書きの形か(値: unknown): 値 is 見下ろし図の下書き {
    if (!オブジェクトか(値)) return false
    if (!配列か(値['等高線一覧']) || !値['等高線一覧'].every(等高線の形か)) return false
    if (!数値か(値['大升の一辺の升目数'])) return false
    return 配列か(値['大升の塗り一覧']) && 値['大升の塗り一覧'].every(大升の塗りの形か)
}

export function 等高線の形か(値: unknown): 値 is 等高線 {
    if (!オブジェクトか(値)) return false
    if (!配列か(値['頂点列']) || !値['頂点列'].every(平面の位置の形か)) return false
    return 数値か(値['高さメートル']) && typeof 値['閉じている'] === 'boolean'
}

export function 平面の位置の形か(値: unknown): 値 is 平面の位置 {
    if (!オブジェクトか(値)) return false
    return 数値か(値['x']) && 数値か(値['z'])
}

export function 大升の塗りの形か(値: unknown): 値 is 大升の塗り {
    if (!オブジェクトか(値)) return false
    if (!数値か(値['列']) || !数値か(値['行'])) return false
    const 高さ = 値['高さメートル']
    const 層 = 値['層']
    return (高さ === null || 数値か(高さ)) && (層 === null || 地表材質層の形か(層))
}

const 地表材質層一覧: readonly 地表材質層[] = ['草', '泥', '岩', '砂']

function 地表材質層の形か(値: unknown): 値 is 地表材質層 {
    return 文字列か(値) && 地表材質層一覧.some((層) => 層 === 値)
}
