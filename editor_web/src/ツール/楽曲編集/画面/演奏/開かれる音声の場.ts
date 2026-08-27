import type { 予約補充コマンド } from 'SengenAudio'
import type { 楽曲編集状態 } from '../../編集モデル/index.ts'
import { 音声の場 } from './音声の場.ts'

// 音声の場を開くまで持ち続け、開いた後は同じ場を貸し続ける口。
// ブラウザは人の操作を起点にしないと音を出さないため、開くのは最初の再生か最初の発音まで待たされる。
// その「まだ無いかもしれない」を1箇所へ閉じ、演奏サービスの側から場の不在を消す。
export class 開かれる音声の場 {
    private _場: 音声の場 | null = null

    public constructor(
        private readonly _状態: 楽曲編集状態,
        private readonly _予約の補充: 予約補充コマンド,
    ) {}

    // まだ開いていなければ開く。開いていて眠っていれば起こす。
    public async 開く(): Promise<音声の場> {
        if (this._場 !== null) {
            await this._場.眠っていたら起こす()
            return this._場
        }
        const 場 = await 音声の場.人の操作を起点に開く(this._状態.楽曲)
        場.予定表.配線する(this._予約の補充)
        this._場 = 場
        return 場
    }

    // 既に開いている場だけを返す。開いていなければ何もしない側が使う。
    public 開いていれば(): 音声の場 | null {
        return this._場
    }

    public 破棄する(): void {
        this._場?.破棄する()
        this._場 = null
    }
}
