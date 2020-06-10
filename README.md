# Ceres 
a shitty fantasy console written in rust using a proprietary MIPS based asm instruction set. a bit of inspiration from PICO-8

### GRAPHICS

uhhhhh funny 128x72 screen

### Register layout and info

all registers are unsigned 16 bit

| Register number | Register name | Register usage       |
| --------------- | ------------- | -------------------- |
| 0               | z0            | always contains zero |
| 1               | gp            | global pointer       |
| 2               | gp            | stack pointer        |
| 3               | ra            | return address       |
| 4-6             | a0-a2         | argument registers   |
| 7               | v0            | return register      |
| 8               | v1            | return register 2    |
| 9-15            | t0-t6         | temporary registers  |

