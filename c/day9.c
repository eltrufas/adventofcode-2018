#include <stdlib.h>

#include <stdint.h>
#include <stdio.h>

#define MARBLES 7216400 + 1
#define PLAYERS 419

struct Marble {
    size_t v;
    size_t n;
    size_t p;
};

void insert(struct Marble* ms, size_t i, size_t v) {
    ms[ms[i].n].p = v;
    ms[v].n = ms[i].n;
    ms[v].p = i;
    ms[i].n = v;
}

size_t lremove(struct Marble *ms, size_t i) {
    ms[ms[i].p].n = ms[i].n;
    ms[ms[i].p].p = ms[i].p;

    return ms[i].n;
}

int main() {
    size_t i, j;

    size_t current_player;
    size_t circle;
    size_t scores[PLAYERS];

    struct Marble *marbles = calloc(MARBLES, sizeof(struct Marble));

    circle = 0;
    marbles[0].v = 0;
    marbles[0].p = 0;
    marbles[0].n = 0;

    current_player = 0;

    for (i = 0; i < PLAYERS; i++) {
        scores[i] = 0;
    }

    for (i = 1; i <= MARBLES; i++) {
        marbles[i].v = i;
        if (i % 23) {
            circle = marbles[circle].n;
            insert(marbles, circle, i);
            circle = marbles[circle].n;
        } else {
            scores[current_player] += i;
            for (j = 0; j < 7; j++) {
                circle = marbles[circle].p;
            }
            scores[current_player] += circle;
            circle = lremove(marbles, circle);
        }
        current_player = (current_player + 1) % PLAYERS;
    }

    size_t m = 0;
    for (i = 0; i < PLAYERS; i++) {
        if (scores[i] > m) {
            m = scores[i];
        }
    }

    printf("%lu\n", m);
}
