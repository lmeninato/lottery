# Lottery Scheduling

This is a basic implementation of lottery scheduling to understand the impact of compensatory tickets.

## Input format

```
1 // use compensatory tickets or not
10 // quantum
5 // number of iterations to run of the main scheduling loop
2 // number of users
Amy // user name
150 // lottery tickets
4 // number of jobs
15 15 15 15 // work per job
Bob // ...
100
3
20 20 20
```

Here is a sample output from such a file:

```
Quantum is 10 (Comp tickets set to: true) 

User: Amy (150 tickets) has jobs:
Job 0
  Work: 15
  Remaining: 15
Job 1
  Work: 15
  Remaining: 15
Job 2
  Work: 15
  Remaining: 15
Job 3
  Work: 15
  Remaining: 15

User: Bob (100 tickets) has jobs:
Job 0
  Work: 20
  Remaining: 20
Job 1
  Work: 20
  Remaining: 20
Job 2
  Work: 20
  Remaining: 20

At time: 0, the winner was: Amy
At time: 10, the winner was: Amy
At time: 15, the winner was: Bob
At time: 25, the winner was: Bob
At time: 35, the winner was: Amy
There were 5 lotteries
User: Bob won 2 times (expected 2.00)
User: Bob performed 20 units of work (44.44%)
User: Amy won 3 times (expected 3.00)
User: Amy performed 25 units of work (55.56%)
```

To build, simply clone this directory and run `cargo build`.

To run, you can either use the executable directly or do `cargo run < sample.txt` or whatever
input file you'd like.

The sample file could be tweaked to have a different quantum, turn on/off compensatory tickets,
change the jobs each user submits, change the number of users, etc.
For instance, in `sample2.txt` we added another user `Carol`:

```
1
10
20
3
Amy 
150
4
15 15 15 15
Bob
100
3
20 20 20
Carol
200
1
60
```

Which generated the following trace:

```
Quantum is 10 (Comp tickets set to: true) 

User: Amy (150 tickets) has jobs:
Job 0
  Work: 15
  Remaining: 15
Job 1
  Work: 15
  Remaining: 15
Job 2
  Work: 15
  Remaining: 15
Job 3
  Work: 15
  Remaining: 15

User: Bob (100 tickets) has jobs:
Job 0
  Work: 20
  Remaining: 20
Job 1
  Work: 20
  Remaining: 20
Job 2
  Work: 20
  Remaining: 20

User: Carol (200 tickets) has jobs:
Job 0
  Work: 60
  Remaining: 60

At time: 0, the winner was: Amy
At time: 10, the winner was: Bob
At time: 20, the winner was: Bob
At time: 30, the winner was: Carol
At time: 40, the winner was: Carol
At time: 50, the winner was: Bob
At time: 50, the winner was: Carol
At time: 60, the winner was: Amy
At time: 65, the winner was: Carol
At time: 75, the winner was: Amy
At time: 85, the winner was: Amy
At time: 90, the winner was: Carol
At time: 100, the winner was: Bob
At time: 110, the winner was: Carol
At time: 120, the winner was: Amy
At time: 130, the winner was: Carol
At time: 130, the winner was: Amy
At time: 135, the winner was: Amy
At time: 145, the winner was: Bob
At time: 155, the winner was: Bob
There were 20 lotteries
User: Bob won 6 times (expected 4.44)
User: Bob performed 40 units of work (25.81%)
User: Amy won 7 times (expected 6.67)
User: Amy performed 55 units of work (35.48%)
User: Carol won 7 times (expected 8.89)
User: Carol performed 60 units of work (38.71%)
```

# Compensatory Tickets

In section 3.4 of the Lottery Scheduling [paper](https://www.usenix.org/legacy/publications/library/proceedings/osdi/full_papers/waldspurger.pdf)
the authors explain compensation tickets:

> A client which consumes only a fraction f of its allocated resource quantum can be granted a compensation
> ticket that inflates its value by 1/f until the client starts its
> next quantum. This ensures that each client’s resource consumption, equal to f times its per-lottery win probability p,
> is adjusted by 1/f to match its allocated share p. Without
> compensation tickets, a client that does not consume its entire allocated quantum would receive less than its entitled
> share of the processor.

