module Calendar.Impl

open System
open Common

open Calendar.Api

type Calendar(startDate: DateTime, days: int) =
    interface ICalendar with
        member _.Dates() : seq<DateTime> = startDate.Dates(days)
