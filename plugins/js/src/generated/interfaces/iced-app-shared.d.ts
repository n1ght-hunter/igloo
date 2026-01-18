declare module 'iced:app/shared@0.1.0' {
  export type Pixels = number;
  /**
   * A color in the sRGB color space.
   */
  export interface Color {
    /**
     * Red component, 0.0 - 1.0
     */
    r: number,
    /**
     * Green component, 0.0 - 1.0
     */
    g: number,
    /**
     * Blue component, 0.0 - 1.0
     */
    b: number,
    /**
     * Alpha component, 0.0 - 1.0
     */
    a: number,
  }
  /**
   * # Variants
   * 
   * ## `"contain"`
   * 
   * ## `"cover"`
   * 
   * ## `"fill"`
   * 
   * ## `"none"`
   * 
   * ## `"scale-down"`
   */
  export type ContentFit = 'contain' | 'cover' | 'fill' | 'none' | 'scale-down';
  /**
   * # Variants
   * 
   * ## `"linear"`
   * 
   * ## `"nearest"`
   */
  export type FilterMethod = 'linear' | 'nearest';
  export type Rotation = RotationFloating | RotationSolid;
  export interface RotationFloating {
    tag: 'floating',
    val: number,
  }
  export interface RotationSolid {
    tag: 'solid',
    val: number,
  }
  
  export class Element implements Disposable {
    /**
     * This type does not have a public constructor.
     */
    private constructor();
    [Symbol.dispose](): void;
  }
}
