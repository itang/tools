package cli

import utest.*

object CommandTest extends TestSuite:

  val tests = Tests:
    test("from"):
      assert(Command.from("month") == Some(MonthCommand))
      assert(Command.from("m") == Some(MonthCommand))
      assert(Command.from("-m") == Some(MonthCommand))

      assert(Command.from("d") == Some(DaysCommand))
      assert(Command.from("-d") == Some(DaysCommand))
      assert(Command.from("days") == Some(DaysCommand))

      assert(Command.from("q") == Some(QuarterCommand))
      assert(Command.from("-q") == Some(QuarterCommand))
      assert(Command.from("quarter") == Some(QuarterCommand))

      assert(Command.from("week") == Some(WeekCommand))
      assert(Command.from("-w") == Some(WeekCommand))
      assert(Command.from("w") == Some(WeekCommand))

      assert(Command.from("unknown") == None)

    test("fromList"):
      import Command.fromList
      assert:
        fromList(List("m", "d", "q", "w")) == List(
          ("m", MonthCommand),
          ("d", DaysCommand),
          ("q", QuarterCommand),
          ("w", WeekCommand)
        )

      assert:
        fromList(List("m", "unknown1", "d", "unknown2"), _ => {}) == List(("m", MonthCommand), ("d", DaysCommand))
