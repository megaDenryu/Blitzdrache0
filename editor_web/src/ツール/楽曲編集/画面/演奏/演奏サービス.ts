import { テンポ, 演奏の範囲の既定, type 演奏の範囲, type 楽曲編集状態 } from '../../編集モデル/index.ts'
import { 予約の補充を演奏へ委ねる, type I予約を補充できる側 } from './予約の補充を演奏へ委ねる.ts'
import { 単発の発音 } from './単発の発音.ts'
import { 再生位置の知らせ役, type I再生位置を答えられる側 } from './再生位置の知らせ役.ts'
import { 演奏の進行 } from './演奏の進行.ts'
import { 演奏の知らせの口 } from './演奏の知らせの口.ts'
import { 開かれる音声の場 } from './開かれる音声の場.ts'
import type { 再生位置 } from './再生位置.ts'

// 楽曲の再生と、升目を押したときの1音の発音を受け持つ操作サービス。
// 音を出せなかったときは黙って諦めず、演奏の知らせの口から画面へ理由を伝える。
export class 演奏サービス implements I予約を補充できる側, I再生位置を答えられる側 {
    public readonly 知らせ役: 再生位置の知らせ役 = new 再生位置の知らせ役(this)
    public readonly 知らせの口: 演奏の知らせの口 = new 演奏の知らせの口()
    private readonly _場の口: 開かれる音声の場
    private readonly _単発の発音: 単発の発音
    private _進行: 演奏の進行 | null = null
    private _範囲: 演奏の範囲 = 演奏の範囲の既定

    public constructor(private readonly _状態: 楽曲編集状態) {
        this._場の口 = new 開かれる音声の場(_状態, new 予約の補充を演奏へ委ねる(this))
        this._単発の発音 = new 単発の発音(_状態, this._場の口)
    }

    public get 再生中か(): boolean {
        return this._進行 !== null
    }

    public get 演奏の範囲(): 演奏の範囲 {
        return this._範囲
    }

    public 演奏の範囲を変える(範囲: 演奏の範囲): void {
        this._範囲 = 範囲
        this._進行?.範囲を変える(範囲)
    }

    public 再生と停止を切り替える(): void {
        if (this.再生中か) {
            this.停止する()
            return
        }
        this.知らせの口.伝えることを消す()
        void this._再生を試みる()
    }

    public 停止する(): void {
        if (!this.再生中か) return
        this._止まった形へ戻す()
        this.知らせ役.いまの位置を届ける()
    }

    // 先頭へ戻す。再生中なら先頭から鳴らし直し、止まっているなら印を消すだけにする。
    public 先頭へ戻す(): void {
        if (!this.再生中か) {
            this.知らせ役.いまの位置を届ける()
            return
        }
        this.停止する()
        void this._再生を試みる()
    }

    public 升目の音を1回鳴らす(トラックの位置: number, 行の位置: number): void {
        void this._単発の発音
            .升目の音を鳴らす(トラックの位置, 行の位置, this._いまのテンポ())
            .catch((原因: unknown) => this.知らせの口.音を出せなかったことを伝える(原因))
    }

    public 予約を補充する(): void {
        this._進行?.補充する()
    }

    public 現在の再生位置(): 再生位置 | null {
        return this._進行 === null ? null : this._進行.現在の再生位置()
    }

    // テンポ・ミキサー・トラックの音量のどれが変わっても、楽曲の値でまとめて鳴り方へ映す。
    public 楽曲の設定の変更を反映する(): void {
        this._進行?.速さの変化を取り込む()
        this._場の口.開いていれば()?.楽曲の設定を反映する(this._状態.楽曲, this._いまのテンポ())
    }

    public 破棄する(): void {
        this._進行 = null
        this.知らせ役.届けるのをやめる()
        this._場の口.破棄する()
    }

    private async _再生を試みる(): Promise<void> {
        try {
            const 場 = await this._場の口.開く()
            this._進行 = new 演奏の進行(場, this._状態, this._範囲, 場.現在時刻())
            場.予定表.始める()
            this.知らせ役.届け始める()
            this.知らせ役.いまの位置を届ける()
        } catch (原因: unknown) {
            this._音を出せなかった(原因)
        }
    }

    // 途中まで進んだ再生を必ず止まった形へ戻す。ボタンが「停止」のまま音が出ない状態を作らないためである。
    private _音を出せなかった(原因: unknown): void {
        this._止まった形へ戻す()
        this.知らせ役.いまの位置を届ける()
        this.知らせの口.音を出せなかったことを伝える(原因)
    }

    private _止まった形へ戻す(): void {
        this._進行 = null
        this.知らせ役.届けるのをやめる()
        const 場 = this._場の口.開いていれば()
        if (場 !== null && 場.予定表.動作中か) 場.予定表.やめる()
    }

    private _いまのテンポ(): テンポ {
        return this._進行?.現在のテンポ() ?? テンポ.生成する(this._状態.楽曲.テンポ)
    }
}
