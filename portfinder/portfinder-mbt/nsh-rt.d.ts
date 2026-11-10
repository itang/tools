// `nsh:rt` is internal and ships no declaration; the MoonBit app uses this
// subset of its renderer ABI.
declare module "nsh:rt" {
  export function element(tag: string): unknown;
  export function text(value: string): unknown;
  export function append(parent: unknown, child: unknown): void;
  export function insert(parent: unknown, value: unknown, marker?: unknown): void;
  export function prop(node: unknown, name: string, value: unknown): void;
  export function component(component: unknown, props: unknown): unknown;
}
