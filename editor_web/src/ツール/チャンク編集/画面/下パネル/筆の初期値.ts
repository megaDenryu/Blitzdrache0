import type { 造成筆致種別, 地表材質層 } from '../../../../生成/編集資源契約.ts'

// 棚に出る筆のつまみの初期の位置。編集状態もこの値から始めることで、人が見ているつまみの位置と
// 実際に筆が使う値が最初から一致する(エディター制作スキル第1条・第2条)。
export const 造成の筆の初期値: {
    readonly 種別: 造成筆致種別
    readonly 半径メートル: number
    readonly 強さ: number
} = { 種別: '盛る', 半径メートル: 20.0, 強さ: 0.5 }

export const 地表の材質の筆の初期値: {
    readonly 層: 地表材質層
    readonly 半径メートル: number
    readonly 流量: number
} = { 層: '草', 半径メートル: 15.0, 流量: 0.4 }
