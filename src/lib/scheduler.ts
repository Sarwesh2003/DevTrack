// Task scoring heuristic
// task_score = priority_weight + overdue_weight + weakness_weight + dependency_weight + track_balance_weight

export type Priority = 'High' | 'Medium' | 'Low'

export interface TaskScoreInputs {
  priority: Priority
  overdueDays: number
  isWeak: boolean
  unblocksDependent: boolean
  trackNeglected: boolean
}

const PRIORITY_WEIGHTS: Record<Priority, number> = {
  High: 5,
  Medium: 3,
  Low: 1,
}

export function taskScore(inputs: TaskScoreInputs): number {
  let score = PRIORITY_WEIGHTS[inputs.priority]

  // Overdue revision: +1 to +4 depending on lateness
  if (inputs.overdueDays > 0) {
    score += Math.min(4, Math.max(1, Math.floor(inputs.overdueDays / 2)))
  }

  // Weak recall / failed previous attempt: +3
  if (inputs.isWeak) {
    score += 3
  }

  // Unblocks dependent topic: +2
  if (inputs.unblocksDependent) {
    score += 2
  }

  // Track balance: +1 when a track has been neglected too long
  if (inputs.trackNeglected) {
    score += 1
  }

  return score
}

export function rankTasksByScore(
  tasks: Array<{ id: string } & TaskScoreInputs>
): Array<{ id: string; score: number }> {
  return tasks
    .map((t) => ({ id: t.id, score: taskScore(t) }))
    .sort((a, b) => b.score - a.score)
}

export function fitsWithinCapacity(
  estimatedMinutes: number,
  availableMinutes: number,
  bufferRatio = 0.15
): boolean {
  const buffer = availableMinutes * bufferRatio
  return estimatedMinutes <= availableMinutes - buffer
}
