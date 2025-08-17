/// <reference path="./interfaces/iced-app-alignment.d.ts" />
/// <reference path="./interfaces/iced-app-button.d.ts" />
/// <reference path="./interfaces/iced-app-checkbox.d.ts" />
/// <reference path="./interfaces/iced-app-column.d.ts" />
/// <reference path="./interfaces/iced-app-container.d.ts" />
/// <reference path="./interfaces/iced-app-element.d.ts" />
/// <reference path="./interfaces/iced-app-length.d.ts" />
/// <reference path="./interfaces/iced-app-message.d.ts" />
/// <reference path="./interfaces/iced-app-padding.d.ts" />
/// <reference path="./interfaces/iced-app-progress-bar.d.ts" />
/// <reference path="./interfaces/iced-app-row.d.ts" />
/// <reference path="./interfaces/iced-app-rule.d.ts" />
/// <reference path="./interfaces/iced-app-shared.d.ts" />
/// <reference path="./interfaces/iced-app-text.d.ts" />
/// <reference path="./interfaces/iced-app-toggler.d.ts" />
/// <reference path="./interfaces/iced-app-tooltip.d.ts" />
declare module 'iced:app/app@0.1.0' {
  export type Element = import('iced:app/shared@0.1.0').Element;
  export type Message = import('iced:app/message@0.1.0').Message;
  export type * as IcedAppAlignment010 from 'iced:app/alignment@0.1.0'; // import iced:app/alignment@0.1.0
  export type * as IcedAppButton010 from 'iced:app/button@0.1.0'; // import iced:app/button@0.1.0
  export type * as IcedAppCheckbox010 from 'iced:app/checkbox@0.1.0'; // import iced:app/checkbox@0.1.0
  export type * as IcedAppColumn010 from 'iced:app/column@0.1.0'; // import iced:app/column@0.1.0
  export type * as IcedAppContainer010 from 'iced:app/container@0.1.0'; // import iced:app/container@0.1.0
  export type * as IcedAppElement010 from 'iced:app/element@0.1.0'; // import iced:app/element@0.1.0
  export type * as IcedAppLength010 from 'iced:app/length@0.1.0'; // import iced:app/length@0.1.0
  export type * as IcedAppMessage010 from 'iced:app/message@0.1.0'; // import iced:app/message@0.1.0
  export type * as IcedAppPadding010 from 'iced:app/padding@0.1.0'; // import iced:app/padding@0.1.0
  export type * as IcedAppProgressBar010 from 'iced:app/progress-bar@0.1.0'; // import iced:app/progress-bar@0.1.0
  export type * as IcedAppRow010 from 'iced:app/row@0.1.0'; // import iced:app/row@0.1.0
  export type * as IcedAppRule010 from 'iced:app/rule@0.1.0'; // import iced:app/rule@0.1.0
  export type * as IcedAppShared010 from 'iced:app/shared@0.1.0'; // import iced:app/shared@0.1.0
  export type * as IcedAppText010 from 'iced:app/text@0.1.0'; // import iced:app/text@0.1.0
  export type * as IcedAppToggler010 from 'iced:app/toggler@0.1.0'; // import iced:app/toggler@0.1.0
  export type * as IcedAppTooltip010 from 'iced:app/tooltip@0.1.0'; // import iced:app/tooltip@0.1.0
  export * as message from 'iced:app/message@0.1.0'; // export iced:app/message@0.1.0
  export function update(message: Message): void;
  /**
  * Exported function for creating a view
  */
  export function view(): Element;
}
