/// <reference path="../src/generated/wit.d.ts" />

/**
 * Type-level tests. These are checked by `tsc --noEmit` via `tsconfig.test.json`;
 * there is no runtime assertion library. Each `@ts-expect-error` must trigger, or
 * the compile fails.
 */
import {
  createApp,
  Column,
  Row,
  Text,
  Space,
  Rule,
  Button,
  Checkbox,
  Slider,
  Scrollable,
  Container,
  type App,
} from '../src/index.js';

type Msg =
  | { type: 'inc' }
  | { type: 'toggle'; value: boolean }
  | { type: 'slide'; value: number };

/** A message-free widget fits any app's tree. */
const plainColumn: App<null, Msg> = {
  init: () => null,
  update: () => {},
  view: () => Column.new().push(Text.new('hi')).push(Space.new()).push(Rule.horizontal(1)),
};
void plainColumn;

/** Handlers infer the message type without `as const`. */
const goodApp: App<number, Msg> = {
  init: () => 0,
  update: () => {},
  view: () =>
    Column.new()
      .push(Button.new(Text.new('+')).onPress(() => ({ type: 'inc' })))
      .push(Checkbox.new(false).onToggle((value) => ({ type: 'toggle', value })))
      .push(Slider.new(0, 10, 0, (value) => ({ type: 'slide', value }))),
};
void goodApp;

/** An accumulating container collects a union; createApp rejects a message outside Msg. */
const badApp: App<number, Msg> = {
  init: () => 0,
  update: () => {},
  // @ts-expect-error - 'nope' is not assignable to Msg
  view: () => Column.new().push(Button.new(Text.new('x')).onPress(() => ({ type: 'nope' }))),
};
void badApp;

/** A pinned container errors on the offending push itself. */
Row.new<Msg>()
  .push(Button.new(Text.new('ok')).onPress(() => ({ type: 'inc' })))
  // @ts-expect-error - message not in Msg
  .push(Button.new(Text.new('bad')).onPress(() => ({ type: 'nope' })));

/** Handler value parameters carry the widget's runtime type. */
Checkbox.new(false).onToggle((v) => {
  const _b: boolean = v;
  return { type: 'toggle', value: _b };
});
Slider.new(0, 1, 0, (v) => {
  const _n: number = v;
  return { type: 'slide', value: _n };
});

/** Single-child containers infer their content's message type. */
const scrollApp: App<null, Msg> = {
  init: () => null,
  update: () => {},
  view: () =>
    Scrollable.new(
      Container.new(Button.new(Text.new('+')).onPress(() => ({ type: 'inc' }))),
    ),
};
void scrollApp;

/** createApp returns a host-facing class with no message parameter. */
const Application = createApp<number, Msg>(goodApp);
void new Application();
