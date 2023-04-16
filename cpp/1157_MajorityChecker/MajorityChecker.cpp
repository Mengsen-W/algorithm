/*
 * @Date: 2023-04-16
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-16
 * @FilePath: /algorithm/cpp/1157_MajorityChecker/MajorityChecker.cpp
 */

#include <cassert>
#include <random>
#include <unordered_map>
#include <vector>

using namespace std;

class MajorityChecker {
 public:
  MajorityChecker(vector<int>& arr) : arr(arr) {
    for (int i = 0; i < arr.size(); ++i) {
      loc[arr[i]].push_back(i);
    }
  }

  int query(int left, int right, int threshold) {
    int length = right - left + 1;
    uniform_int_distribution<int> dis(left, right);

    for (int i = 0; i < k; ++i) {
      int x = arr[dis(gen)];
      vector<int>& pos = loc[x];
      int occ = upper_bound(pos.begin(), pos.end(), right) - lower_bound(pos.begin(), pos.end(), left);
      if (occ >= threshold) {
        return x;
      } else if (occ * 2 >= length) {
        return -1;
      }
    }

    return -1;
  }

 private:
  static constexpr int k = 20;

  const vector<int>& arr;
  unordered_map<int, vector<int>> loc;
  mt19937 gen{random_device{}()};
};

int main() {
  vector<int> arr{1, 1, 2, 2, 1, 1};
  MajorityChecker m{arr};
  assert(m.query(0, 5, 4) == 1);
  assert(m.query(0, 3, 3) == -1);
  assert(m.query(2, 3, 2) == 2);
}