function func(ns: number | string) {
  return ns > 4; // Only in error in tsc 5.0
}
