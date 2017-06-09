#include <iostream>
#include <cmath>
#include <iomanip>

double NormalCdf(double x, double mu, double sigma) {
    return 0.5 * (1. + std::erf((x - mu) / sigma * M_SQRT1_2));
}

int main() {
    std::cout << std::fixed << std::setprecision(4);
    std::cout << NormalCdf(9800, 205 * 49, sqrt(49) * 15) << std::endl;
    return 0;
}
