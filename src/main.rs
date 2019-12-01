mod day_1;

fn main() {
    let (payload_fuel, total_fuel) = day_1::get_fuel_mass_compensated("res/day1.txt");
    println!(
        "Day 1-1: Payload fuel: {}, Total fuel: {}",
        payload_fuel, total_fuel
    );
}
