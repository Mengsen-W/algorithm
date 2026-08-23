// Package main ...
package main

func sumGame(num string) bool {
	n := len(num)

	get := func(s string) (int, int) {
		nn, qq := 0, 0
		for _, ch := range s {
			if ch == '?' {
				qq++
			} else {
				nn += int(ch - '0')
			}
		}
		return nn, qq
	}

	n0, q0 := get(num[:n/2])
	n1, q1 := get(num[n/2:])

	return ((q0+q1)%2 == 1) || (n0-n1 != (q1-q0)*9/2)
}

func main() {
	tests := []struct{
		num string
		ans bool
	}{
      {"5023", false},
      {"25??", true},
      {"?3295???", false},
	}

	for _, test := range tests {
		if sumGame(test.num) != test.ans {
			panic("wrong")
		}
	}
}