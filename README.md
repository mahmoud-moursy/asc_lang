## ASC; ASM for the [ATC fantasy console](https://github.com/atc_console)

Syntax:
```
/ Create comments by enclosing anything in two forward slashes. /

/ This program paints a line diagonally across the screen /

/ This includes two bytes for the header information /
bytes +02 +00;

/ let: declares a variable that does not 
  get replaced if it already exists /
let $00 0;
/ var: creates a variable and replaces 
  prexisting variable values        /
var $01 1;

/ $00 and $01 are addresses; Add $00's value (0, initially) to 
  $01's value (1), and save the output to $00 /
add $00 $01 $00;

/ +ab corresponds to a byte that isn't an address.
  At the bytecode level, there is no difference between
  variable addresses and bits, but this decision creates a
  clear distinction between constant inputs and variable inputs. /
pix $00 $00 +ab;
```