// Behavioural checks for the pure-MoonBit port of the old JS scanners.

///|
test "all_digits / is_decimal" {
  assert_true(all_digits("8080"))
  assert_false(all_digits("80a0"))
  assert_false(all_digits(""))
  assert_true(is_decimal("12.5"))
  assert_true(is_decimal("0"))
  assert_false(is_decimal("1.2.3"))
  assert_false(is_decimal(""))
}

///|
test "parse_etime" {
  assert_eq(parse_etime("02:03:04"), 2 * 3600 + 3 * 60 + 4)
  assert_eq(parse_etime("1-02:03:04"), 86400 + 2 * 3600 + 3 * 60 + 4)
}

///|
test "ps_fields keeps command with spaces" {
  let f = ps_fields("  1234     1 01:02 456 0.5 /usr/bin/my proc")
  assert_eq(f.length(), 6)
  assert_eq(f[0].to_owned(), "1234")
  assert_eq(f[1].to_owned(), "1")
  assert_eq(f[2].to_owned(), "01:02")
  assert_eq(f[3].to_owned(), "456")
  assert_eq(f[4].to_owned(), "0.5")
  assert_eq(f[5].to_owned(), "/usr/bin/my proc")
}

///|
test "port_after_last_colon" {
  assert_eq(port_after_last_colon("*:8080").to_owned(), "8080")
  assert_eq(port_after_last_colon("[::1]:443").to_owned(), "443")
}

///|
test "ss_pids finds every pid" {
  let p = ss_pids(
    "tcp LISTEN 0 4096 0.0.0.0:80 0.0.0.0:* users:((\"a\",pid=12,fd=3),(\"b\",pid=34,fd=4))",
  )
  assert_eq(p, [12, 34])
}

///|
test "lsof_address drops trailing LISTEN" {
  let f = words("node 1234 user 12u IPv4 0x0 0t0 TCP *:8080 (LISTEN)")
  assert_eq(lsof_address(f), "*:8080")
}

///|
test "emit_rows orders by first port and formats 10 fields" {
  let info : Map[Int32, PsInfo] = Map([])
  info.set(20, {
    ppid: "1",
    name: "two",
    path: "",
    started: "100",
    cpu: "5",
    mem: "200",
  })
  let tcp : Map[Int32, Array[Int32]] = Map([])
  tcp.set(20, [443, 80])
  let udp : Map[Int32, Array[Int32]] = Map([])
  udp.set(10, [53])
  let addrs : Map[Int32, Array[String]] = Map([])
  addrs.set(20, ["TCP 0.0.0.0:80", "TCP 0.0.0.0:443"])
  addrs.set(10, ["UDP *:53"])
  let out = emit_rows(info, tcp, udp, addrs)
  let lines = out.split("\n").to_array()
  assert_eq(lines.length(), 2)
  // pid 10 (port 53) before pid 20 (first port 80)
  // pid 10 has only udp 53; 10 fields: pid|name|ppid|path|tcp|udp|started|cpu|mem|addrs
  assert_true(lines[0].has_prefix("10|"))
  assert_true(lines[0].contains("UDP *:53"))
  assert_true(lines[1].has_prefix("20|"))
  assert_true(lines[1].contains("80,443"))
}

///|
test "macos_rows and linux_rows parse end to end" {
  let ps = " 1234 1 01:02:03 4567 0.5 /usr/bin/node\n"
  let tcp = "COMMAND PID USER FD TYPE DEVICE SIZE/OFF NODE NAME\nnode 1234 u 12u IPv4 0x0 0t0 TCP *:8080 (LISTEN)\n"
  let mac = macos_rows(tcp, "", ps, 1_000_000)
  assert_true(mac.contains("8080"))
  assert_true(mac.contains("node"))

  let ss = "tcp LISTEN 0 4096 0.0.0.0:8080 0.0.0.0:* users:((\"node\",pid=1234,fd=3))\n"
  let ps_lin = " 1234 1 3600 4567 0.5 node\n"
  let lin = linux_rows(ss, ps_lin, 1_000_000)
  assert_true(lin.contains("8080"))
  assert_true(lin.contains("node"))
}
