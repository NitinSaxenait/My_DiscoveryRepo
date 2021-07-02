
In this chapter - LEDs Again we are going to "on" the LEDs using registers only.
We are not going to initialize gpio, peripherals or rcc we are going to work on the registers of: Rcc and GPIO


**What we have done here?**
We have taken the GPIO AND RCC from crate stm32 Discovery and then we have passed the GPIO AND RCC as Register Block
which contains many more registers.
These Register block are having const address which are Raw Pointer by default.

How to access them?
Now to access these Raw pointers we have to take reference and then deference them but this is done only inside the 
unsafe block.

we have used -> `&*GPIOE::ptr(), &*RCC::ptr())` to get the access of the Register Blocks.
Now we have to enable the in/out/porte register using the rcc register(ahbenr) this is the register which provide us the
method to enable port e through "iopeen".

Next we have to set the pins of the port e as output which are input by default. This we do by using moder register
which is used to change the mode as input or output.

At last, we provide the LEDs bit 1 as power "on" using odr register to provide the output.

We have implemented this because using addresses directly can make us fall into bad address.

