export function toISODate(date: Date): string {
  const year = date.getFullYear()
  const month = String(date.getMonth() + 1).padStart(2, '0')
  const day = String(date.getDate()).padStart(2, '0')
  return `${year}-${month}-${day}`
}

export function parseISODate(value: string): Date {
  const [year, month, day] = value.split('-').map(Number)
  return new Date(year, (month ?? 1) - 1, day ?? 1)
}

export function todayISO(): string {
  return toISODate(new Date())
}

export function addDaysISO(value: string, days: number): string {
  const date = parseISODate(value)
  date.setDate(date.getDate() + days)
  return toISODate(date)
}

/** Monday-based week start for the given date (defaults to today). */
export function weekStartISO(value: string = todayISO()): string {
  const date = parseISODate(value)
  const offset = (date.getDay() + 6) % 7
  date.setDate(date.getDate() - offset)
  return toISODate(date)
}

/** ISO weekday number: Monday = 1 ... Sunday = 7. */
export function isoWeekday(value: string): number {
  return ((parseISODate(value).getDay() + 6) % 7) + 1
}

export function formatDateHuman(value: string): string {
  return parseISODate(value).toLocaleDateString(undefined, {
    weekday: 'short',
    month: 'short',
    day: 'numeric',
  })
}

export const WEEKDAY_LABELS: Record<number, string> = {
  1: 'Mon',
  2: 'Tue',
  3: 'Wed',
  4: 'Thu',
  5: 'Fri',
  6: 'Sat',
  7: 'Sun',
}
