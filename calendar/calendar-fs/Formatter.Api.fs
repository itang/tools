module Formatter.Api

open Calendar.Api

type IFormatter<'a> =
    abstract member Format: ICalendar -> 'a
    abstract member Name: string with get
