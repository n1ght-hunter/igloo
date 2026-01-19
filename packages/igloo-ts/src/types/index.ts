// Type helpers
export { Length, type WitLength } from './length.js';
export { Padding, type WitPadding } from './padding.js';
export { Color, type WitColor } from './color.js';
export {
  Horizontal,
  Vertical,
  Alignment,
  type HorizontalType,
  type VerticalType,
  type AlignmentType,
} from './alignment.js';

// Enums
export { ContentFit, FilterMethod, Anchor, Position } from './enums.js';

// Re-export common types from WIT
export type { Pixels, Rotation } from 'iced:app/shared@0.1.0';
export type { LineHeight, Shaping, Wrapping, Alignment as TextAlignment } from 'iced:app/text@0.1.0';
export type { MessageId, Message } from 'iced:app/message@0.1.0';
