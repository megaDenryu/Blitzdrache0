import type { ポインタ操作種別 } from './ポインタ操作割り当て.ts'

interface 位置3次元様 {
    readonly x: number
    readonly y: number
    readonly z: number
}

export interface カメラ操作対象ビュー {
    readonly カメラ制御: {
        回転する(デルタX: number, デルタY: number): void
        移動する(デルタX: number, デルタY: number): void
        注視点を取得する(): 位置3次元様
    }
    readonly 注視点表示制御: {
        操作された(x: number, y: number, z: number): void
    }
}

// 右ドラッグの回転・中ドラッグの平行移動をカメラ制御へ適用し、注視点の印へ操作があったことを
// 知らせる操作サービス。三次元を持つエディターのポインタ配線が共有する
// (依存する三次元ビューはコンストラクタで注入し、呼び出しのたびに引き回さない)。
export class カメラ操作適用器 {
    public constructor(private readonly _ビュー: カメラ操作対象ビュー) {}

    // 視点の操作でなければ(左ボタンの主作業)falseを返し、呼び出し元がその動きを編集の操作へ回す。
    public 視点の操作として適用したか(操作種別: ポインタ操作種別, デルタX: number, デルタY: number): boolean {
        if (操作種別 === 'カメラ回転') this._ビュー.カメラ制御.回転する(デルタX, デルタY)
        else if (操作種別 === 'カメラ平行移動') this._ビュー.カメラ制御.移動する(デルタX, デルタY)
        else return false

        const 注視点 = this._ビュー.カメラ制御.注視点を取得する()
        this._ビュー.注視点表示制御.操作された(注視点.x, 注視点.y, 注視点.z)
        return true
    }
}
