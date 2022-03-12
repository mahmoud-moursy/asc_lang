## ASC; ASM for the ATC fantasy console

Syntax:
```
/ Comments are enclosed in forward slashes /

/ Syntax of invocations below: /
pix 3 3 0;
/ All lines must end with ;. /

/ Access variable addresses (Upto Addr 255) with a $ /
w $3 3;

/ w writes value to a variable register. Each register stores exactly 64 bits of info. /

pix $3 $3 @ffffff;
```