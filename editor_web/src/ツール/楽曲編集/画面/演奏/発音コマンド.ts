import {
    周波数ヘルツ,
    秒,
    音高番号,
    type 演奏コマンド,
    type 音声部品,
} from 'SengenAudio'
import { ステップ, type 拍毎分, type 鳴り始める音 } from '../../編集モデル/index.ts'
import type { 打楽器を鳴らす音源, 音高を鳴らす音源 } from './音源の口.ts'
import type { 音の出口 } from './音の出口.ts'
import type { 楽器の音源棚 } from './楽器の音源棚.ts'

// 音高を持つ音1つを、指定された時刻に鳴らす操作。
export class 音高を鳴らすコマンド implements 演奏コマンド {
    public constructor(
        private readonly _音源: 音高を鳴らす音源,
        private readonly _周波数: 周波数ヘルツ,
        private readonly _長さ: 秒,
        private readonly _出力先: 音声部品,
    ) {}

    public 演奏する(開始時刻: 秒): void {
        this._音源.鳴らす({ 開始時刻, 周波数: this._周波数, 長さ: this._長さ, 出力先: this._出力先 })
    }
}

// 打楽器の1打を、指定された時刻に鳴らす操作。長さは打点の長さでなく音色が決める。
export class 打楽器を鳴らすコマンド implements 演奏コマンド {
    public constructor(
        private readonly _音源: 打楽器を鳴らす音源,
        private readonly _長さ: 秒,
        private readonly _出力先: 音声部品,
    ) {}

    public 演奏する(開始時刻: 秒): void {
        this._音源.鳴らす({ 開始時刻, 長さ: this._長さ, 出力先: this._出力先 })
    }
}

// 鳴り始める音1つを、その楽器の音源とトラックの合流点へ結び付けた演奏コマンドにする。
export function 発音コマンドを組み立てる(
    音: 鳴り始める音,
    現在の拍毎分: 拍毎分,
    音源棚: 楽器の音源棚,
    出口: 音の出口,
): 演奏コマンド {
    const 出力先 = 出口.トラックの合流点(音.トラックの位置)
    switch (音.種類) {
        case '音高の音': {
            const 長さ = 現在の拍毎分.ステップ数の秒数(ステップ.生成する(音.長さのステップ数))
            return new 音高を鳴らすコマンド(
                音源棚.音高の音源を貸す(音.楽器),
                音高番号.生成する(音.音高番号).周波数へ変換する(),
                秒.生成する(長さ.数値()),
                出力先,
            )
        }
        case '打楽器の音': {
            const 打撃 = 音源棚.打楽器の打撃を貸す(音.楽器, 音.打楽器)
            return new 打楽器を鳴らすコマンド(打撃.音源, 打撃.長さ, 出力先)
        }
    }
}
