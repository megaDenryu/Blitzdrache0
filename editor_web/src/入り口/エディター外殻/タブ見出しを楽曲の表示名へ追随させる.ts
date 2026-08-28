import type { 外殻レイアウト } from 'VscodeShellLayout'
import type { I楽曲の表示名の届け先 } from '../../ツール/楽曲編集/index.ts'

// 楽曲の表示名が変わったとき、その楽曲を開いている文書タブの見出しを同じ名前へ差し替える。
// 楽曲を開くたびに1件ずつ作り、受け持つタブの綴りを保持する。
export class タブ見出しを楽曲の表示名へ追随させる implements I楽曲の表示名の届け先 {
    public constructor(
        private readonly _シェル: 外殻レイアウト,
        private readonly _タブの綴り: string,
    ) {}

    public 楽曲の表示名が変わった(新しい表示名: string): void {
        this._シェル.タブのラベルを変える(this._タブの綴り, 新しい表示名)
    }
}
