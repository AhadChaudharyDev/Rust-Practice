mod exercises;

fn main() {
    // Function call for common_programming_concept
    exercises::common_programming_concepts::hello_world::hello();
    exercises::common_programming_concepts::variables_and_mutability::variables();
    exercises::common_programming_concepts::data_types::start();
    exercises::common_programming_concepts::functions::fun();
    exercises::common_programming_concepts::control_flow::control_flow();

    // Function call for ownership_and_borrowing
    exercises::ownership_and_borrowing::ownership::ownership();
    exercises::ownership_and_borrowing::refrences_and_borrowing::borrowing();
    exercises::ownership_and_borrowing::the_slice_type::slice_type();  
}
