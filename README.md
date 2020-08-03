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
