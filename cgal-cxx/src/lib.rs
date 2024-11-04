use autocxx::prelude::*;

include_cpp!(
    #include "CGAL/Simple_cartesian.h"
    safety!(unsafe)
    generate!("Point_2")
);

#[test]
fn test() {
}