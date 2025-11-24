how will dashing work?

- keep ticks from when switching from reverse to drive
- check if the user presses the gas within a very short amount of time after moving into drive
    - however; need to ensure that they pressed DOWN during that period, 
    not just holding down the key. There's a difference.

- when in a crazy dash, we need to allow the player to surpass max velocity. 
Which is slightly awkward because currently the logic has a strict max velocity.

Idea: may be a slightly awkward way to allow the max velocity to be raised 
(not a const anymore) if the player surpasses it (by crazy dashing). 
Then, if they dip back down to a normal speed....

How about implement it and then we can worry about semantics.