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
        setPosition(0,105);
        label = new VisLabel("Size: ");
        slider = new VisSlider(0.01f, 0.1f, 0.01f, false);

        add(label).padRight(10);
        add(slider);
        row();
        VisTextButton textButton = new VisTextButton("Set");
        add(new VisTextButton("Close", new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                setVisible(false);
            }
        })).padTop(10).padLeft(30).padBottom(10);
        add(textButton).padTop(10).padLeft(50).padBottom(10);

        pack();
        slider.setValue(0.05f);

        textButton.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                for (PlayBackBody current : playBackScreen.getBodies()){
                    current.getSprite().setScale(slider.getValue());
                }
            }
        });
    }
}
