package tuple_too_large

// Keeper has a 5-element return tuple — exactly at the limit, should emit.
func Keeper() (int, int, int, int, int) {
	return 1, 2, 3, 4, 5
}

// KeeperWithError has a 5-element inner + trailing error, should emit as
// Result<(int, int, int, int, int), error>.
func KeeperWithError() (int, int, int, int, int, error) {
	return 1, 2, 3, 4, 5, nil
}

// Unpack6 mirrors samber/lo's shape: 6-element return, exceeds limit, skipped.
func Unpack6() (int, int, int, int, int, int) {
	return 1, 2, 3, 4, 5, 6
}

// Unpack9 is the concrete samber/lo case from bugs.md #8.
func Unpack9() (int, int, int, int, int, int, int, int, int) {
	return 1, 2, 3, 4, 5, 6, 7, 8, 9
}

// Unpack6Err: 6-element inner + trailing error. Inner wrap is too large, so
// the function should be skipped entirely (the SkipReason propagates out of
// the Result<..., error> wrap via `returns.go`).
func Unpack6Err() (int, int, int, int, int, int, error) {
	return 1, 2, 3, 4, 5, 6, nil
}
