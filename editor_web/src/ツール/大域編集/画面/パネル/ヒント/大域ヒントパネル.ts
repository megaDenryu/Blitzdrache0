import { div, span, DivC, SpanC, LV2HtmlComponentBase } from 'sengen-ui'
import type { 大域編集モード } from '../モード切替/大域モード定義.ts'
import { ヒント枠, 強調 } from './スタイル.css.ts'

// 各モードの操作案内文を切り替え表示するLV2素部品。
export class 大域ヒントパネル extends LV2HtmlComponentBase {
    protected _componentRoot: DivC
    private readonly _テキスト要素: SpanC

    public constructor(初期モード: 大域編集モード = '選択') {
        super()
        this._テキスト要素 = span()
        this._componentRoot = div({ class: ヒント枠 }).child(this._テキスト要素)
        this.モードを更新する(初期モード)
    }

    public モードを更新する(モード: 大域編集モード): void {
        this._テキスト要素.clearChildren()
        switch (モード) {
            case '選択':
                this._テキスト要素.childs([
                    span({ text: '左クリックで道路点を選択してインスペクター表示' }),
                    span({ text: ' (左ドラッグは何もしません)。※右ドラッグでカメラ回転、中ドラッグでカメラ平行移動' }),
                ])
                break
            case '大域造成':
                this._テキスト要素.childs([
                    span({ class: 強調, text: '左ドラッグで山脈造成' }),
                    span({ text: ' (シフトで削り/平滑化)。※右ドラッグでカメラ回転、中ドラッグでカメラ平行移動' }),
                ])
                break
            case '広域道路作成':
                this._テキスト要素.childs([
                    span({ class: 強調, text: '左クリックで点追加' }),
                    span({ text: ' (チャンク境界を跨いで配置可能)。※右ドラッグでカメラ回転、中ドラッグでカメラ平行移動' }),
                ])
                break
            case '広域道路編集':
                this._テキスト要素.childs([
                    span({ class: 強調, text: '点を選択して削除・調整' }),
                    span({ text: '。※右ドラッグでカメラ回転、中ドラッグでカメラ平行移動' }),
                ])
                break
        }
    }
}
