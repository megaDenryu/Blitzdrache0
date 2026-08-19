import { span, type DivC } from 'sengen-ui'
import { 外殻レイアウト, アクティビティID, 設定アイコン } from 'VscodeShellLayout'
import type { テーマ配色 } from '../../../submodules/VscodeShellLayout/src/テーマ/テーマ型.ts'

export const エクスプローラーアクティビティID = アクティビティID('explorer')
export const 設定アクティビティID = アクティビティID('settings')

// エディター外殻のシェルレイアウト(アクティビティバー・左右サイドバー・メニュー・
// ステータスバーの配置)を構築する。呼び出し元(エディター外殻)は初期テーマの配色と
// 右サイドバーへ渡すインスペクタースロットだけを渡し、できあがったシェルを受け取る。
export function シェルを構築する(初期vsl配色: Partial<テーマ配色>, インスペクタースロット: DivC): 外殻レイアウト {
    return new 外殻レイアウト({
        テーマ: 初期vsl配色,
        タイトル: 'Blitzdrache0 エディター',
        アクティビティ項目一覧: [{
            id: エクスプローラーアクティビティID,
            ラベル: 'エクスプローラー',
            アイコン: (size, color) => span({ text: 'E' }).setStyleCSS({
                fontWeight: 'bold', fontSize: `${size * 0.8}px`, color,
                display: 'flex', alignItems: 'center', justifyContent: 'center', width: `${size}px`, height: `${size}px`,
            }),
        }],
        アクティビティバー下部項目一覧: [{
            id: 設定アクティビティID,
            ラベル: '設定',
            アイコン: (size, color) => 設定アイコン(size, color),
        }],
        メニューバー表示: true,
        ステータスバー表示: true,
        ステータスバー右テキスト: 'Blitzdrache0 v0.1.0',
        パネル初期表示: false,
        右サイドバー有効: true,
        右サイドバー内容: インスペクタースロット,
        // 260px既定だとインスペクターの見出し・バッジ・モード切替ボタンが折れずに収まらない。
        // 各パネルのラベル幅実測から280pxへ広げる(参照: 是正の規律)。
        右サイドバー既定幅: 280,
        // アクティビティ項目が1つ('エクスプローラー')しかなく、48px幅に日本語ラベルを
        // 常時表示すると任意の文字位置で折り返される(「エクスプロ/ーラー」)。VSCode本家と
        // 同じくアイコン+ツールチップのみにする。
        アクティビティバーラベルを常に隠す: true,
    })
}
