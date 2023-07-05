module Formater.Api

open Calendar.Api

type IFormater =
    abstract member Format: ICalendar -> string
