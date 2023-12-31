use desim_benchmark::simulation as desim_simulation;
use sim_v2::simulation as sim_v2_simulation;
use simrs_benchmark::simulation as simrs_simulation;
use simulator_benchmark::simulation as simulator_simulation;

fn main() {
    desim_simulation(50000.0);
    simrs_simulation(50000);
    simulator_simulation(50000);
    sim_v2_simulation(50000);
}
