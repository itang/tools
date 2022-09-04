interface Dist {
  lang: string;
  args?: string[];
  name?: string;
}

type Dists = Array<Dist>;
const dists: Dists = [
  { lang: "rust", name: "rs" },
  { lang: "dart" },
  { lang: "js" },
  { lang: "ts", args: ["--typedArrays", "false"] },
  { lang: "python", name: "py" },
  { lang: "dart" },
];

await Deno.spawn("dotnet", {
  args: ["fable", "clean"],
  stdout: "inherit",
  stderr: "inherit",
  stdin: "inherit",
});

const distTasks = dists.map((dist) => {
  const cmd = "dotnet";

  const name = dist.name ?? dist.lang;
  let args = ["fable", "-o", `dist_${name}`, "--lang", dist.lang];
  if (dist.args) {
    args = args.concat(dist.args);
  }

  console.log(`EXEC: ${cmd} ${args.join(" ")}`);

  Deno.spawn(cmd, { args: args, stdout: "inherit", stderr: "inherit" });
});

await Promise.all(distTasks);
