import { 秒, type I音声の作業場 } from 'SengenAudio'
import type { 打楽器の種類, 楽器 } from '../../../../生成/編集資源契約.ts'
import { 楽器の音色を求める } from '../../編集モデル/index.ts'
import { 打楽器の打撃の長さ, 打楽器の音源を組み立てる } from './打楽器の音源を組み立てる.ts'
import { 音高の音源を組み立てる } from './音高の音源を組み立てる.ts'
import type { 打楽器を鳴らす音源, 音高を鳴らす音源 } from './音源の口.ts'

// 打楽器の1打分の長さは音色が決めるため、鳴らす側へ音源と一緒に渡す。
export interface 打楽器の打撃 {
    readonly 音源: 打楽器を鳴らす音源
    readonly 長さ: 秒
}

// 楽器ごとの音源を作業場の中で1度だけ組み立てて持ち続ける棚。
// 音源は発音のたびにノードを作って鳴り終わりで捨てるため、音源そのものを打点ごとに作り直す必要は無い。
export class 楽器の音源棚 {
    private readonly _音高の音源: Map<楽器, 音高を鳴らす音源> = new Map()
    private readonly _打楽器の打撃: Map<string, 打楽器の打撃> = new Map()

    public constructor(private readonly _作業場: I音声の作業場) {}

    public 音高の音源を貸す(対象の楽器: 楽器): 音高を鳴らす音源 {
        const 手持ち = this._音高の音源.get(対象の楽器)
        if (手持ち !== undefined) return 手持ち
        const 音色 = 楽器の音色を求める(対象の楽器)
        if (音色.種類 !== '音高を鳴らす') {
            throw new Error(`打楽器の楽器へ音高を鳴らさせようとしています: ${対象の楽器}`)
        }
        const 音源 = 音高の音源を組み立てる(this._作業場, 音色.作り方)
        this._音高の音源.set(対象の楽器, 音源)
        return 音源
    }

    public 打楽器の打撃を貸す(対象の楽器: 楽器, 打楽器: 打楽器の種類): 打楽器の打撃 {
        const 棚の見出し = `${対象の楽器}/${打楽器}`
        const 手持ち = this._打楽器の打撃.get(棚の見出し)
        if (手持ち !== undefined) return 手持ち
        const 音色 = 楽器の音色を求める(対象の楽器)
        if (音色.種類 !== '打楽器を鳴らす') {
            throw new Error(`音高の楽器へ打楽器を鳴らさせようとしています: ${対象の楽器}`)
        }
        const 作り方 = 音色.打楽器ごとの作り方[打楽器]
        const 打撃: 打楽器の打撃 = {
            音源: 打楽器の音源を組み立てる(this._作業場, 作り方),
            長さ: 打楽器の打撃の長さ(作り方),
        }
        this._打楽器の打撃.set(棚の見出し, 打撃)
        return 打撃
    }

    public 破棄する(): void {
        for (const 音源 of this._音高の音源.values()) 音源.破棄する()
        for (const 打撃 of this._打楽器の打撃.values()) 打撃.音源.破棄する()
        this._音高の音源.clear()
        this._打楽器の打撃.clear()
    }
}
