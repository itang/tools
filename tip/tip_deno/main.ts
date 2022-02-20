const head = Deno.args.find((_) => true);
if (head) {
  const file = `${Deno.env.get("HOME") || ""}/bin/data/tip/${head}.md`;
  const content = Deno.readTextFileSync(file);
  console.info(content);
}
