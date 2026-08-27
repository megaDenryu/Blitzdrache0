import { 濾波の種類, 発振の波形 } from 'SengenAudio'
import type { 波の形, 通す帯域 } from '../../編集モデル/index.ts'

// 編集モデルが持つ日本語の綴りを、SengenAudioの値オブジェクトへ写す。写しはこの2つの工程だけが持つ。
export function 波の形を発振の波形へ写す(形: 波の形): 発振の波形 {
    switch (形) {
        case '正弦':
            return 発振の波形.正弦()
        case '矩形':
            return 発振の波形.矩形()
        case '三角':
            return 発振の波形.三角()
        case '鋸':
            return 発振の波形.鋸()
    }
}

export function 通す帯域を濾波の種類へ写す(帯域: 通す帯域): 濾波の種類 {
    switch (帯域) {
        case '低音通過':
            return 濾波の種類.低音通過()
        case '高音通過':
            return 濾波の種類.高音通過()
        case '帯域通過':
            return 濾波の種類.帯域通過()
    }
}
