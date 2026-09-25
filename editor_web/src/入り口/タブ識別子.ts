import type { チャンク座標 } from '../境界/通信/index.ts'
import { 大域世界表示名, 建物定義IDを生成する, type 建物定義ID, 楽曲IDを生成する, type 楽曲ID } from '../境界/index.ts'

// エディタータブの一意識別子。JSのMapは参照等価のため、Mapのキーには`文字列()`が返す
// stringを使う(生値へ剥がすのはMapを持つ側の境界1箇所に閉じる)。
export class タブ識別子 {
    private constructor(private readonly _文字列: string) {}

    public static 大域世界(): タブ識別子 {
        return new タブ識別子(大域世界表示名)
    }

    public static 使い方(): タブ識別子 {
        return new タブ識別子('使い方')
    }

    public static マテリアル(): タブ識別子 {
        return new タブ識別子('マテリアル')
    }

    public static 建物から生成する(建物定義ID: 建物定義ID): タブ識別子 {
        return new タブ識別子(`建物_${建物定義ID}`)
    }

    public static 楽曲から生成する(楽曲ID: 楽曲ID): タブ識別子 {
        return new タブ識別子(`楽曲_${楽曲ID}`)
    }

    public static チャンクから生成する(座標: チャンク座標): タブ識別子 {
        return new タブ識別子(`チャンク_${座標.x}_${座標.z}`)
    }

    // VscodeShellLayoutのタブ選択・タブ閉じるイベントは生のstringでタブIDを渡してくる。
    // 外部から受け取った綴りをこの型へ戻す境界はここ1箇所に閉じる。
    public static 文字列から復元する(文字列: string): タブ識別子 {
        return new タブ識別子(文字列)
    }

    // タブ識別子がチャンクのものであれば座標を復元する。チャンク以外の識別子ならnull。
    public チャンク座標を復元する(): チャンク座標 | null {
        if (!this._文字列.startsWith('チャンク_')) return null
        const 部分 = this._文字列.slice('チャンク_'.length).split('_')
        if (部分.length !== 2) return null
        const x = Number(部分[0])
        const z = Number(部分[1])
        if (Number.isNaN(x) || Number.isNaN(z)) return null
        return { x, z }
    }

    // タブ識別子が建物のものであれば建物定義IDを復元する。建物以外の識別子ならnull。
    public 建物定義IDを復元する(): 建物定義ID | null {
        if (!this._文字列.startsWith('建物_')) return null
        const 文字列 = this._文字列.slice('建物_'.length)
        return 文字列 === '' ? null : 建物定義IDを生成する(文字列)
    }

    // タブ識別子が楽曲のものであれば楽曲IDを復元する。楽曲以外の識別子ならnull。
    public 楽曲IDを復元する(): 楽曲ID | null {
        if (!this._文字列.startsWith('楽曲_')) return null
        const 文字列 = this._文字列.slice('楽曲_'.length)
        return 文字列 === '' ? null : 楽曲IDを生成する(文字列)
    }

    public 文字列(): string {
        return this._文字列
    }
}
