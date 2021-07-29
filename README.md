This template will provide you the way to work with the stm32-discovery board as a compass by blinking the LED'S of the
Discovery Board based on the Magnetic Field Direction.

This template covers the [Take 1](https://docs.rust-embedded.org/discovery/15-led-compass/take-1.html) Challenge of The Discovery Book using lsm303agr. 

## What I am using here to complete the challenge.
- **Rust Programming Language.**
- **A stm32f3 Discovery Board.**
- **Few dependencies.**

## What we are actually doing here?

We are going to print the (x,y) axis values on itm terminal and will blink the LED'S based on the (x,y)->Magnetic Field
Direction.

#### To Verify it: On these (x,y) axis values these LED'S will show us the Direction of the Magnetic Field.

**if (x > 0, y > 0)**

- **(true, true) => Direction will be Southeast, LD9 will Blink**
- **(false, true) => Direction will be Northeast, LD5 will Blink**
- **(true, false) => Direction will be Southwest, LD8 will Blink**
- **(false, false) => Direction will be Northwest, LD4 will Blink**

## Build the Project
**Now before building this project you need to solder your board. It will help in printing the data to itm terminal.
Use this [link](https://docs.rust-embedded.org/discovery/06-hello-world/index.html) to solder your f3 Board.

#### Note: Make sure the F3 Board is connected to your computer.

### Step 1:
- Open terminal from **home directory** and execute Command

`cd /tmp && touch itm.txt`

Then

`itmdump -F -f itm.txt`

Leave this terminal running. Now in new terminal run command.

`cd /tmp && openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg`

### Step 2:
 Execute the Command 
 
`cargo run` 

from directory `stm32f3-compass`

Then we will be in gdb terminal. Now execute command:

- `return`
- `step`
- `continue`

Move your board and check the values of (x,y) and you will notice that your different Led will on whenever x,y will 
change their quadrant. 
