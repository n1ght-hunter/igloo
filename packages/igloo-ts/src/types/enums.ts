/**
 * How content should fit within its bounds.
 */
export enum ContentFit {
  /** Scale to fit inside bounds while maintaining aspect ratio */
  Contain = 'contain',
  /** Scale to cover bounds while maintaining aspect ratio */
  Cover = 'cover',
  /** Stretch to fill bounds exactly */
  Fill = 'fill',
  /** No scaling */
  None = 'none',
  /** Scale down only if larger than bounds */
  ScaleDown = 'scale-down',
}

/**
 * Image filtering method for scaling.
 */
export enum FilterMethod {
  /** Smooth interpolation (bilinear) */
  Linear = 'linear',
  /** Sharp pixel interpolation */
  Nearest = 'nearest',
}

/**
 * Anchor position for scrollbar alignment.
 */
export enum Anchor {
  Start = 'start',
  End = 'end',
}

/**
 * Position for tooltip placement.
 */
export enum Position {
  Top = 'top',
  Bottom = 'bottom',
  Left = 'left',
  Right = 'right',
  FollowCursor = 'follow-cursor',
}
