package com.mygdx.game;

/**
 * Created by fraayala19 on 12/12/17.
 */
public abstract class BaseScreen {
    private Boot boot;

    public BaseScreen(Boot boot) {
        this.boot = boot;
    }

    public Boot getBoot() {
        return boot;
    }
}
