// Package main ...
package main

import "fmt"

type State struct {
	prev, curr, tight, lead int
	cnt, sum                int64
}

func solve(num int64) int64 {
	// 数字小于 3 位波动值为 0
	if num < 100 {
		return 0
	}
	s := fmt.Sprintf("%d", num)
	n := len(s)

	currStates := []State{{10, 10, 1, 1, 1, 0}}

	for pos := 0; pos < n; pos++ {
		limit := int(s[pos] - '0')
		cnt := [2][2][11][11]int64{}
		sum := [2][2][11][11]int64{}

		for _, st := range currStates {
			maxDigit := limit
			if st.tight == 0 {
				maxDigit = 9
			}
			for digit := 0; digit <= maxDigit; digit++ {
				newLead := 0
				if st.lead == 1 && digit == 0 {
					newLead = 1
				}
				newPrev := st.curr
				newCurr := digit
				if newLead == 1 {
					newCurr = 10
				}
				newTight := 0
				if st.tight == 1 && digit == maxDigit {
					newTight = 1
				}

				add := int64(0)
				// 已有三位有效数字时才计算波动（prev和curr都有效，且不是前导零）
				if newLead == 0 && st.prev != 10 && st.curr != 10 {
					if (st.prev < st.curr && st.curr > digit) ||
						(st.prev > st.curr && st.curr < digit) {
						add = st.cnt
					}
				}

				cnt[newTight][newLead][newPrev][newCurr] += st.cnt
				sum[newTight][newLead][newPrev][newCurr] += st.sum + add
			}
		}

		// 收集合法状态
		nextStates := []State{}
		for tight := 0; tight < 2; tight++ {
			for lead := 0; lead < 2; lead++ {
				for prev := 0; prev <= 10; prev++ {
					for curr := 0; curr <= 10; curr++ {
						c := cnt[tight][lead][prev][curr]
						sVal := sum[tight][lead][prev][curr]
						// 如果当前为有效状态，则进入下一轮计算
						if c != 0 {
							nextStates = append(nextStates, State{prev, curr, tight, lead, c, sVal})
						}
					}
				}
			}
		}
		currStates = nextStates
	}

	// 累加所有合法状态的波动值之和
	var ans int64 = 0
	for _, st := range currStates {
		ans += st.sum
	}
	return ans
}

func totalWaviness(num1 int64, num2 int64) int64 {
	return solve(num2) - solve(num1-1)
}

func main() {
	tests := []struct {
		num1 int64
		num2 int64
		ans  int64
	}{
		{120, 130, 3},
		{198, 202, 3},
		{4848, 4848, 2},
	}

	for _, test := range tests {
		if res := totalWaviness(test.num1, test.num2); res != test.ans {
			fmt.Printf("totalWaviness(%d, %d) = %d, want %d\n", test.num1, test.num2, res, test.ans)
		}
	}
}
