// Re-export alignment types from WIT
export type {
  Horizontal as HorizontalType,
  Vertical as VerticalType,
  Alignment as AlignmentType,
} from 'iced:app/alignment@0.1.0';

/**
 * Horizontal alignment values.
 */
export enum Horizontal {
  Left = 'left',
  Center = 'center',
  Right = 'right',
}

/**
 * Vertical alignment values.
 */
export enum Vertical {
  Top = 'top',
  Center = 'center',
  Bottom = 'bottom',
}

/**
 * Generic alignment values.
 */
export enum Alignment {
  Start = 'start',
  Center = 'center',
  End = 'end',
}
