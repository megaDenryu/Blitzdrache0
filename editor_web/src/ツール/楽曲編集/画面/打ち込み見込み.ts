import type { 打ち込みの対象 } from '../../../生成/編集資源契約.ts'

export interface 升目の当たりの記録 {
    readonly パターンの名乗り: string
    readonly トラックの位置: number
    readonly 行の位置: number
    readonly ステップ: number
}

// ドラッグ中に画面が見込みとして表示する打ち込み内容。
export type 打ち込みドラッグ見込み =
    | {
          readonly 種類: '音を伸ばす'
          readonly 対象: 打ち込みの対象
          readonly 始まりのステップ: number
          readonly 終わりのステップ: number
          readonly 進行に従うか: boolean
      }
    | {
          readonly 種類: '打点を置く'
          readonly 対象: 打ち込みの対象
          readonly ステップ: number
          readonly 進行に従うか: boolean
      }
    | {
          readonly 種類: '範囲の打点を消す'
          readonly 対象: 打ち込みの対象
          readonly 始まりのステップ: number
          readonly 終わりのステップ: number
      }
    | {
          readonly 種類: '打点を消す'
          readonly 対象: 打ち込みの対象
          readonly ステップ: number
      }
