package com.mygdx.game.playback.ui;

import com.badlogic.gdx.scenes.scene2d.Actor;
import com.badlogic.gdx.scenes.scene2d.utils.ChangeListener;
import com.kotcrab.vis.ui.util.TableUtils;
import com.kotcrab.vis.ui.widget.VisSlider;
import com.kotcrab.vis.ui.widget.VisTextButton;
import com.kotcrab.vis.ui.widget.VisWindow;
import com.mygdx.game.playback.PlayBackBody;
import com.mygdx.game.playback.PlayBackScreen;

/**
 * Created by Fran on 1/14/2018.
 */
public class ProgressWindow extends VisWindow{

    VisSlider slider;
    PlayBackScreen playBackScreen;

    public ProgressWindow(PlayBackScreen playBackScreen) {
        super("Playback Progress");

        this.playBackScreen = playBackScreen;

        TableUtils.setSpacingDefaults(this);
        addWidgets();
        pack();
    }

    private void addWidgets(){


        final VisSlider mySlider = new VisSlider(0f, 1f, 0.01f, false);
        slider = mySlider;
        add(slider).width(300);
        final PlayBackScreen myPlayBackScreen = playBackScreen;

        slider.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                if(mySlider.isDragging()) {
                    myPlayBackScreen.setCurrFrame((int) (myPlayBackScreen.getTotalFrames() * mySlider.getValue()));
                    myPlayBackScreen.setCurrTime((myPlayBackScreen.getTotalFrames() * mySlider.getValue()) / 60);
                }
            }
        });

        padRight(3f);

        final VisTextButton pauseButton = new VisTextButton("Pause");

        pauseButton.addListener(new ChangeListener() {
            private boolean lastState = false;
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                lastState=!lastState;
                myPlayBackScreen.setPaused(lastState);
            }
        });

        add(pauseButton);

    }

    @Override
    public void act(float delta) {
        float newValue = (float)playBackScreen.getCurrFrame()/playBackScreen.getTotalFrames();
        if(newValue<1&&!slider.isDragging()){
            slider.setValue(newValue);
        }

    }

}
