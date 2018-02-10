package com.mygdx.game.playback.ui;/*
* =====================================================================
* ==      Created by davrockenzahn19        ==    Date: 2/10/18   ==
* =====================================================================
* ==      Project: orbit-simulator-2    ==
* =====================================================================

*/

import com.badlogic.gdx.graphics.Color;
import com.badlogic.gdx.scenes.scene2d.Actor;
import com.badlogic.gdx.scenes.scene2d.utils.ChangeListener;
import com.kotcrab.vis.ui.widget.VisLabel;
import com.kotcrab.vis.ui.widget.VisTextButton;
import com.kotcrab.vis.ui.widget.VisTextField;
import com.kotcrab.vis.ui.widget.VisWindow;
import com.mygdx.game.playback.PlayBackScreen;

public class CamSpeed extends VisWindow{

    private VisLabel label;
    private VisTextField inputField;
    private VisTextButton resetButton;

    public CamSpeed(final PlayBackScreen playBackScreen){
        super("Camera speed");

        label = new VisLabel("Value: ");
        inputField = new VisTextField("1");
        resetButton = new VisTextButton("Default");

        add(label);
        add(inputField);
        row();
        add(new VisTextButton("Close", new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                setVisible(false);
            }
        }));
        add(resetButton);

        resetButton.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                playBackScreen.setCamVelocity(1000);
                inputField.setText("1");
            }
        });

        final VisTextButton setButton = new VisTextButton("Set");

        setButton.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                try {
                    float input = Float.parseFloat(inputField.getText());
                    inputField.setInputValid(true);
                    playBackScreen.setCamVelocity(input*1000f);

                }catch (Exception e) {
                    System.out.println("Error: invalid input. Cam speed input must be a \"float\" value");
                    inputField.setInputValid(false);
                }
            }
        });


        add(setButton);
        pack();

    }


}
