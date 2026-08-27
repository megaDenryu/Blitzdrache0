import type { 予約補充コマンド } from 'SengenAudio'

// 予約の補充を実際に行える相手の規約。演奏サービスが実装する。
export interface I予約を補充できる側 {
    予約を補充する(): void
}

// 演奏予定表が「予約が尽きかけた」と告げる先。予定表は1度しか配線できないため、
// 演奏が始まるたびに作り直される中身ではなく、演奏サービスへ委ねる薄い層をここに置く。
export class 予約の補充を演奏へ委ねる implements 予約補充コマンド {
    public constructor(private readonly _演奏: I予約を補充できる側) {}

    public 補充する(): void {
        this._演奏.予約を補充する()
    }
}
