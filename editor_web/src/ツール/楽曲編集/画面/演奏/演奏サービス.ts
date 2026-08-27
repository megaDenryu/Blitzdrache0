import { 配線ポート } from 'sengen-ui'
import {
    升目の音を組み立てる,
    拍毎分,
    演奏の範囲の既定,
    type 演奏の範囲,
    type 楽曲編集状態,
    type 鳴り始める音,
} from '../../編集モデル/index.ts'
import { 毎フレームの見張り, type I画面の1コマごとに起こされる側 } from './毎フレームの見張り.ts'
import { 予約の補充を演奏へ委ねる, type I予約を補充できる側 } from './予約の補充を演奏へ委ねる.ts'
import { 発音コマンドを組み立てる } from './発音コマンド.ts'
import { 演奏の進行 } from './演奏の進行.ts'
import { 開かれる音声の場 } from './開かれる音声の場.ts'
import type { I再生位置の届け先, 再生位置 } from './再生位置.ts'

// 楽曲の再生と、升目を押したときの1音の発音を受け持つ操作サービス。
export class 演奏サービス implements I予約を補充できる側, I画面の1コマごとに起こされる側 {
    public readonly on再生位置が変わった: 配線ポート<I再生位置の届け先> =
        new 配線ポート<I再生位置の届け先>('演奏サービス')
    private readonly _見張り: 毎フレームの見張り = new 毎フレームの見張り(this)
    private readonly _場の口: 開かれる音声の場
    private _進行: 演奏の進行 | null = null
    private _範囲: 演奏の範囲 = 演奏の範囲の既定

    public constructor(private readonly _状態: 楽曲編集状態) {
        this._場の口 = new 開かれる音声の場(_状態, new 予約の補充を演奏へ委ねる(this))
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
        if (this.再生中か) this.停止する()
        else void this._再生を始める()
    }

    public 停止する(): void {
        if (!this.再生中か) return
        this._進行 = null
        this._見張り.やめる()
        const 場 = this._場の口.開いていれば()
        if (場 !== null && 場.予定表.動作中か) 場.予定表.やめる()
        this._再生位置を知らせる()
    }

    // 先頭へ戻す。再生中なら先頭から鳴らし直し、止まっているなら印を消すだけにする。
    public 先頭へ戻す(): void {
        if (!this.再生中か) {
            this._再生位置を知らせる()
            return
        }
        this.停止する()
        void this._再生を始める()
    }

    public 升目の音を1回鳴らす(トラックの位置: number, 行の位置: number): void {
        void this._1音を鳴らす(升目の音を組み立てる(this._状態.楽曲, トラックの位置, 行の位置))
    }

    public 予約を補充する(): void {
        this._進行?.補充する()
    }

    public 画面が1コマ進んだ(): void {
        this._再生位置を知らせる()
    }

    public 現在の再生位置(): 再生位置 | null {
        return this._進行 === null ? null : this._進行.現在の再生位置()
    }

    // 拍毎分・ミキサー・トラックの音量のどれが変わっても、楽曲の値でまとめて鳴り方へ映す。
    public 楽曲の設定の変更を反映する(): void {
        this._進行?.速さの変化を取り込む()
        this._場の口.開いていれば()?.楽曲の設定を反映する(this._状態.楽曲, this._いまの拍毎分())
    }

    public 破棄する(): void {
        this._進行 = null
        this._見張り.やめる()
        this._場の口.破棄する()
    }

    private async _再生を始める(): Promise<void> {
        if (this.再生中か) return
        const 場 = await this._場の口.開く()
        this._進行 = new 演奏の進行(場, this._状態, this._範囲, 場.現在時刻())
        場.予定表.始める()
        this._見張り.始める()
        this._再生位置を知らせる()
    }

    private async _1音を鳴らす(音: 鳴り始める音): Promise<void> {
        const 場 = await this._場の口.開く()
        発音コマンドを組み立てる(音, this._いまの拍毎分(), 場.音源棚, 場.出口).演奏する(
            場.音声の時計の秒へ写す(場.現在時刻()),
        )
    }

    private _いまの拍毎分(): 拍毎分 {
        return this._進行?.現在の拍毎分() ?? 拍毎分.生成する(this._状態.楽曲.拍毎分)
    }

    private _再生位置を知らせる(): void {
        if (!this.on再生位置が変わった.配線済みか) return
        this.on再生位置が変わった.先.再生位置が変わった(this.現在の再生位置())
    }
}
