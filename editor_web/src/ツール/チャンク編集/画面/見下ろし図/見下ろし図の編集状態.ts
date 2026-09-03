import type { 地表材質層 } from '../../../../生成/編集資源契約.ts'
import { 大升の一辺の升目数の既定 } from '../../編集モデル/index.ts'

// 高さ場から等高線を導く間隔の既定(設計正本の判断5)。
export const 等高線を導く間隔の既定メートル = 2

// 見下ろし図の上での編集の選択と、右サイドバーの2パネルが持つ設定。編集モデルではなく画面の状態であり、
// 保存しない。描いている途中の等高線はポインタ操作係が持つ(離した瞬間にだけ意味を持つため)。
// 参照: `_doc/設計/見下ろし図による地形編集.md`「判断7」
export class 見下ろし図の編集状態 {
    public 選択中の等高線の添字: number | null = null
    public 新しい等高線の高さメートル: number = 0
    public 等高線を導く間隔メートル: number = 等高線を導く間隔の既定メートル
    public 大升の一辺の升目数: number = 大升の一辺の升目数の既定
    public 大升に置く高さメートル: number = 0
    public 大升に高さを置くか: boolean = true
    public 大升に置く層: 地表材質層 = '草'
    public 大升に層を置くか: boolean = true
    public 大升の塗りを消すか: boolean = false
}
