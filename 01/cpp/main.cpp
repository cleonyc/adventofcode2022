#include <iostream>
#include <fstream>
#include <vector>
#include <algorithm>
#include <numeric>

using namespace std;

int main()
{
    ifstream input_file;
    input_file.open("input.txt");

    if (input_file.is_open())
    {
        string line;
        vector<long> elf_vals = {0};
        long i = 0;
        while (getline(input_file, line))
        {
            if (line.empty())
            {
                elf_vals.push_back(0);
                ++i;
                continue;
            }
            long calories = stoi(line);
            elf_vals[i] += calories;
        }
        long part1 = *max_element(elf_vals.begin(), elf_vals.end());
        cout << "part 1: " << part1 << endl;
        sort(elf_vals.begin(), elf_vals.end());
        long part2 = accumulate(elf_vals.end() - 3, elf_vals.end(), 0);
        cout << "part 2: " << part2 << endl;
    }
}