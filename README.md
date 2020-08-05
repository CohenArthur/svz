# svz

Struct VisualiZation tool for C/C++ data structures.

`svz` allows you to view the relationship between the different data structures
in your code. It makes debugging easier, and allows skimming through structures
without looking for them in the whole file.

## Installation

`cargo install --git https://github.com/cohenarthur/svz`

## Usage

The `svz` binary produces a .dot graph.

`svz <file> | dot -Txlib` will directly show the graph.
`svz <file> | dot -Tpng > out.png` will save the graph to a png file named "out.png"

## Limitations

* As of right now, `svz` can only handle one file at a time. #12
* It's also not battle-tested when it comes to weird structure declarations, such as
those without a name and simply a typdef.
* This isn't tested yet, but I fear `svz` is quite slow. Some improvements can definitely
be made on the graph implementation, which right now is a travesty

## Later on

Genericizing `svz` so that it could expose traits, allowing almost any format of data
structure to be parsed would definitely be interesting. In the same way, outputting
to something other than .dot would also be nice.
