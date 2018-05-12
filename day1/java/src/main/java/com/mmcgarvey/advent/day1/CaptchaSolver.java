package com.mmcgarvey.advent.day1;

public class CaptchaSolver {
    public int solve(Captcha captcha) {
        int sum = 0;
        for (IntPair pair : captcha) {
            if (pair.getLeft() == pair.getRight()) {
                sum = sum + pair.getLeft();
            }
        }
        return sum;
    }
}
