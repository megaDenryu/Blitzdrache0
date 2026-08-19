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
                    span({ text: '道路点は広域道路編集モードで選びます' }),
                    span({ text: ' (このモードでは左クリック・左ドラッグとも何もしません)。※右ドラッグでカメラ回転、中ドラッグでカメラ平行移動' }),
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
                    span({ class: 強調, text: '左クリックで道の終わりへ点を追加' }),
                    span({ text: ' (チャンク境界を跨いで配置可能)。※右ドラッグでカメラ回転、中ドラッグでカメラ平行移動' }),
                ])
                break
            case '広域道路編集':
                this._テキスト要素.childs([
                    span({ class: 強調, text: '点を左ドラッグで移動、クリックで選択' }),
                    span({ text: '。道の上をクリックすると、その場所へ点を割り込ませます。選んだ点は道路パネルの削除ボタンで消せます。※右ドラッグでカメラ回転、中ドラッグでカメラ平行移動' }),
                ])
                break
        }
    }
}
