/* code adapted, some comments copied, from
[this page](https://prng.di.unimi.it/xoshiro256plusplus.c). */

/*  Written in 2019 by David Blackman and Sebastiano Vigna (vigna@acm.org)

To the extent possible under law, the author has dedicated all copyright
and related and neighboring rights to this software to the public domain
worldwide. This software is distributed without any warranty.

See <http://creativecommons.org/publicdomain/zero/1.0/>. */

/* This is xoshiro256++ 1.0, one of our all-purpose, rock-solid generators.
It has excellent (sub-ns) speed, a state (256 bits) that is large
enough for any parallel application, and it passes all tests we are
aware of.

For generating just floating-point numbers, xoshiro256+ is even faster.

The state must be seeded so that it is not everywhere zero. If you have
a 64-bit seed, we suggest to seed a splitmix64 generator and use its
output to fill s. */

pub struct Prng64 {
    seed: [u64; 4],
}

impl Prng64 {
    pub fn new(seed: [u64; 4]) -> Prng64 {
        Prng64 { seed }
    }

    pub fn seed(&mut self, seed: [u64; 4]) {
        self.seed = seed;
    }

    fn next(&mut self) -> u64 {
        let result: u64 = (self.seed[0].wrapping_add(self.seed[3]))
            .rotate_left(23)
            .wrapping_add(self.seed[0]);

        let t: u64 = self.seed[1] << 17;

        self.seed[2] ^= self.seed[0];
        self.seed[3] ^= self.seed[1];
        self.seed[1] ^= self.seed[2];
        self.seed[0] ^= self.seed[3];

        self.seed[2] ^= t;

        self.seed[3] = self.seed[3].rotate_left(45);

        result
    }

    /* This is the jump function for the generator. It is equivalent
    to 2^128 calls to next(); it can be used to generate 2^128
    non-overlapping subsequences for parallel computations. */
    pub fn jump(&mut self) {
        const JUMP: [u64; 4] = [
            0x180ec6d33cfd0aba,
            0xd5a61266f0c9392c,
            0xa9582618e03fc9aa,
            0x39abdc4529b1661c,
        ];

        let mut s0: u64 = 0;
        let mut s1: u64 = 0;
        let mut s2: u64 = 0;
        let mut s3: u64 = 0;

        for jump_constant in JUMP.iter() {
            for b in 0..64 {
                if (jump_constant & 1u64 << b) != 0 {
                    s0 ^= self.seed[0];
                    s1 ^= self.seed[1];
                    s2 ^= self.seed[2];
                    s3 ^= self.seed[3];
                }
                self.next();
            }
        }

        self.seed[0] = s0;
        self.seed[1] = s1;
        self.seed[2] = s2;
        self.seed[3] = s3;
    }

    /* This is the long-jump function for the generator. It is equivalent to
    2^192 calls to next(); it can be used to generate 2^64 starting points,
    from each of which jump() will generate 2^64 non-overlapping
    subsequences for parallel distributed computations. */
    pub fn long_jump(&mut self) {
        const LONG_JUMP: [u64; 4] = [
            0x76e15d3efefdcbbf,
            0xc5004e441c522fb3,
            0x77710069854ee241,
            0x39109bb02acbe635,
        ];

        let mut s0: u64 = 0;
        let mut s1: u64 = 0;
        let mut s2: u64 = 0;
        let mut s3: u64 = 0;

        for jump_constant in LONG_JUMP.iter() {
            for b in 0..64 {
                if (jump_constant & 1u64 << b) != 0 {
                    s0 ^= self.seed[0];
                    s1 ^= self.seed[1];
                    s2 ^= self.seed[2];
                    s3 ^= self.seed[3];
                }
                self.next();
            }
        }

        self.seed[0] = s0;
        self.seed[1] = s1;
        self.seed[2] = s2;
        self.seed[3] = s3;
    }
}

impl Iterator for Prng64 {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next())
    }
}
