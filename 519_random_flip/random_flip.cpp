/*
 * @Date: 2021-12-04 05:46:57
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-04 05:51:33
 */

#include <cassert>
#include <cmath>
#include <cstdlib>
#include <ctime>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
 public:
  Solution(int m, int n) {
    this->m = m;
    this->n = n;
    total = m * n;
    bucketSize = sqrt(m * n);
    for (int i = 0; i < total; i += bucketSize) buckets.push_back({});

    srand(time(nullptr));
  }

  vector<int> flip() {
    int x = rand() % total;
    int sumZero = 0;
    int curr = 0;
    total--;

    for (auto& bucket : buckets) {
      int bucket_size = bucket.size();
      if (sumZero + bucketSize - bucket_size > x) {
        for (int i = 0; i < bucketSize; ++i) {
          if (!bucket.count(curr + i)) {
            if (sumZero == x) {
              bucket.emplace(curr + i);
              return {(curr + i) / n, (curr + i) % n};
            }
            sumZero++;
          }
        }
      }
      curr += bucketSize;
      sumZero += bucketSize - bucket.size();
    }

    return {};
  }

  void reset() {
    for (auto& bucket : buckets) bucket.clear();

    total = m * n;
  }

 private:
  int m;
  int n;
  int bucketSize;
  int total;
  vector<unordered_set<int>> buckets;
};

int main() {
  Solution* obj = new Solution(3, 1);
  vector<int> param_1 = obj->flip();
}