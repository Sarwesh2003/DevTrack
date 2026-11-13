import { describe, it, expect } from 'vitest'
import { taskScore, rankTasksByScore, fitsWithinCapacity } from '@/lib/scheduler'

describe('taskScore', () => {
  it('computes base priority weight', () => {
    expect(taskScore({ priority: 'High', overdueDays: 0, isWeak: false, unblocksDependent: false, trackNeglected: false })).toBe(5)
    expect(taskScore({ priority: 'Medium', overdueDays: 0, isWeak: false, unblocksDependent: false, trackNeglected: false })).toBe(3)
    expect(taskScore({ priority: 'Low', overdueDays: 0, isWeak: false, unblocksDependent: false, trackNeglected: false })).toBe(1)
  })

  it('adds overdue weight', () => {
    const base = taskScore({ priority: 'Medium', overdueDays: 0, isWeak: false, unblocksDependent: false, trackNeglected: false })
    const overdue1 = taskScore({ priority: 'Medium', overdueDays: 1, isWeak: false, unblocksDependent: false, trackNeglected: false })
    const overdue5 = taskScore({ priority: 'Medium', overdueDays: 5, isWeak: false, unblocksDependent: false, trackNeglected: false })
    expect(overdue1).toBe(base + 1)
    expect(overdue5).toBe(base + 2)
    expect(overdue5).toBeGreaterThan(base)
  })

  it('adds weakness weight', () => {
    const base = taskScore({ priority: 'Medium', overdueDays: 0, isWeak: false, unblocksDependent: false, trackNeglected: false })
    const weak = taskScore({ priority: 'Medium', overdueDays: 0, isWeak: true, unblocksDependent: false, trackNeglected: false })
    expect(weak).toBe(base + 3)
  })

  it('adds dependency weight', () => {
    const base = taskScore({ priority: 'Medium', overdueDays: 0, isWeak: false, unblocksDependent: false, trackNeglected: false })
    const unblocks = taskScore({ priority: 'Medium', overdueDays: 0, isWeak: false, unblocksDependent: true, trackNeglected: false })
    expect(unblocks).toBe(base + 2)
  })

  it('adds track balance weight', () => {
    const base = taskScore({ priority: 'Medium', overdueDays: 0, isWeak: false, unblocksDependent: false, trackNeglected: false })
    const neglected = taskScore({ priority: 'Medium', overdueDays: 0, isWeak: false, unblocksDependent: false, trackNeglected: true })
    expect(neglected).toBe(base + 1)
  })

  it('combines all weights', () => {
    const result = taskScore({
      priority: 'High',
      overdueDays: 10,
      isWeak: true,
      unblocksDependent: true,
      trackNeglected: true,
    })
    expect(result).toBe(5 + 4 + 3 + 2 + 1) // 15
  })
})

describe('rankTasksByScore', () => {
  it('ranks tasks by score descending', () => {
    const tasks = [
      { id: 'a', priority: 'Low' as const, overdueDays: 0, isWeak: false, unblocksDependent: false, trackNeglected: false },
      { id: 'b', priority: 'High' as const, overdueDays: 0, isWeak: false, unblocksDependent: false, trackNeglected: false },
      { id: 'c', priority: 'Medium' as const, overdueDays: 0, isWeak: false, unblocksDependent: false, trackNeglected: false },
    ]
    const ranked = rankTasksByScore(tasks)
    expect(ranked[0].id).toBe('b')
    expect(ranked[1].id).toBe('c')
    expect(ranked[2].id).toBe('a')
    expect(ranked[0].score).toBeGreaterThan(ranked[1].score)
    expect(ranked[1].score).toBeGreaterThan(ranked[2].score)
  })
})

describe('fitsWithinCapacity', () => {
  it('returns true when estimate fits with buffer', () => {
    expect(fitsWithinCapacity(45, 60)).toBe(true) // 45 <= 60 - 9
    expect(fitsWithinCapacity(50, 60)).toBe(true) // 50 <= 60 - 9 = 51
  })

  it('returns false when estimate exceeds capacity minus buffer', () => {
    expect(fitsWithinCapacity(52, 60)).toBe(false) // 52 > 60 - 9 = 51
    expect(fitsWithinCapacity(60, 60)).toBe(false) // 60 > 60 - 9 = 51
  })

  it('respects custom buffer ratio', () => {
    expect(fitsWithinCapacity(50, 60, 0)).toBe(true) // no buffer
    expect(fitsWithinCapacity(50, 60, 0.2)).toBe(false) // 50 > 60 - 12 = 48
  })
})
