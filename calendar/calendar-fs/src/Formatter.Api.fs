module Formatter.Api

open Calendar.Api

type IFormatter<'T> =
    abstract member Format: ICalendar -> 'T
    abstract member Name: string with get
