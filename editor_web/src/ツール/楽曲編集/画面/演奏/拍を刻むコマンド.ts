import type { 秒, 演奏コマンド, 音声部品 } from 'SengenAudio'
import type { メトロノームの入切 } from '../../編集モデル/index.ts'
import type { 打楽器を鳴らす音源 } from './音源の口.ts'

// メトロノームの1打を、指定された時刻に鳴らす操作。長さは打点の長さでなく音色が決める。
// 鳴らす瞬間に入切を読むのは、予約を積んだ後で人がメトロノームを切ったときにその拍から鳴らさないためである。
// 予約の側を入切で止めると、切ってから先読みの幅のあいだ鳴り続けてしまう。
export class 拍を刻むコマンド implements 演奏コマンド {
    public constructor(
        private readonly _入切: メトロノームの入切,
        private readonly _音源: 打楽器を鳴らす音源,
        private readonly _長さ: 秒,
        private readonly _出力先: 音声部品,
    ) {}

    public 演奏する(開始時刻: 秒): void {
        if (!this._入切.入っているか) return
        this._音源.鳴らす({ 開始時刻, 長さ: this._長さ, 出力先: this._出力先 })
    }
}
