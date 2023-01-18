/*
 * @Date: 2022-06-09 09:47:37
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-09 09:51:27
 * @FilePath: /algorithm/497_random_points_in_non-overlapping_rectangles/random_points_in_non-overlapping_rectangles.cpp
 */

#include <random>
#include <vector>

using namespace std;

class Solution {
 public:
  Solution(vector<vector<int>> rects) : rects{rects} {
    this->arr.emplace_back(0);
    for (auto& rect : rects) {
      this->arr.emplace_back(arr.back() + (rect[2] - rect[0] + 1) * (rect[3] - rect[1] + 1));
    }
  }

  vector<int> pick() {
    uniform_int_distribution<int> dis(0, arr.back() - 1);
    int k = dis(gen) % arr.back();
    int rectIndex = upper_bound(arr.begin(), arr.end(), k) - arr.begin() - 1;
    k = k - arr[rectIndex];
    int a = rects[rectIndex][0], b = rects[rectIndex][1];
    int y = rects[rectIndex][3];
    int col = y - b + 1;
    int da = k / col;
    int db = k - col * da;
    return {a + da, b + db};
  }

 private:
  vector<int> arr;
  vector<vector<int>> rects;
  mt19937 gen{random_device{}()};
};

int main() {
  Solution solution = Solution({{-2, -2, 1, 1}, {2, 2, 4, 6}});
  solution.pick();
}