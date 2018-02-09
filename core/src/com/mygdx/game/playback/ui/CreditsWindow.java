package com.mygdx.game.playback.ui;/*
* =====================================================================
* ==      Created by davrockenzahn19        ==    Date: 2/8/18   ==
* =====================================================================
* ==      Project: orbit-simulator-2    ==
* =====================================================================

*/

import com.kotcrab.vis.ui.widget.VisLabel;
import com.kotcrab.vis.ui.widget.VisWindow;

public class CreditsWindow extends VisWindow {

    public CreditsWindow(){
        super("Credits");

        add(new VisLabel("Programming \n" +
                              "     -   David Rockenzahn \n" +
                              "     -   Fransisco Ayala"));

        pack();
    }
}
