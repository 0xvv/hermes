# Hermes

Rust based poker hand evaluator heavily inspired from PiMastah/pokerhandevaluator

Hands are encoded using 32 bits:

Each rank is mapped to a prime number, since products of prime numbers are unique we can use them as keys to represent each hands.
Suits are only important when we have a flush, we don't need to know the color of each card.

- bits 0 - 26 : Product of the 5 cards prime values 
- bit 27 : Flag if one card is a club
- bit 28 : Flag if one card is a diamond
- bit 29 : Flag if one card is a heart
- bit 30 : Flag if one card is a spade
- bit 31 : Unused bit

```
bits 31                15               0
      |                 |               |
      00000000 00000000 00000000 00000000
       |  |\                           /
       |  |  \                       /
      suits     products of primes   
```
After generating a map with each handkey and their rank we can simply look up a hand, lower is better.
