import type { はめ口の値, 升目の宣言 } from '../../../生成/編集資源契約.ts'

// 升目の4つの縦の面。契約の型は面ごとに別の欄を持つため、面を値として扱うためにこの並びが要る。
export const 全側面 = ['正面', '背面', '左面', '右面'] as const

export type 升目の側面 = (typeof 全側面)[number]

export function 側面のはめ口を読む(宣言: 升目の宣言, 側面: 升目の側面): はめ口の値 {
    switch (側面) {
        case '正面':
            return 宣言.正面
        case '背面':
            return 宣言.背面
        case '左面':
            return 宣言.左面
        case '右面':
            return 宣言.右面
    }
}

export function 側面のはめ口を差し替えた宣言(宣言: 升目の宣言, 側面: 升目の側面, 値: はめ口の値): 升目の宣言 {
    switch (側面) {
        case '正面':
            return { ...宣言, 正面: 値 }
        case '背面':
            return { ...宣言, 背面: 値 }
        case '左面':
            return { ...宣言, 左面: 値 }
        case '右面':
            return { ...宣言, 右面: 値 }
    }
}

// 隣の升目から見て、この面と同じ物理面にあたる側面。隣を継ぐ面へ壁を入れない検査に要る。
export function 向かい合う側面(側面: 升目の側面): 升目の側面 {
    switch (側面) {
        case '正面':
            return '背面'
        case '背面':
            return '正面'
        case '左面':
            return '右面'
        case '右面':
            return '左面'
    }
}

// 名指した側面の向こう隣の升目へ進む、横と奥の増分。軸の向きは骨格の側面の宣言が決めている。
export function 側面へ進む増分(側面: 升目の側面): { 横: number; 奥: number } {
    switch (側面) {
        case '正面':
            return { 横: 0, 奥: -1 }
        case '背面':
            return { 横: 0, 奥: 1 }
        case '左面':
            return { 横: -1, 奥: 0 }
        case '右面':
            return { 横: 1, 奥: 0 }
    }
}
