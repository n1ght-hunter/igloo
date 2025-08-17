/// <reference path="./iced-app-button.d.ts" />
/// <reference path="./iced-app-checkbox.d.ts" />
/// <reference path="./iced-app-column.d.ts" />
/// <reference path="./iced-app-container.d.ts" />
/// <reference path="./iced-app-progress-bar.d.ts" />
/// <reference path="./iced-app-row.d.ts" />
/// <reference path="./iced-app-rule.d.ts" />
/// <reference path="./iced-app-shared.d.ts" />
/// <reference path="./iced-app-text.d.ts" />
/// <reference path="./iced-app-toggler.d.ts" />
/// <reference path="./iced-app-tooltip.d.ts" />
declare module 'iced:app/element@0.1.0' {
  export function textToElement(text: Text): Element;
  export function columnToElement(column: Column): Element;
  export function buttonToElement(button: Button): Element;
  export function rowToElement(row: Row): Element;
  export function containerToElement(container: Container): Element;
  export function tooltipToElement(tooltip: Tooltip): Element;
  export function ruleToElement(rule: Rule): Element;
  export function checkboxToElement(checkbox: Checkbox): Element;
  export function progressBarToElement(progressBar: ProgressBar): Element;
  export function togglerToElement(toggler: Toggler): Element;
  export type Element = import('iced:app/shared@0.1.0').Element;
  export type Text = import('iced:app/text@0.1.0').Text;
  export type Column = import('iced:app/column@0.1.0').Column;
  export type Row = import('iced:app/row@0.1.0').Row;
  export type Container = import('iced:app/container@0.1.0').Container;
  export type Tooltip = import('iced:app/tooltip@0.1.0').Tooltip;
  export type Button = import('iced:app/button@0.1.0').Button;
  export type Rule = import('iced:app/rule@0.1.0').Rule;
  export type Checkbox = import('iced:app/checkbox@0.1.0').Checkbox;
  export type ProgressBar = import('iced:app/progress-bar@0.1.0').ProgressBar;
  export type Toggler = import('iced:app/toggler@0.1.0').Toggler;
}
