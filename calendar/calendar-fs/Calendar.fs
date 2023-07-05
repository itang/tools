module Calendar

open System
open Common

type ICalendar =
    abstract Dates: unit -> seq<DateTime>

type Calendar(startDate: DateTime, days: int) =
    interface ICalendar with
        member _.Dates() : seq<DateTime> = startDate.Dates(days)
