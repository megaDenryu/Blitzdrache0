import { テーマCSS変数名 } from '../../../submodules/VscodeShellLayout/src/テーマ/テーマCSS変数.ts'
import type { テーマ配色 } from '../../../submodules/VscodeShellLayout/src/テーマ/テーマ型.ts'
import type { テーマ名, エディターテーマ定義, エディター配色 } from './テーマ型.ts'
import { 既定テーマ名 } from './テーマ型.ts'
import { エディターCSS変数名 } from './テーマ変数.ts'
import { テーマ登録簿 } from './テーマ定義一覧.ts'
import { テーマの控えを読む, テーマの控えを書く } from '../../境界/保存/index.ts'

function 登録テーマを既定つきで取得する(名前: テーマ名): エディターテーマ定義 {
    const 対象テーマ = テーマ登録簿.get(名前) ?? テーマ登録簿.get(既定テーマ名)
    if (対象テーマ === undefined) {
        throw new Error(`テーマ登録簿に既定テーマ「${既定テーマ名}」が登録されていません`)
    }
    return 対象テーマ
}

// エディター全体のカラーテーマ状態管理・DOMルートへのCSS変数一括反映・永続化を行うサービス。
export class テーマ管理サービス {
    private _現在テーマ: エディターテーマ定義
    private readonly _リスナー一覧: Set<(テーマ: エディターテーマ定義) => void> = new Set()

    public constructor() {
        const 控え = テーマの控えを読む()
        this._現在テーマ = 登録テーマを既定つきで取得する(控え.種別 === '控えあり' ? 控え.テーマ名 : 既定テーマ名)
    }

    public 現在テーマを取得する(): エディターテーマ定義 {
        return this._現在テーマ
    }

    public テーマを変更する(名前: テーマ名): void {
        const 対象テーマ = テーマ登録簿.get(名前)
        if (!対象テーマ) return

        this._現在テーマ = 対象テーマ
        this.DOMへ適用する(対象テーマ)
        テーマの控えを書く(名前)

        for (const リスナー of this._リスナー一覧) {
            リスナー(対象テーマ)
        }
    }

    // ルート要素（document.documentElement）へVSLおよびエディター独自のCSS変数を1箇所で設定する。
    public DOMへ適用する(テーマ: エディターテーマ定義 = this._現在テーマ): void {
        if (typeof document === 'undefined' || !document.documentElement) {
            return
        }
        const root = document.documentElement

        // VSLテーマCSS変数の適用
        let vslキー: keyof テーマ配色
        for (vslキー in テーマCSS変数名) {
            const 値 = テーマ.vsl配色[vslキー]
            if (値 !== undefined) {
                root.style.setProperty(テーマCSS変数名[vslキー], 値)
            }
        }

        // エディター固有CSS変数の適用
        let editorキー: keyof エディター配色
        for (editorキー in エディターCSS変数名) {
            const 値 = テーマ.エディター配色[editorキー]
            if (値 !== undefined) {
                root.style.setProperty(エディターCSS変数名[editorキー], 値)
            }
        }
    }

    public onテーマ変更(リスナー: (テーマ: エディターテーマ定義) => void): () => void {
        this._リスナー一覧.add(リスナー)
        return () => {
            this._リスナー一覧.delete(リスナー)
        }
    }
}
