// Revision interval calculation logic
// Pure functions that can be tested independently of Tauri/UI

export type RecallResult = 'correct_clear' | 'correct_slow' | 'partial' | 'incorrect'

export const DEFAULT_INTERVALS = [1, 3, 7, 14, 30, 60]

export function nextInterval(
  currentInterval: number,
  result: RecallResult,
  _repetition: number
): number {
  const idx = DEFAULT_INTERVALS.indexOf(currentInterval)

  switch (result) {
    case 'correct_clear':
      // Advance to next interval
      if (idx >= 0 && idx < DEFAULT_INTERVALS.length - 1) {
        return DEFAULT_INTERVALS[idx + 1]
      }
      // Beyond last interval - extend by 30 days
      return currentInterval + 30
    case 'correct_slow':
      // Same or next shorter interval
      if (idx > 0) {
        return DEFAULT_INTERVALS[idx - 1]
      }
      return currentInterval
    case 'partial':
      // Schedule in 1-3 days
      return 3
    case 'incorrect':
      // Return to learning state, schedule in 1 day
      return 1
  }
}

export function shouldRepeatAtSameOrShorter(result: RecallResult): boolean {
  return result === 'correct_slow'
}

export function isAdvance(result: RecallResult): boolean {
  return result === 'correct_clear'
}

export function isPartial(result: RecallResult): boolean {
  return result === 'partial'
}

export function isIncorrect(result: RecallResult): boolean {
  return result === 'incorrect'
}
