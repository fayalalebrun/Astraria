package com.mygdx.game.playback.ui;/*
* =====================================================================
* ==      Created by davrockenzahn19        ==    Date: 2/8/18   ==
* =====================================================================
* ==      Project: orbit-simulator-2    ==
* =====================================================================

*/

import com.badlogic.gdx.scenes.scene2d.Actor;
import com.badlogic.gdx.scenes.scene2d.utils.ChangeListener;
import com.kotcrab.vis.ui.widget.VisLabel;
import com.kotcrab.vis.ui.widget.VisSlider;
import com.kotcrab.vis.ui.widget.VisTextButton;
import com.kotcrab.vis.ui.widget.VisWindow;
import com.mygdx.game.playback.PlayBackBody;
import com.mygdx.game.playback.PlayBackScreen;

public class BodyScale extends VisWindow {

    private VisLabel label;
    private VisSlider slider;

    public BodyScale(final PlayBackScreen playBackScreen){
        super("Object scale");
        label = new VisLabel("Size: ");
        slider = new VisSlider(1f, 10f, 0.1f, false);

        add(label).padRight(10);
        add(slider);
        row();
        VisTextButton textButton = new VisTextButton("Set");
        add(textButton);
        pack();
        slider.setValue(0.05f);

        textButton.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                playBackScreen.setBodyScaleMod(slider.getValue());
            }
        });
    }
}
