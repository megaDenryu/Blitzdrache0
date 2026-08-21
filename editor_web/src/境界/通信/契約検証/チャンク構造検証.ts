import type {
    チャンク構造,
    チャンクの道路,
    建物の配置,
    散布の個体,
    散布の設定,
} from '../../../生成/編集資源契約.ts'
import { 配列か, 長整数か, 数値か, オブジェクトか, 文字列か } from './オブジェクト判定.ts'
import { 位置3次元の形か } from './大域世界構造検証.ts'

// 受信したチャンク構造のJSONが型契約に適合しているかを実行時に検査する。
export function チャンク構造の形か(値: unknown): 値 is チャンク構造 {
    if (!オブジェクトか(値)) return false
    if (!配列か(値['道路一覧'])) return false
    if (!値['道路一覧'].every(チャンクの道路の形か)) return false
    if (!建物一覧の形か(値['建物一覧']) || !散布の設定の形か(値['散布'])) return false
    return 散布の個体一覧の形か(値['散布の個体一覧'])
}

export function チャンクの道路の形か(値: unknown): 値 is チャンクの道路 {
    if (!オブジェクトか(値)) return false
    if (!配列か(値['制御点列'])) return false
    if (!値['制御点列'].every(位置3次元の形か)) return false
    return (
        数値か(値['全幅メートル']) &&
        数値か(値['散布除外バッファメートル']) &&
        数値か(値['細分割数'])
    )
}

export function 建物一覧の形か(値: unknown): 値 is Array<建物の配置> {
    if (!配列か(値)) return false
    return 値.every(建物の配置の形か)
}

export function 建物の配置の形か(値: unknown): 値 is 建物の配置 {
    if (!オブジェクトか(値)) return false
    return (
        文字列か(値['識別子']) &&
        文字列か(値['建物定義ID']) &&
        位置3次元の形か(値['位置']) &&
        数値か(値['向きラジアン']) &&
        数値か(値['基礎半径メートル']) &&
        数値か(値['なじみ半径メートル'])
    )
}

export function 散布の設定の形か(値: unknown): 値 is 散布の設定 {
    if (!オブジェクトか(値)) return false
    return 数値か(値['最小間隔メートル']) && 長整数か(値['乱数の種'])
}

export function 散布の個体一覧の形か(値: unknown): 値 is Array<散布の個体> {
    if (!配列か(値)) return false
    return 値.every(散布の個体の形か)
}

export function 散布の個体の形か(値: unknown): 値 is 散布の個体 {
    if (!オブジェクトか(値)) return false
    return (
        文字列か(値['安定識別子']) &&
        数値か(値['チャンク中心からの東メートル']) &&
        数値か(値['チャンク中心からの南メートル'])
    )
}

