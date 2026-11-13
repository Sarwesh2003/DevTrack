import { describe, it, expect } from 'vitest'
import {
  nextInterval,
  DEFAULT_INTERVALS,
  shouldRepeatAtSameOrShorter,
  isAdvance,
  isPartial,
  isIncorrect,
} from '@/lib/revision'

describe('revision intervals', () => {
  describe('nextInterval', () => {
    it('advances to next interval on correct_clear', () => {
      expect(nextInterval(1, 'correct_clear', 1)).toBe(3)
      expect(nextInterval(3, 'correct_clear', 2)).toBe(7)
      expect(nextInterval(7, 'correct_clear', 3)).toBe(14)
      expect(nextInterval(14, 'correct_clear', 4)).toBe(30)
      expect(nextInterval(30, 'correct_clear', 5)).toBe(60)
    })

    it('extends beyond last interval by 30 days', () => {
      expect(nextInterval(60, 'correct_clear', 6)).toBe(90)
      expect(nextInterval(90, 'correct_clear', 7)).toBe(120)
    })

    it('repeats at same or shorter interval on correct_slow', () => {
      expect(nextInterval(7, 'correct_slow', 3)).toBe(3)
      expect(nextInterval(3, 'correct_slow', 2)).toBe(1)
      expect(nextInterval(1, 'correct_slow', 1)).toBe(1)
    })

    it('schedules partial in 3 days', () => {
      expect(nextInterval(30, 'partial', 5)).toBe(3)
      expect(nextInterval(1, 'partial', 1)).toBe(3)
    })

    it('returns incorrect to 1 day', () => {
      expect(nextInterval(60, 'incorrect', 10)).toBe(1)
      expect(nextInterval(1, 'incorrect', 1)).toBe(1)
    })
  })

  describe('DEFAULT_INTERVALS', () => {
    it('contains the expected intervals', () => {
      expect(DEFAULT_INTERVALS).toEqual([1, 3, 7, 14, 30, 60])
    })
  })

  describe('shouldRepeatAtSameOrShorter', () => {
    it('returns true for correct_slow', () => {
      expect(shouldRepeatAtSameOrShorter('correct_slow')).toBe(true)
    })

    it('returns false for other results', () => {
      expect(shouldRepeatAtSameOrShorter('correct_clear')).toBe(false)
      expect(shouldRepeatAtSameOrShorter('partial')).toBe(false)
      expect(shouldRepeatAtSameOrShorter('incorrect')).toBe(false)
    })
  })

  describe('isAdvance', () => {
    it('returns true only for correct_clear', () => {
      expect(isAdvance('correct_clear')).toBe(true)
      expect(isAdvance('correct_slow')).toBe(false)
      expect(isAdvance('partial')).toBe(false)
      expect(isAdvance('incorrect')).toBe(false)
    })
  })

  describe('isPartial', () => {
    it('returns true only for partial', () => {
      expect(isPartial('partial')).toBe(true)
      expect(isPartial('correct_clear')).toBe(false)
      expect(isPartial('incorrect')).toBe(false)
    })
  })

  describe('isIncorrect', () => {
    it('returns true only for incorrect', () => {
      expect(isIncorrect('incorrect')).toBe(true)
      expect(isIncorrect('correct_clear')).toBe(false)
      expect(isIncorrect('partial')).toBe(false)
    })
  })
})
