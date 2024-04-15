#ifndef __data2Tree__
#define __data2Tree__

#include <vector>

class Point {
    public:
        int x = 0;
        int y = 0;
};


class PointVector {
    public:
        std::vector<int> x;
        std::vector<int> y;
};

class TwoPoints {
    public:
        Point p1;
        Point p2;
};

class SeveralPoints {
    public:
        std::vector<Point> points;
};


#endif
