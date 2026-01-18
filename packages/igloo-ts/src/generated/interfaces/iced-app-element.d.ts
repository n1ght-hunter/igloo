/// <reference path="./iced-app-button.d.ts" />
/// <reference path="./iced-app-checkbox.d.ts" />
/// <reference path="./iced-app-column.d.ts" />
/// <reference path="./iced-app-combo-box.d.ts" />
/// <reference path="./iced-app-container.d.ts" />
/// <reference path="./iced-app-float.d.ts" />
/// <reference path="./iced-app-grid.d.ts" />
/// <reference path="./iced-app-image.d.ts" />
/// <reference path="./iced-app-keyed.d.ts" />
/// <reference path="./iced-app-markdown.d.ts" />
/// <reference path="./iced-app-pane-grid.d.ts" />
/// <reference path="./iced-app-pick-list.d.ts" />
/// <reference path="./iced-app-progress-bar.d.ts" />
/// <reference path="./iced-app-radio.d.ts" />
/// <reference path="./iced-app-row.d.ts" />
/// <reference path="./iced-app-rule.d.ts" />
/// <reference path="./iced-app-scrollable.d.ts" />
/// <reference path="./iced-app-shared.d.ts" />
/// <reference path="./iced-app-slider.d.ts" />
/// <reference path="./iced-app-space.d.ts" />
/// <reference path="./iced-app-svg.d.ts" />
/// <reference path="./iced-app-table.d.ts" />
/// <reference path="./iced-app-text-input.d.ts" />
/// <reference path="./iced-app-text.d.ts" />
/// <reference path="./iced-app-toggler.d.ts" />
/// <reference path="./iced-app-tooltip.d.ts" />
/// <reference path="./iced-app-vertical-slider.d.ts" />
declare module 'iced:app/element@0.1.0' {
  export function explain(element: Element, color: Color): Element;
  export function textToElement(text: Text): Element;
  export function columnToElement(column: Column): Element;
  export function buttonToElement(button: Button): Element;
  export function rowToElement(row: Row): Element;
  export function containerToElement(container: Container): Element;
  export function tooltipToElement(tooltip: Tooltip): Element;
  export function ruleToElement(rule: Rule): Element;
  export function checkboxToElement(checkbox: Checkbox): Element;
  export function comboBoxToElement(comboBox: ComboBox): Element;
  export function floatToElement(float: Float): Element;
  export function gridToElement(grid: Grid): Element;
  export function progressBarToElement(progressBar: ProgressBar): Element;
  export function togglerToElement(toggler: Toggler): Element;
  export function radioToElement(radio: Radio): Element;
  export function imageToElement(image: Image): Element;
  export function keyedColumnToElement(keyedColumn: KeyedColumn): Element;
  export function markdownToElement(markdown: Markdown): Element;
  export function paneGridToElement(paneGrid: PaneGrid): Element;
  export function pickListToElement(pickList: PickList): Element;
  export function sliderToElement(slider: Slider): Element;
  export function verticalSliderToElement(verticalSlider: VerticalSlider): Element;
  export function svgToElement(svg: Svg): Element;
  export function tableToElement(table: Table): Element;
  /**
   * text-editor-to-element: func(text-editor: text-editor) -> element;
   */
  export function textInputToElement(textInput: TextInput): Element;
  export function spaceToElement(space: Space): Element;
  export function scrollableToElement(scrollable: Scrollable): Element;
  export type Element = import('iced:app/shared@0.1.0').Element;
  export type Text = import('iced:app/text@0.1.0').Text;
  export type Column = import('iced:app/column@0.1.0').Column;
  export type Row = import('iced:app/row@0.1.0').Row;
  export type Container = import('iced:app/container@0.1.0').Container;
  export type Tooltip = import('iced:app/tooltip@0.1.0').Tooltip;
  export type Button = import('iced:app/button@0.1.0').Button;
  export type Rule = import('iced:app/rule@0.1.0').Rule;
  export type Checkbox = import('iced:app/checkbox@0.1.0').Checkbox;
  export type ComboBox = import('iced:app/combo-box@0.1.0').ComboBox;
  export type Float = import('iced:app/float@0.1.0').Float;
  export type Grid = import('iced:app/grid@0.1.0').Grid;
  export type ProgressBar = import('iced:app/progress-bar@0.1.0').ProgressBar;
  export type Toggler = import('iced:app/toggler@0.1.0').Toggler;
  export type Radio = import('iced:app/radio@0.1.0').Radio;
  export type Image = import('iced:app/image@0.1.0').Image;
  export type KeyedColumn = import('iced:app/keyed@0.1.0').KeyedColumn;
  export type Markdown = import('iced:app/markdown@0.1.0').Markdown;
  export type PaneGrid = import('iced:app/pane-grid@0.1.0').PaneGrid;
  export type PickList = import('iced:app/pick-list@0.1.0').PickList;
  export type Slider = import('iced:app/slider@0.1.0').Slider;
  export type VerticalSlider = import('iced:app/vertical-slider@0.1.0').VerticalSlider;
  export type Svg = import('iced:app/svg@0.1.0').Svg;
  export type Table = import('iced:app/table@0.1.0').Table;
  export type TextInput = import('iced:app/text-input@0.1.0').TextInput;
  export type Space = import('iced:app/space@0.1.0').Space;
  export type Scrollable = import('iced:app/scrollable@0.1.0').Scrollable;
  export type Color = import('iced:app/shared@0.1.0').Color;
}
