module Formater.Api

open Calendar.Api

type IFormater<'a> =
    abstract member Format: ICalendar -> 'a
    abstract member Name: string with get
