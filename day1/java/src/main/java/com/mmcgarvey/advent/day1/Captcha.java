package com.mmcgarvey.advent.day1;

import java.util.ArrayList;
import java.util.Iterator;
import java.util.List;
import java.util.Spliterator;
import java.util.function.Consumer;

public class Captcha implements Iterable<IntPair> {
    private final List<IntPair> pairSequence;

    private Captcha(List<IntPair> pairSequence) {
        this.pairSequence = pairSequence;
    }

    public static Captcha of(int... sequence) {
        List<IntPair> intPairs = new ArrayList<>();
        for (int i = 0; i < sequence.length; i++) {
            int next = i + 1;
            if (next == sequence.length) {
                next = 0;
            }
            intPairs.add(new IntPair(sequence[i], sequence[next]));
        }
        return new Captcha(intPairs);
    }

    public static Captcha of(String input) {
        return Captcha.of(input.chars().map(Character::getNumericValue).toArray());
    }

    @Override
    public Iterator<IntPair> iterator() {
        return pairSequence.iterator();
    }

    @Override
    public void forEach(Consumer<? super IntPair> action) {
        pairSequence.forEach(action);
    }

    @Override
    public Spliterator<IntPair> spliterator() {
        return null;
    }
}
