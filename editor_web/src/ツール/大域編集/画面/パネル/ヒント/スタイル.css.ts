import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../../境界/index.ts'

// ヒント文は複数語からなる操作案内であり折返しを許す。text-wrap: prettyで折り位置を
// 最適化する(グローバルのline-break: strictと組み合わせ、禁則も守った読みやすい折返しにする)。
// 灯りの色地+左端の木のアクセントバーでカードや情報バッジと区別する(ヒント帯の役割の明確化)。
export const ヒント枠 = style({
    padding: '8px 12px 8px 9px',
    fontSize: '11px',
    lineHeight: '1.4',
    backgroundColor: エディターCSS変数('ヒント背景'),
    borderRadius: '8px',
    border: `1px solid ${エディターCSS変数('境界線')}`,
    borderLeft: `3px solid ${エディターCSS変数('境界線')}`,
    color: エディターCSS変数('テキスト薄'),
    textWrap: 'pretty',
})

export const 強調 = style({
    color: エディターCSS変数('アクセント文字'),
    fontWeight: 'bold',
})
