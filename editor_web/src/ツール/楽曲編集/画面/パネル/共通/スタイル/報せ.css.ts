import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../../../境界/index.ts'

// 操作の結果として起きることを前もって知らせる帯。テーマに警告専用の配色が無いため、
// 注意を促す用途で用意されている危険ボタンの配色を借りる。
export const 警告の帯 = style({
    padding: '6px 10px',
    fontSize: '11px',
    borderRadius: '4px',
    backgroundColor: エディターCSS変数('危険ボタン背景'),
    border: `1px solid ${エディターCSS変数('危険ボタン枠線')}`,
    color: エディターCSS変数('危険ボタン文字'),
})

export const 案内の帯 = style({
    padding: '8px 12px',
    fontSize: '12px',
    borderRadius: '4px',
    backgroundColor: エディターCSS変数('ビューポート背景'),
    border: `1px solid ${エディターCSS変数('境界線')}`,
    color: エディターCSS変数('テキスト主'),
    fontFamily: 'monospace',
})
