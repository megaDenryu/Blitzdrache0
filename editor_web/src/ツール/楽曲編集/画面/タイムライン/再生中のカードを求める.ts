import { ステップ, 通しステップからカードを求める, type カード位置, type 曲構成のカード, type 演奏の範囲 } from '../../編集モデル/index.ts'
import type { 再生位置 } from '../演奏/index.ts'

// 曲構成のとおりに鳴らしているときだけ、いま鳴っているカードの位置を求める。
// パターンを繰り返しているときは曲構成の外を鳴らしているため、印を出すカードは無い(null)。
export function 再生中のカードを求める(
    カード列: readonly 曲構成のカード[],
    位置: 再生位置 | null,
    範囲: 演奏の範囲,
): カード位置 | null {
    if (範囲 !== '曲構成のとおり' || 位置 === null || カード列.length === 0) return null
    const カードの位置情報 = 通しステップからカードを求める(カード列, ステップ.生成する(位置.通しステップ))
    if (カードの位置情報 === null) return null
    const カード = カード列[カードの位置情報.カードの添字]
    return カード === undefined ? null : カード.位置
}
