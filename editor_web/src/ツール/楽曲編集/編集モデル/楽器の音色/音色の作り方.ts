import type { 打楽器の種類 } from '../../../../生成/編集資源契約.ts'

// 発振器が生む波の形。SengenAudioの発振の波形へ写す綴りであり、この層は生の英語名を持たない。
export type 波の形 = '正弦' | '矩形' | '三角' | '鋸'

// 濾波器が通す帯域。SengenAudioの濾波の種類へ写す綴りである。
export type 通す帯域 = '低音通過' | '高音通過' | '帯域通過'

// 加算合成で重ねる1つの倍音。比は基本の周波数に対する倍率である。
export interface 倍音の設定 {
    readonly 比: number
    readonly 利得: number
    readonly 音を止めるまでの秒数: number
}

export interface 加算合成の作り方 {
    readonly 種類: '加算合成'
    readonly 倍音一覧: readonly 倍音の設定[]
}

export interface 減算合成の作り方 {
    readonly 種類: '減算合成'
    readonly 波の形: 波の形
    readonly 濾波の始まりの遮断周波数: number
    readonly 濾波の終わりの遮断周波数: number
    readonly 立ち上がりの秒数: number
    readonly 保持の秒数: number
    readonly 減衰の秒数: number
}

export interface 撥弦合成の作り方 {
    readonly 種類: '撥弦合成'
    readonly 減衰の強さ: number
    readonly 胴の濾波の開始遮断周波数: number
    readonly 胴の濾波の終了遮断周波数: number
}

export interface 周波数変調合成の作り方 {
    readonly 種類: '周波数変調合成'
    readonly 変調波の周波数比: number
    readonly 変調の深さ: number
    readonly 深さの減衰の秒数: number
}

// 音高を持つ音を作るための合成技法と、その技法へ与える値。
export type 音色の作り方 =
    | 加算合成の作り方
    | 減算合成の作り方
    | 撥弦合成の作り方
    | 周波数変調合成の作り方

// 打楽器1つ分の音の作り方。雑音から作るものと、決まった低い音高から作るものがある。
export type 打楽器の音色の作り方 =
    | {
          readonly 種類: '雑音の打撃'
          readonly 通す帯域: 通す帯域
          readonly 中心の周波数: number
          readonly 鋭さ: number
          readonly 長さの秒数: number
          readonly 減衰の秒数: number
      }
    | {
          readonly 種類: '決まった音高の打撃'
          readonly 周波数ヘルツ: number
          readonly 長さの秒数: number
          readonly 作り方: 減算合成の作り方
      }

// 楽器1つが鳴らすものの定義。旋律の楽器は音高を鳴らし、打楽器の楽器は種類ごとの打撃を鳴らす。
export type 楽器の音色 =
    | { readonly 種類: '音高を鳴らす'; readonly 作り方: 音色の作り方 }
    | {
          readonly 種類: '打楽器を鳴らす'
          readonly 打楽器ごとの作り方: Readonly<Record<打楽器の種類, 打楽器の音色の作り方>>
      }
