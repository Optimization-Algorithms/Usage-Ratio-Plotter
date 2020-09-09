# Usage-Ratio-Plotter

![Rust](https://github.com/Optimization-Algorithms/Usage-Ratio-Plotter/workflows/Rust/badge.svg)

Plot usage ratios from Feature Kernel Log on PNG or SVG file

## Compile

To compile this program you need ```cargo```. Clone this repository, open it into 
your system shell then run

```bash
cargo build --release
```

The optimized executable will be located into  directory 
```target/release```, the 
executable name depends on the platform:
- ```urp``` on Unix
- ```urp.exe``` on Windows

## Usage

```urp``` generates a plot from the usage ratio csv generate by [feasth](https://github.com/Optimization-Algorithms/feasth). This plot will show 
how the usage ratio evolves during Feature Kernel iterations and the output
status of the sub problem. 

Each sub problem will be represented by a point in the graph. The color 
indicates the sub problem status

| Status           | Color |
|------------------|-------|
| Infeasible       | RED   |
| Linear Feasible  | BLUE  |
| Integer Feasible | GREEN |
| Timeout          | BLACK |

On the **X**-axis represents the sub problem id, the **Y**-axis indicates
the sub problem usage ratio.






