import * as addon from 'native';

export class Ammonia {
  private cleaner: addon.Cleaner;

  /**
   * Build a Cleaner with the given options
   */
  public constructor(options?: Partial<addon.BuilderOptions>) {
    this.cleaner = addon.build(options || {});
  }

  /**
   * Sanitizes an HTML fragment in a string according to the configured options.
   */
  public clean(input: string): string {
    return addon.clean(this.cleaner, input);
  }
}

/**
 * Sanitizes an HTML fragment in a string according to the given options.
 */
export function clean(input: string, options?: Partial<addon.BuilderOptions>): string {
  const cleaner = addon.build(options || {});
  return addon.clean(cleaner, input);
}
