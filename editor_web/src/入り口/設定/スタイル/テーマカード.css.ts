import { style, globalStyle } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../テーマ/テーマ変数.ts'

// 設定パネルに並ぶテーマ選択カードのスタイル。
export const テーマリスト = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '10px',
})

export const テーマカード = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '8px',
    padding: '12px',
    borderRadius: '6px',
    border: `1px solid ${エディターCSS変数('境界線')}`,
    backgroundColor: エディターCSS変数('カード背景'),
    backdropFilter: エディターCSS変数('ガラス背景ぼかし'),
    boxShadow: エディターCSS変数('カード影'),
    cursor: 'pointer',
    transition: 'all 0.15s ease',
    ':hover': {
        borderColor: エディターCSS変数('アクセントホバー'),
        backgroundColor: エディターCSS変数('項目ホバー背景'),
    },
})

globalStyle(`${テーマカード}[data-selected="true"]`, {
    borderColor: エディターCSS変数('選択枠線'),
    backgroundColor: エディターCSS変数('選択背景'),
})

export const カードヘッダー = style({
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'space-between',
    gap: '8px',
})

export const テーマ表示名 = style({
    fontSize: '13px',
    fontWeight: 600,
    color: エディターCSS変数('テキスト主'),
    minWidth: 0,
    flex: '1',
    whiteSpace: 'nowrap',
    overflow: 'hidden',
    textOverflow: 'ellipsis',
})

// 活性表現はエクスプローラーの選択項目・モード切替の選択ボタンと同じ淡若葉地+糸杉文字+
// 糸杉の細枠に揃える(工房テーマの活性表現の統一)。
export const 選択バッジ = style({
    fontSize: '10px',
    padding: '2px 6px',
    borderRadius: '4px',
    backgroundColor: エディターCSS変数('選択背景'),
    color: エディターCSS変数('選択文字'),
    border: `1px solid ${エディターCSS変数('選択枠線')}`,
    fontWeight: 'bold',
    whiteSpace: 'nowrap',
    flexShrink: 0,
})

// テーマ説明は複数語からなる文であり折返しを許す。text-wrap: prettyで折り位置を最適化する
// (グローバルのline-break: strictと組み合わせ、禁則も守った読みやすい折返しにする)。
export const テーマ説明 = style({
    fontSize: '11px',
    color: エディターCSS変数('テキスト副'),
    lineHeight: '1.4',
    textWrap: 'pretty',
})

export const パレットプレビュー = style({
    display: 'flex',
    gap: '4px',
    alignItems: 'center',
    marginTop: '4px',
})

export const 色スウォッチ = style({
    width: '16px',
    height: '16px',
    borderRadius: '3px',
    border: `1px solid ${エディターCSS変数('境界線薄')}`,
})
