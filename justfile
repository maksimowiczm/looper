project_dir := justfile_directory()
build_release_dir := project_dir + "/cmake-build-release"
build_debug_dir := project_dir + "/cmake-build-debug"
def_iterations := "100"
def_population_size := "50"
def_crossover_prob := "0.9"
def_mutation := "rand/1/bin"
def_mutation_factor := "0.8"
def_function := "rastrigin"
def_verbose := "finished"
def_goal := "min"
def_variables := "0 3 3 5"

# Default recipe: show usage
default:
    @echo "Usage:"
    @echo "  just build-debug     # Build the project in debug mode"
    @echo "  just build-release   # Build the project in release mode"
    @echo "  just clean           # Clean up build artifacts"
    @echo "  just run-debug [iterations] [population_size] [crossover_prob] [mutation] [function] [mutation_factor] [verbose] [variables]"
    @echo "                       # Run the looper binary in debug mode with optional parameters"
    @echo "  just run-release [iterations] [population_size] [crossover_prob] [mutation] [function] [mutation_factor] [verbose] [variables]"
    @echo "                       # Run the looper binary in release mode with optional parameters"
    @echo ""
    @echo "  [iterations]         # Number of iterations"
    @echo "  [population_size]    # Number of individuals in the population (preferably 50)"
    @echo "  [crossover_prob]     # Probability of crossover (0.0 to 1.0)"
    @echo "  [mutation]           # Mutation strategy (rand/1/bin, rand/1/bin, best/2/bin, current/1/bin etc.)"
    @echo "  [function]           # Function to optimize (0:rastrigin, 1:griewank, 2:sphere, 3:rosenbrock, 4:ackley, 5:schwefel, 6:levy, 7:beale, 8:michalewicz, 9:booth)"
    @echo "  [mutation_factor]    # Mutation factor (0.0 to 2.0)"
    @echo "  [verbose]            # Verbosity level (none, iteration, finished)"

# Build the project in debug mode
build-debug:
    @cargo build

# Build the project in release mode
build-release:
    @cargo build --release

# Clean up build artifacts
clean:
    @cargo clean

# Run the looper binary in debug mode
run-debug mutation=def_mutation function=def_function variables=def_variables iterations=def_iterations population_size=def_population_size crossover_prob=def_crossover_prob mutation_factor=def_mutation_factor verbose=def_verbose goal=def_goal:
    @cargo run \
        --package looper \
        --bin looper -- \
        --iterations {{ iterations }} \
        --population-size {{ population_size }} \
        --crossover-probability {{ crossover_prob }} \
        --mutation {{ mutation }} \
        --mutation-factor {{ mutation_factor }} \
        --function {{ function }} \
        --verbose {{ verbose }} \
        --goal {{ goal }} \
        --variables {{ variables }}

# Run the looper binary in release mode
run-release mutation=def_mutation function=def_function variables=def_variables iterations=def_iterations population_size=def_population_size crossover_prob=def_crossover_prob mutation_factor=def_mutation_factor verbose=def_verbose goal=def_goal:
    @cargo run \
        --quiet \
        --release \
        --package looper \
        --bin looper -- \
        --iterations {{ iterations }} \
        --population-size {{ population_size }} \
        --crossover-probability {{ crossover_prob }} \
        --mutation {{ mutation }} \
        --mutation-factor {{ mutation_factor }} \
        --function {{ function }} \
        --verbose {{ verbose }} \
        --goal {{ goal }} \
        --variables {{ variables }}
