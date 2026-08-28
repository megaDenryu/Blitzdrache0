import { 升目の音を組み立てる, type テンポ, type 楽曲編集状態 } from '../../編集モデル/index.ts'
import { 発音コマンドを組み立てる } from './発音コマンド.ts'
import type { 開かれる音声の場 } from './開かれる音声の場.ts'

// 升目を押したときのように、予定表を通さずその場で1音だけ鳴らす工程。
// 予約の列とは進み方が違う(いま鳴らして終わり)ため、演奏の進行から分けて持つ。
export class 単発の発音 {
    public constructor(
        private readonly _状態: 楽曲編集状態,
        private readonly _場の口: 開かれる音声の場,
    ) {}

    public async 升目の音を鳴らす(
        トラックの位置: number,
        行の位置: number,
        現在のテンポ: テンポ,
    ): Promise<void> {
        const 音 = 升目の音を組み立てる(this._状態.楽曲, トラックの位置, 行の位置)
        const 場 = await this._場の口.開く()
        発音コマンドを組み立てる(音, 現在のテンポ, 場.音源棚, 場.出口).演奏する(
            場.音声の時計の秒へ写す(場.現在時刻()),
        )
    }
}
