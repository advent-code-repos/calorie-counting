mod controllers;

use controllers::calorie_controller::CalorieController;

fn main() {
    const PATH: &str = "./inputs/input.example";
    CalorieController::run(PATH);
}
