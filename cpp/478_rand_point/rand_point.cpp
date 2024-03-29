/*
 * @Date: 2022-06-05 10:22:07
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-05 10:30:53
 * @FilePath: /algorithm/478_rand_point/rand_point.cpp
 */

#include <random>

using namespace std;

class Solution {
 private:
  mt19937 gen{random_device{}()};
  uniform_real_distribution<double> dis;
  double xc, yc, r;

 public:
  Solution(double radius, double x_center, double y_center) : dis(0, 1), xc(x_center), yc(y_center), r(radius) {}

  vector<double> randPoint() {
    double u = dis(gen), theta = dis(gen) * 2 * acos(-1.0);
    double r = sqrt(u);
    return {xc + r * cos(theta) * this->r, yc + r * sin(theta) * this->r};
  }
};

int main() {}