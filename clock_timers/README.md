CLOCK AND TIMERS - FINDINGS

In this chapter we studied about how to use delay without using its peripherals that is without implementing it.

We have change our way to use the delay as we were using before in previous chapters.

 Here we are going to implement the delay using hardware and for that we need to set time6 peripheral of the 
 hardware which is in apb1enr clock.

 time6 timer will provide this cr(ControlRegister) and now we need to write on this cr to set the counter in one 
 pulse mode which will make it work like an alarm clock.
 
 // opm -> In this mode counter will stop after the event occur & CEN (CounterEnable gets clear)

 // Now I am going to enable the counter in opm and will not set the counter to work.

We are going to use timer as the alarm clock we are going to set the delay time. Counter will count the time and reach 
to the end value provided by the Arr register and after that when counter reach to the end value it generate a overflow
and an event occur and status register set the uif flat to the counter.