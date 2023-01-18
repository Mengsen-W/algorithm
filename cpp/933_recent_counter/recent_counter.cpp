/*
 * @Date: 2022-05-06 07:13:29
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-06 07:19:24
 * @FilePath: /algorithm/933_recent_counter/recent_counter.cpp
 */

#include <cassert>
#include <queue>

using namespace std;

class RecentCounter {
  queue<int> q;

 public:
  RecentCounter() {}

  int ping(int t) {
    q.push(t);
    while (q.front() < t - 3000) {
      q.pop();
    }
    return q.size();
  }
};

int main() {
  RecentCounter rc;
  assert(rc.ping(1) == 1);
  assert(rc.ping(100) == 2);
  assert(rc.ping(3001) == 3);
  assert(rc.ping(3002) == 3);
}