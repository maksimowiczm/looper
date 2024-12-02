import argparse
import pandas as pd
import matplotlib.pyplot as plt
import numpy as np

# Define Rastrigin function (for a single variable)
def rastrigin(X, A=10):
    if len(X) == 1:
        x = X[0]
        return A + x**2 - A * np.cos(2 * np.pi * x)
    else:
        return A * len(X) + np.sum([x**2 - A * np.cos(2 * np.pi * x) for x in X])

# Define another example function (e.g., Sphere function)
def sphere(X):
    return np.sum(np.square(X))

def parse_arguments():
    parser = argparse.ArgumentParser(
        description="Plot population data from a CSV file with a given function."
    )
    parser.add_argument("csv_file", type=str, help="Path to the CSV file.")
    parser.add_argument(
        "--function",
        type=str,
        choices=["rastrigin", "sphere"],
        default="rastrigin",
        help="Objective function to plot.",
    )
    parser.add_argument(
        "--show", action="store_true", help="Display the plot in a window."
    )
    parser.add_argument(
        "--output",
        type=str,
        help="File path to save the plot (e.g., plot.png).",
        default="plot.png",
    )
    parser.add_argument(
        "--x-bound",
        type=float,
        nargs=2,
        default=[-5, 5],
        help="X-axis boundaries (e.g., --x_bound -5 5).",
    )
    parser.add_argument(
        "--y-bound",
        type=float,
        nargs=2,
        default=[-5, 5],
        help="Y-axis boundaries (e.g., --y_bound -5 5).",
    )
    parser.add_argument(
        "--max-iter",
        type=int,
        nargs=1,
        default=10,
        help="Number of iterations to display.",
    )
    return parser.parse_args()

def read_population_data(csv_file):
    return pd.read_csv(csv_file)

def get_function_by_name(function_name):
    if function_name == "rastrigin":
        return rastrigin
    elif function_name == "sphere":
        return sphere
    else:
        raise ValueError(f"Unknown function: {function_name}")

def plot_function_with_individuals(data, function_name, x_bound, y_bound, max_iterations):
    objective_function = get_function_by_name(function_name)
    num_variables = len([col for col in data.columns if col.startswith("var_")])

    # Limit the number of iterations to plot to 10
    unique_iterations = data["iteration"].unique()
    max_iterations = 10
    if len(unique_iterations) > max_iterations:
        sampled_iterations = np.linspace(0, len(unique_iterations) - 1, max_iterations - 1, dtype=int)
        sampled_iterations = np.append(sampled_iterations, len(unique_iterations) - 1)  # Ensure last iteration
        iterations_to_plot = unique_iterations[sampled_iterations]
    else:
        iterations_to_plot = unique_iterations

    # Filter data for selected iterations
    data = data[data["iteration"].isin(iterations_to_plot)]

    if num_variables == 1:
        plot_2d_function(data, objective_function, x_bound)
    elif num_variables == 2:
        plot_3d_function(data, objective_function, x_bound, y_bound)
    else:
        print(f"Error: Only 1 or 2-variable functions are supported, but found {num_variables} variables.")
        return

def plot_2d_function(data, objective_function, x_bound):
    """Plot 1D objective function and individuals."""
    x = np.linspace(x_bound[0], x_bound[1], 400)
    y = np.array([objective_function([i]) for i in x])

    # Plot the objective function
    plt.figure(figsize=(10, 6))
    plt.plot(x, y, label=f"{objective_function.__name__.capitalize()} Function", color="blue")

    # Plot individuals by iteration with different colors
    iterations = data["iteration"].unique()
    colors = plt.cm.rainbow(np.linspace(0, 1, len(iterations)))

    for idx, iteration in enumerate(iterations):
        iter_data = data[data["iteration"] == iteration]
        plt.scatter(
            iter_data["var_1"],
            iter_data["fitness"],
            color=colors[idx],
            label=f"Iteration {iteration}",
            edgecolors="black",
        )

    plt.xlim(x_bound)
    plt.xlabel("Variable 1")
    plt.ylabel("Objective Value")
    plt.title(f"1D {objective_function.__name__.capitalize()} Function with Individuals")
    plt.grid(True)
    plt.legend()

def plot_3d_function(data, objective_function, x_bound, y_bound):
    """Plot 3D objective function and individuals."""

    fig = plt.figure(figsize=(10, 8))
    ax = fig.add_subplot(111, projection="3d")

    x = np.linspace(x_bound[0], x_bound[1], 100)
    y = np.linspace(y_bound[0], y_bound[1], 100)
    X, Y = np.meshgrid(x, y)
    Z = np.array([[objective_function([i, j]) for j in y] for i in x])

    # Plot the objective function surface with transparency
    ax.plot_surface(X, Y, Z, cmap="viridis", alpha=0.5)

    # Scatter the individuals' positions
    iterations = data["iteration"].unique()
    colors = plt.cm.rainbow(np.linspace(0, 1, len(iterations)))

    for idx, iteration in enumerate(iterations):
        iter_data = data[data["iteration"] == iteration]
        ax.scatter(
            iter_data["var_1"],
            iter_data["var_2"],
            iter_data["fitness"],
            color=colors[idx],
            label=f"Iteration {iteration}",
            edgecolors="black",
        )

    ax.set_xlim(x_bound)
    ax.set_ylim(y_bound)
    ax.set_xlabel("Variable 1")
    ax.set_ylabel("Variable 2")
    ax.set_zlabel("Fitness")
    ax.set_title(f"3D {objective_function.__name__.capitalize()} Function with Individuals")
    ax.legend()

def main():
    # Parse command-line arguments
    args = parse_arguments()

    # Read the CSV data
    data = read_population_data(args.csv_file)

    # Plot individuals on top of the objective function
    plot_function_with_individuals(data, args.function, args.x_bound, args.y_bound)

    # Show the plot or save it
    if args.show:
        try:
            plt.show()
        except Exception:
            print(
                "Interactive plotting is not supported, saving the plot to a file instead."
            )
            plt.savefig(args.output)
            print(f"Plot saved to {args.output}")
    else:
        plt.savefig(args.output)
        print(f"Plot saved to {args.output}")

if __name__ == "__main__":
    main()
