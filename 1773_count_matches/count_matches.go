/*
 * @Date: 2022-10-29
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-29
 * @FilePath: /algorithm/1773_count_matches/count_matches.go
 */

package main

func countMatches(items [][]string, ruleKey, ruleValue string) (ans int) {
	d := map[string]int{"type": 0, "color": 1, "name": 2}
	index := d[ruleKey]
	for _, item := range items {
		if item[index] == ruleValue {
			ans++
		}
	}
	return
}

func main() {
	assert := func(b bool) {
		if !b {
			panic("Not Passed")
		}
	}

	{
		items := [][]string{{"phone", "blue", "pixel"}, {"computer", "silver", "lenovo"}, {"phone", "gold", "iphone"}}
		ruleKey := "color"
		ruleValue := "silver"
		ans := 1
		assert(countMatches(items, ruleKey, ruleValue) == ans)
	}

	{
		items := [][]string{{"phone", "blue", "pixel"}, {"computer", "silver", "phone"}, {"phone", "gold", "iphone"}}
		ruleKey := "type"
		ruleValue := "phone"
		ans := 2
		assert(countMatches(items, ruleKey, ruleValue) == ans)
	}
}
