#include <iostream>
#include <string_view>
#include <vector>
#include <fstream>
#include <algorithm>
#include <array>
#include <functional>
#include <iostream>
#include <string_view>

int main()
{
  std::ifstream f("input");
  std::vector<int> l;
  std::vector<int> r;
  std::string line;
  int total = 0;
  while (std::getline(f, line))
  {
    int a = std::stoi(line.substr(0, 5).c_str());
    l.push_back(a);

    int b = std::stoi(line.substr(8, 5).c_str());
    r.push_back(b);

  }
  std::sort(l.begin(), l.end());
  std::sort(r.begin(), r.end());
  for (int i = 0; i < l.size(); i++) {
    total += std::abs(l.at(i) - r.at(i));
  }
  std::cout << total;
  return 0;
}


