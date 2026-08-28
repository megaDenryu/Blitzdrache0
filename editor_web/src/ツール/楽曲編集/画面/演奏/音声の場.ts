import {
    ブラウザの音声の作業場,
    タイマーの起床源,
    ミリ秒,
    演奏予定表,
    秒,
} from 'SengenAudio'
import type { 楽曲 } from '../../../../生成/編集資源契約.ts'
import { 秒数, type メトロノームの入切, type テンポ } from '../../編集モデル/index.ts'
import { メトロノームの音 } from './メトロノームの音.ts'
import { 楽器の音源棚 } from './楽器の音源棚.ts'
import { 音の出口 } from './音の出口.ts'

// 音声の時計の現在時刻からこの先までに来る演奏を、時刻を指定して前もって予約する。
const 先読みの幅 = 秒.生成する(0.12)
const 見張りの周期 = ミリ秒.生成する(25)
const 予約補充のしきい値 = 8

// ブラウザが音を出せる状態になってから初めて作れる、演奏の一式を所有する。
// ブラウザは人の操作を起点にしないと音を出さないため、作るのは非同期の工程に限る。
export class 音声の場 {
    private constructor(
        private readonly _作業場: ブラウザの音声の作業場,
        public readonly 出口: 音の出口,
        public readonly 音源棚: 楽器の音源棚,
        public readonly 予定表: 演奏予定表,
        public readonly メトロノームの音: メトロノームの音,
    ) {}

    public static async 人の操作を起点に開く(楽曲: 楽曲, メトロノームの入切: メトロノームの入切): Promise<音声の場> {
        const 作業場 = ブラウザの音声の作業場.作成する()
        await 作業場.再開する()
        const 予定表 = new 演奏予定表({
            作業場,
            起床源: new タイマーの起床源(),
            先読みの幅,
            見張りの周期,
            予約補充のしきい値,
        })
        return new 音声の場(
            作業場,
            new 音の出口(作業場, 楽曲),
            new 楽器の音源棚(作業場),
            予定表,
            new メトロノームの音(作業場, 予定表, メトロノームの入切),
        )
    }

    public 現在時刻(): 秒数 {
        return 秒数.生成する(this._作業場.現在時刻().数値())
    }

    public 音声の時計の秒へ写す(時刻: 秒数): 秒 {
        return 秒.生成する(時刻.数値())
    }

    // ミキサーとトラックの音量は人が動かすたびに変わるため、楽曲の側の値でまとめて上書きする。
    public 楽曲の設定を反映する(楽曲: 楽曲, 現在のテンポ: テンポ): void {
        this.出口.ミキサー設定を反映する(楽曲.ミキサー設定, 現在のテンポ)
        for (const [位置, トラック] of 楽曲.トラック構成.entries()) {
            if (位置 < this.出口.トラックの本数()) this.出口.トラックの音量を反映する(位置, トラック.音量)
        }
    }

    public async 眠っていたら起こす(): Promise<void> {
        if (this._作業場.状態() !== '動作中') await this._作業場.再開する()
    }

    public 破棄する(): void {
        if (this.予定表.動作中か) this.予定表.やめる()
        this.音源棚.破棄する()
        this.メトロノームの音.破棄する()
        this.出口.破棄する()
        void this._作業場.破棄する()
    }
}
