/*
 * @Date: 2023-02-07
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-07
 * @FilePath: /algorithm/golang/1604_alert_names/alert_names.go
 */

package main

import (
	"reflect"
	"sort"
)

func alertNames(keyName, keyTime []string) (ans []string) {
	timeMap := map[string][]int{}
	for i, name := range keyName {
		t := keyTime[i]
		hour := int(t[0]-'0')*10 + int(t[1]-'0')
		minute := int(t[3]-'0')*10 + int(t[4]-'0')
		timeMap[name] = append(timeMap[name], hour*60+minute)
	}
	for name, times := range timeMap {
		sort.Ints(times)
		for i, t := range times[2:] {
			if t-times[i] <= 60 {
				ans = append(ans, name)
				break
			}
		}
	}
	sort.Strings(ans)
	return
}

func main() {
	assert := func(a, b []string) {
		if !reflect.DeepEqual(a, b) {
			panic("Not Passed")
		}
	}

	{
		keyName := []string{"daniel", "daniel", "daniel", "luis", "luis", "luis", "luis"}
		keyTime := []string{"10:00", "10:40", "11:00", "09:00", "11:00", "13:00", "15:00"}
		ans := []string{"daniel"}
		assert(alertNames(keyName, keyTime), ans)
	}

	{
		keyName := []string{"alice", "alice", "alice", "bob", "bob", "bob", "bob"}
		keyTime := []string{"12:01", "12:00", "18:00", "21:00", "21:20", "21:30", "23:00"}
		ans := []string{"bob"}
		assert(alertNames(keyName, keyTime), ans)
	}

	{
		keyName := []string{"john", "john", "john"}
		keyTime := []string{"23:58", "23:59", "00:01"}
		ans := []string{}
		assert(alertNames(keyName, keyTime), ans)
	}

	{
		keyName := []string{"leslie", "leslie", "leslie", "clare", "clare", "clare", "clare"}
		keyTime := []string{"13:00", "13:20", "14:00", "18:00", "18:51", "19:30", "19:49"}
		ans := []string{"clare", "leslie"}
		assert(alertNames(keyName, keyTime), ans)
	}
}
