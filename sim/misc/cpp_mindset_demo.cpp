


struct Vec3 {
    float x = 0;
    float y = 0;
    float z = 0;
};






class Node{
    public:
        Vec3 sum_forces;
        // Other members
};


class Spring{
    Node * node1;
    Node * node2;
    // Other members

    void do_physics() {
        // do physics and calculate the force on each atom
        Vec3 some_force_vector = {4235.0, 2423525.123, 155.656};

        node1->sum_forces += 0.5*some_force_vector;
        node2->sum_forces += -0.5*some_force_vector;
    }
};

