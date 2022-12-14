# rs_mazegen

This is the **Rust** implementation of the maze generator originally written in Python 
[here](https://github.com/thoutn/pymazegen). 

The library provides means to generate mazes using different graph algorithms. 
One can switch between these algorithms and observe the differences among the results. 
Each algorithm provides a unique maze structure.
The generated maze can be saved as an image, and used to solve it manually or using 
an automated maze solver. The raw format of the maze is also available, 
and is intended to be used for the latter.

## Folder structure

The project follows a standard folder structure: <br>
`src/` contains the source code, the main package. 
The `rs_mazegen` crate has several subpackages:
- `algos/` contains all the implemented maze generation algorithms.
      Each algorithm is contained in its own module.
- `maze/` contains the structs that are used to represent the maze - `Cell` and `Grid`
  for generating rectangular mazes, and `CirCell` and `CircGrid` for circular mazes
  represented in polar coordinates.
- `presenter/` contains the main methods to visualise the generated mazes.

## How to use the project

### The basics

Build mazes using the default algorithm `Algo::RecursiveBacktracking` and of default size
of `20x20`:

```
use rs_mazegen as m;
m::build();
```

And save them using `save_as_img` method:
```
m::save_as_img("maze_one");
```
This will save the maze to `maze_one.png` with the default file format of PNG.

### Customisation

Change the maze size using `init` method (e.g. to `15x50`):

```
m::init(15, 50);
m::build();
```

Change the algorithm using `build_with` instead of `build`:

```
use rs_mazegen::Algo;
m::build_with(Algo::Sidewinder);
```

Change the *cell size* and *wall thickness* of the maze when saving it into 
an image using `img_config` as follows:

```
m::img_config(20, 20);
m::save_as_img("img_config");
```

The above code will create an image of the maze with the same cell and wall size.

## Licence

[MIT License](LICENSE)