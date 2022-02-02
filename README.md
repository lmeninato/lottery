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

To build, simply clone this directory and run `cargo build`.

To run, you can either use the executable directly or do `cargo run < sample.txt` or whatever
input file you'd like.

# Compensatory Tickets

In section 3.4 of the Lottery Scheduling [paper](https://www.usenix.org/legacy/publications/library/proceedings/osdi/full_papers/waldspurger.pdf)
the authors explain compensation tickets:

> A client which consumes only a fraction f of its allocated resource quantum can be granted a compensation
> ticket that inflates its value by 1/f until the client starts its
> next quantum. This ensures that each client’s resource consumption, equal to f times its per-lottery win probability p,
> is adjusted by 1/f to match its allocated share p. Without
> compensation tickets, a client that does not consume its entire allocated quantum would receive less than its entitled
> share of the processor.

