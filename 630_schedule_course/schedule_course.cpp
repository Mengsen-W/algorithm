/*
 * @Date: 2021-12-14 05:54:56
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-14 06:07:19
 */

#include <cassert>
#include <queue>
#include <vector>

using namespace std;

class Solution {
 public:
  int scheduleCourse(vector<vector<int>> courses) {
    sort(courses.begin(), courses.end(),
         [](const auto& c0, const auto& c1) { return c0[1] < c1[1]; });

    priority_queue<int> q;
    // 优先队列中所有课程的总时间
    int total = 0;

    for (const auto& course : courses) {
      int ti = course[0], di = course[1];
      if (total + ti <= di) {
        total += ti;
        q.push(ti);
      } else if (!q.empty() && q.top() > ti) {
        total -= q.top() - ti;
        q.pop();
        q.push(ti);
      }
    }

    return q.size();
  }
};

int main() {
  assert(Solution().scheduleCourse(vector<vector<int>>{
             {100, 200}, {200, 1300}, {1000, 1250}, {2000, 3200}}) == 3);
  assert(Solution().scheduleCourse(vector<vector<int>>{{1, 2}}) == 1);
  assert(Solution().scheduleCourse(vector<vector<int>>{{3, 2},{4,3}}) == 0);
}