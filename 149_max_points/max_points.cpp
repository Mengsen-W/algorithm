/*
 * @Date: 2021-06-24 08:54:02
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-24 09:02:05
 */

#include <cassert>
#include <functional>
#include <unordered_map>
#include <vector>

using namespace std;

// int gcd(int a, int b) { return b ? gcd(b, a % b) : a; }

int maxPoints(vector<vector<int>>& points) {
  function<int(int, int)> gcd = [&](int a, int b) -> int {
    return b ? gcd(b, a % b) : a;
  };
  int n = points.size();
  if (n <= 2) {
    return n;
  }
  int ret = 0;
  for (int i = 0; i < n; i++) {
    if (ret >= n - i || ret > n / 2) {
      break;
    }
    unordered_map<int, int> mp;
    for (int j = i + 1; j < n; j++) {
      int x = points[i][0] - points[j][0];
      int y = points[i][1] - points[j][1];
      if (x == 0) {
        y = 1;
      } else if (y == 0) {
        x = 1;
      } else {
        if (y < 0) {
          x = -x;
          y = -y;
        }
        int gcdXY = gcd(abs(x), abs(y));
        x /= gcdXY, y /= gcdXY;
      }
      mp[y + x * 20001]++;
    }
    int maxn = 0;
    for (auto& [_, num] : mp) {
      maxn = max(maxn, num + 1);
    }
    ret = max(ret, maxn);
  }
  return ret;
}

int main() {
  {
    vector<vector<int>> points{{1, 1}, {2, 2}, {3, 3}};
    assert(maxPoints(points) == 3);
  }
  {
    vector<vector<int>> points{{1, 1}, {3, 2}, {5, 3}, {4, 1}, {2, 3}, {1, 4}};
    assert(maxPoints(points) == 4);
  }
}