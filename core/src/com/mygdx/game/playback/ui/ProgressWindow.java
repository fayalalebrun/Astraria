package com.mygdx.game.playback.ui;

import com.badlogic.gdx.scenes.scene2d.Actor;
import com.badlogic.gdx.scenes.scene2d.utils.ChangeListener;
import com.kotcrab.vis.ui.util.TableUtils;
import com.kotcrab.vis.ui.widget.VisLabel;
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
    VisLabel elapsedTime;

    public ProgressWindow(PlayBackScreen playBackScreen) {
        super("" );

        this.playBackScreen = playBackScreen;

        TableUtils.setSpacingDefaults(this);
        addWidgets();
        pack();
    }

    private void addWidgets(){


        elapsedTime = new VisLabel("99999s");
        add(elapsedTime);

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

        final VisTextButton pauseButton = new VisTextButton("Play");

        pauseButton.addListener(new ChangeListener() {
            private boolean lastState = true;
            @Override
            public void changed(ChangeEvent event, Actor actor) {

                if (lastState){
                    pauseButton.setText("Pause");
                }else {
                    pauseButton.setText("Play");
                }

                lastState=!lastState;
                myPlayBackScreen.setPaused(lastState);


            }
        });

        add(pauseButton).padRight(10);

        final VisTextButton doubleSpeed = new VisTextButton("2x");

        doubleSpeed.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                myPlayBackScreen.setTimeMultiplier(120);
            }
        });

        final VisTextButton normalSpeed = new VisTextButton("1x");

        normalSpeed.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                myPlayBackScreen.setTimeMultiplier(60);
            }
        });

        final VisTextButton halfSpeed = new VisTextButton("0.5x");

        halfSpeed.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                myPlayBackScreen.setTimeMultiplier(30);
            }
        });

        add(halfSpeed);
        add(normalSpeed);
        add(doubleSpeed);

    }

    @Override
    public void act(float delta) {
        float newValue = (float)playBackScreen.getCurrFrame()/playBackScreen.getTotalFrames();
        if(newValue<1&&!slider.isDragging()){
            slider.setValue(newValue);
        }
        elapsedTime.setText((int)(playBackScreen.getCurrFrame()/60f)+"s");

    }

}
