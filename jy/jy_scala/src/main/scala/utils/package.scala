package utils

import java.text.SimpleDateFormat
import java.util.Date

extension (d: Date)
  def format(pattern: String = "yyyyMMdd"): String = new SimpleDateFormat(pattern).format(d)

extension[T] (v: T)
  def orElse(pred: Boolean)(elseAction: (T) => T): T = if (pred) elseAction(v) else v

  def orElse(pred: (T) => Boolean)(elseAction: (T) => T): T = orElse(pred(v))(elseAction)

  inline infix def |>[B] (inline fn: T => B): B = fn(v)

  inline infix def |>!(inline proc: T => Unit) : T =
    proc(v)
    v

extension[A, B] (t: (A, B))
  inline infix def ||>[C](f: (A, B) => C): C = f(t._1, t._2)

def time[T](block: => T): T =
  val start = System.currentTimeMillis()
  val ret = block
  val end = System.currentTimeMillis()
  println(s">>Elapsed time: ${end - start} msecs")
  ret
