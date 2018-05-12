package com.mmcgarvey.advent.day1;

import org.junit.Test;

import java.util.ArrayList;
import java.util.List;

import static org.assertj.core.api.Assertions.assertThat;

public class CaptchaTest {

    @Test
    public void captcha_is_iterable_of_intpair() {
        Captcha captcha = Captcha.of(1,2,3,4);

        List<IntPair> captchaSequence = new ArrayList<>();
        captcha.forEach(captchaSequence::add);

        assertThat(captchaSequence).extracting(IntPair::getLeft).containsExactly(1,2,3,4);
        assertThat(captchaSequence).extracting(IntPair::getRight).containsExactly(2,3,4,1);
    }

    @Test
    public void captcha_can_be_made_from_numeric_string() {
        Captcha captcha = Captcha.of("1234");

        List<IntPair> captchaSequence = new ArrayList<>();
        captcha.forEach(captchaSequence::add);

        assertThat(captchaSequence).extracting(IntPair::getLeft).containsExactly(1,2,3,4);
        assertThat(captchaSequence).extracting(IntPair::getRight).containsExactly(2,3,4,1);
    }
}
