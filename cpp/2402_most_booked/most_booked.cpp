#include <algorithm>
#include <cassert>
#include <queue>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int mostBooked(int n, vector<vector<int>>& meetings) {
    sort(meetings.begin(), meetings.end(),
         [](const vector<int>& v1, const vector<int>& v2) -> bool { return v1[0] < v2[0]; });
    priority_queue<int, vector<int>, greater<int>> availRooms;
    for (int i = 0; i < n; i++) {
      availRooms.push(i);
    }
    priority_queue<pair<long long, int>, vector<pair<long long, int>>, greater<>> usedRooms;
    vector<int> usedCount(n);
    long long curTime = 0;
    for (const auto& meeting : meetings) {
      curTime = max(curTime, static_cast<long long>(meeting[0]));
      if (availRooms.empty()) {
        curTime = max(curTime, usedRooms.top().first);
      }
      while (!usedRooms.empty() && usedRooms.top().first <= curTime) {
        availRooms.push(usedRooms.top().second);
        usedRooms.pop();
      }
      int room = availRooms.top();
      availRooms.pop();
      usedCount[room]++;
      usedRooms.push({curTime + meeting[1] - meeting[0], room});
    }
    int room = 0;
    for (int i = 0; i < n; i++) {
      if (usedCount[i] > usedCount[room]) {
        room = i;
      }
    }
    return room;
  }
};

int main() {
  vector<tuple<int, vector<vector<int>>, int>> tests{
      {2, {{0, 10}, {1, 5}, {2, 7}, {3, 4}}, 0},
      {3, {{1, 20}, {2, 10}, {3, 5}, {4, 9}, {6, 8}}, 1},
  };

  for (auto& [n, meetings, ans] : tests) {
    assert(Solution().mostBooked(n, meetings) == ans);
  }
}