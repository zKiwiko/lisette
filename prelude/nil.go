package lisette

import "reflect"

// IsNilInterface reports whether an interface value is nil or contains
// a nil pointer (typed nil). Returns true for both cases.
func IsNilInterface(x any) bool {
	if x == nil {
		return true
	}
	v := reflect.ValueOf(x)
	switch v.Kind() {
	case reflect.Pointer, reflect.Map, reflect.Chan, reflect.Func, reflect.Interface:
		return v.IsNil()
	default:
		return false
	}
}
