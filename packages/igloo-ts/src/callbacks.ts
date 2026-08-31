import type { CallbackId } from 'iced:app/callbacks@0.1.0';
import type { MessageValue } from 'iced:app/app-instance@0.1.0';
import type { Viewport } from 'iced:app/message-types@0.1.0';

/**
 * A guest-owned callback, keyed by the {@link CallbackId} it was assigned when
 * pushed onto a {@link Frame}.
 *
 * A `fixed` callback carries the plugin message directly (e.g. `Button.onPress`);
 * the others hold a mapper that turns a widget's runtime value into a message
 * (e.g. `Checkbox.onToggle` receives the new boolean state).
 */
export type Callback<Msg> =
  | { kind: 'fixed'; msg: Msg }
  | { kind: 'bool'; map: (value: boolean) => Msg }
  | { kind: 'f32'; map: (value: number) => Msg }
  | { kind: 'f64'; map: (value: number) => Msg }
  | { kind: 'u64'; map: (value: bigint) => Msg }
  | { kind: 'string'; map: (value: string) => Msg }
  | { kind: 'viewport'; map: (value: Viewport) => Msg };

/**
 * One `view()` call's worth of callbacks.
 *
 * `base` is the id of slot 0, so ids minted by an earlier frame fall below it
 * and miss cleanly on lookup instead of resolving to the wrong callback.
 */
export class Frame<Msg> {
  private readonly base: number;
  private readonly slots: Callback<Msg>[] = [];

  constructor(base: number) {
    this.base = base;
  }

  /** The id one past the last slot — the `base` for the frame that replaces it. */
  nextBase(): number {
    return this.base + this.slots.length;
  }

  /** Append a callback and return the id the host should dispatch it by. */
  push(cb: Callback<Msg>): CallbackId {
    const id = this.nextBase();
    this.slots.push(cb);
    return id;
  }

  /** Resolve `id` to a slot in this frame, or `undefined` if it falls outside it. */
  get(id: CallbackId): Callback<Msg> | undefined {
    const index = id - this.base;
    if (index < 0 || index >= this.slots.length) return undefined;
    return this.slots[index];
  }
}

let currentFrame: Frame<unknown> | undefined;

/**
 * Run `fn` with `frame` installed as the frame that widget callbacks register
 * into, restoring the previous frame afterwards (including when `fn` throws).
 */
export function withFrame<Msg, T>(frame: Frame<Msg>, fn: () => T): T {
  const previous = currentFrame;
  currentFrame = frame as Frame<unknown>;
  try {
    return fn();
  } finally {
    currentFrame = previous;
  }
}

function active(): Frame<unknown> {
  if (currentFrame === undefined) {
    throw new Error(
      'igloo: a widget callback was registered outside of view() — build widgets inside your app view function',
    );
  }
  return currentFrame;
}

/** Register a fixed callback carrying `msg`; used by `on-press`-style callbacks. */
export function pushFixed(msg: unknown): CallbackId {
  return active().push({ kind: 'fixed', msg });
}

export function pushBool(map: (value: boolean) => unknown): CallbackId {
  return active().push({ kind: 'bool', map });
}

export function pushF32(map: (value: number) => unknown): CallbackId {
  return active().push({ kind: 'f32', map });
}

export function pushF64(map: (value: number) => unknown): CallbackId {
  return active().push({ kind: 'f64', map });
}

export function pushU64(map: (value: bigint) => unknown): CallbackId {
  return active().push({ kind: 'u64', map });
}

export function pushString(map: (value: string) => unknown): CallbackId {
  return active().push({ kind: 'string', map });
}

export function pushViewport(map: (value: Viewport) => unknown): CallbackId {
  return active().push({ kind: 'viewport', map });
}

/**
 * Match a callback against the value an interaction produced, returning the
 * message it yields — or `undefined` when the kinds do not line up, which the
 * caller treats as a miss rather than an error.
 */
export function resolve<Msg>(cb: Callback<Msg>, value: MessageValue): Msg | undefined {
  switch (value.tag) {
    case 'fixed':
      return cb.kind === 'fixed' ? cb.msg : undefined;
    case 'bool-value':
      return cb.kind === 'bool' ? cb.map(value.val) : undefined;
    case 'f32-value':
      return cb.kind === 'f32' ? cb.map(value.val) : undefined;
    case 'f64-value':
      return cb.kind === 'f64' ? cb.map(value.val) : undefined;
    case 'u64-value':
      return cb.kind === 'u64' ? cb.map(value.val) : undefined;
    case 'string-value':
      return cb.kind === 'string' ? cb.map(value.val) : undefined;
    case 'viewport-value':
      return cb.kind === 'viewport' ? cb.map(value.val) : undefined;
  }
}
