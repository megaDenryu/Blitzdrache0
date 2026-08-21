import type { マテリアル台帳, マテリアル定義, 層割当 } from '../../../生成/編集資源契約.ts'
import { 配列か, オブジェクトか, 文字列か } from './オブジェクト判定.ts'

// 受信したマテリアル台帳のJSONが型契約に適合しているかを実行時に検査する。
export function マテリアル台帳の形か(値: unknown): 値 is マテリアル台帳 {
    if (!オブジェクトか(値)) return false
    if (!配列か(値['マテリアル一覧'])) return false
    if (!値['マテリアル一覧'].every(マテリアル定義の形か)) return false
    return 層割当の形か(値['層割当'])
}

export function マテリアル定義の形か(値: unknown): 値 is マテリアル定義 {
    if (!オブジェクトか(値)) return false
    return 文字列か(値['エンジン材質名']) && 文字列か(値['識別色'])
}

export function 層割当の形か(値: unknown): 値 is 層割当 {
    if (!オブジェクトか(値)) return false
    return (
        文字列か(値['草']) &&
        文字列か(値['泥']) &&
        文字列か(値['岩']) &&
        文字列か(値['砂'])
    )
}
