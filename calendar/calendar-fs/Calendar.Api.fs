module Calendar.Api

type ICalendar =
    abstract Dates: unit -> seq<System.DateTime>
