declare module 'iced:app/shared@0.1.0' {
  export type Pixels = number;
  
  export class Element implements Partial<Disposable> {
    /**
     * This type does not have a public constructor.
     */
    private constructor();
      [Symbol.dispose]?: () => void;
  }
}
