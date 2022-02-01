
In section 3.4 of the Lottery Scheduling [paper](https://www.usenix.org/legacy/publications/library/proceedings/osdi/full_papers/waldspurger.pdf)
the authors explain compensation tickets:

> A client which consumes only a fraction f of its allocated resource quantum can be granted a compensation
> ticket that inflates its value by 1/f until the client starts its
> next quantum. This ensures that each client’s resource consumption, equal to f times its per-lottery win probability p,
> is adjusted by 1/f to match its allocated share p. Without
> compensation tickets, a client that does not consume its entire allocated quantum would receive less than its entitled
> share of the processor.

