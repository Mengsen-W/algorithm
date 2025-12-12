package countmentions

import (
	"sort"
	"strconv"
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
)

func countMentions(numberOfUsers int, events [][]string) []int {
	sort.Slice(events, func(i, j int) bool {
		timeA, _ := strconv.Atoi(events[i][1])
		timeB, _ := strconv.Atoi(events[j][1])
		if timeA != timeB {
			return timeA < timeB
		}
		return events[i][0] != "MESSAGE" && events[j][0] == "MESSAGE"
	})

	count := make([]int, numberOfUsers)
	nextOnlineTime := make([]int, numberOfUsers)

	for _, event := range events {
		curTime, _ := strconv.Atoi(event[1])
		eventType := event[0]

		if eventType == "MESSAGE" {
			target := event[2]
			switch target {
			case "ALL":
				for i := 0; i < numberOfUsers; i++ {
					count[i]++
				}
			case "HERE":
				for i := 0; i < numberOfUsers; i++ {
					if nextOnlineTime[i] <= curTime {
						count[i]++
					}
				}
			default:
				users := strings.Split(target, " ")
				for _, user := range users {
					idx, _ := strconv.Atoi(user[2:])
					count[idx]++
				}
			}
		} else {
			idx, _ := strconv.Atoi(event[2])
			nextOnlineTime[idx] = curTime + 60
		}
	}

	return count
}

func main() {
	tests := []struct {
		numberOfUsers int
		events        [][]string
		expected      []int
	}{
		{2, [][]string{{"MESSAGE", "10", "id1 id0"}, {"OFFLINE", "11", "0"}, {"MESSAGE", "71", "HERE"}}, []int{2, 2}},
		{2, [][]string{{"MESSAGE", "10", "id1 id0"}, {"OFFLINE", "11", "0"}, {"MESSAGE", "12", "ALL"}}, []int{2, 2}},
		{2, [][]string{{"OFFLINE", "10", "0"}, {"MESSAGE", "12", "HERE"}}, []int{0, 1}},
	}

	for index, test := range tests {
		assert.Equal(&testing.T{}, test.expected, countMentions(test.numberOfUsers, test.events), index)
	}
}
