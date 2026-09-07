import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../境界/index.ts'

// 演奏の操作帯と同じ見た目の札にして、同じ行に並んだときに1つの帯として読めるようにする。
export const 楽曲名の枠 = style({
    display: 'flex',
    alignItems: 'center',
    gap: '8px',
    padding: '10px 12px',
    minWidth: '240px',
    flexShrink: 0,
    boxSizing: 'border-box',
    backgroundColor: エディターCSS変数('カード背景'),
    border: `1px solid ${エディターCSS変数('カード枠線')}`,
    borderRadius: '6px',
})

export const 楽曲名の入力 = style({
    flex: 1,
    minWidth: '120px',
    padding: '4px 8px',
    fontSize: '13px',
    fontWeight: 700,
    borderRadius: '4px',
    border: `1px solid ${エディターCSS変数('境界線')}`,
    backgroundColor: エディターCSS変数('パネル背景'),
    color: エディターCSS変数('テキスト主'),
    boxSizing: 'border-box',
})

// 保存先を決めている名乗りは変えられないため、入力欄ではなく添えの表示にする。
export const 名乗りの添え = style({
    fontFamily: 'monospace',
    fontSize: '11px',
    color: エディターCSS変数('テキスト薄'),
    whiteSpace: 'nowrap',
    flexShrink: 0,
})

// 保存・読み込みの押しどころが増えても他の子を押し出さないよう、行の右端へ寄せて詰める。
export const 永続化操作の並び = style({
    display: 'flex',
    alignItems: 'center',
    gap: '6px',
    marginLeft: 'auto',
    flexShrink: 0,
})

// 保存・読み込み結果やエラーを表示する1行物のため、末尾を省略記号で収める。
// 全文は呼び出し側がtitle属性で提供する(楽曲名の欄.ts の 状態文言を更新する 参照)。
export const 永続化の状態文言 = style({
    fontSize: '11px',
    color: エディターCSS変数('テキスト副'),
    whiteSpace: 'nowrap',
    overflow: 'hidden',
    textOverflow: 'ellipsis',
    maxWidth: '140px',
})

export const 永続化のエラー状態文言 = style([永続化の状態文言, {
    color: エディターCSS変数('危険ボタン文字'),
}])
