// Re-export alignment types from WIT
export type {
  Horizontal as HorizontalType,
  Vertical as VerticalType,
  Alignment as AlignmentType,
} from 'iced:app/alignment@0.1.0';

/**
 * Horizontal alignment values.
 */
export const Horizontal = {
  left: 'left' as const,
  center: 'center' as const,
  right: 'right' as const,
} as const;

/**
 * Vertical alignment values.
 */
export const Vertical = {
  top: 'top' as const,
  center: 'center' as const,
  bottom: 'bottom' as const,
} as const;

/**
 * Generic alignment values.
 */
export const Alignment = {
  start: 'start' as const,
  center: 'center' as const,
  end: 'end' as const,
} as const;
