
#include "lib.rs.h"

#include <iostream>
#include <vector>

//using namespace org::environment;
using namespace std;
int main(){

    auto env = new_environment();
    std::cout <<env->print_acc()<<endl;
    std::vector<double> vec{2.0, 4.0};
    std::cout <<env->weighted_sum_vector(vec)<<endl;


    return 0;
}