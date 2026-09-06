import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../../境界/index.ts'

// タイムライン部品の内側の行。人が編集中ずっと見る対象であるため縦には伸ばさない
// (設計正本の判断15)。見出しは固定し、カードの列だけを横にスクロールさせる
// (エディター制作スキル第7条「文脈は固定する」)。画面直下の「タイムラインの行」
// (画面/スタイル.css.ts)は、この部品を配置する外側の枠であり別の階層である。
export const タイムライン内側行 = style({
    display: 'flex',
    alignItems: 'stretch',
    gap: '8px',
    flexShrink: 0,
    padding: '8px 0',
})

export const タイムライン見出し = style({
    flexShrink: 0,
    alignSelf: 'center',
    fontSize: '12px',
    fontWeight: 600,
    color: エディターCSS変数('テキスト主'),
    whiteSpace: 'nowrap',
})

export const タイムラインのカード列域 = style({
    display: 'flex',
    alignItems: 'stretch',
    gap: '8px',
    overflowX: 'auto',
    overflowY: 'hidden',
    flex: 1,
    minWidth: 0,
})

export const 案内文 = style({
    fontSize: '12px',
    color: エディターCSS変数('テキスト副'),
    display: 'flex',
    alignItems: 'center',
    padding: '4px 8px',
})

export const 末尾へ追加ボタンの配置 = style({
    flexShrink: 0,
    alignSelf: 'center',
})
