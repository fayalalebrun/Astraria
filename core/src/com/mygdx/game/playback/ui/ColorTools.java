package com.mygdx.game.playback.ui;/*
* =====================================================================
* ==      Created by davrockenzahn19        ==    Date: 2/8/18   ==
* =====================================================================
* ==      Project: orbit-simulator-2    ==
* =====================================================================

*/

import com.badlogic.gdx.graphics.*;
import com.badlogic.gdx.graphics.Color;
import com.badlogic.gdx.scenes.scene2d.Actor;
import com.badlogic.gdx.scenes.scene2d.ui.Table;
import com.badlogic.gdx.scenes.scene2d.utils.ChangeListener;
import com.badlogic.gdx.scenes.scene2d.utils.ClickListener;
import com.kotcrab.vis.ui.widget.*;
import com.kotcrab.vis.ui.widget.color.ColorPicker;
import com.mygdx.game.playback.PlayBackBody;
import com.mygdx.game.playback.PlayBackScreen;

import java.awt.*;
import java.util.ArrayList;

public class ColorTools extends VisWindow {

    private ArrayList<VisRadioButton> buttonList;
    private VisTextButton color1Button;
    private VisTextButton color2Button;
    private VisLabel color1Label;
    private VisLabel color2Label;

    public ColorTools(final PlayBackScreen playBackScreen){
        super("Color tools");

        Table table = new Table();
        table.setFillParent(true);
        add(table);

         buttonList = new ArrayList<VisRadioButton>();

        final VisRadioButton preset1 = new VisRadioButton("Blue to white");
        final VisRadioButton preset2 = new VisRadioButton("Red to white");
        final VisRadioButton preset3 = new VisRadioButton("Green to white");
        final VisRadioButton custom = new VisRadioButton("Custom colors");

         color1Button = new VisTextButton("Set");
         color2Button = new VisTextButton("Set");
         color1Label = new VisLabel("Upper gradient");
         color2Label = new VisLabel("Lower gradient");

         color1Button.addListener(new ChangeListener() {
             @Override
             public void changed(ChangeEvent event, Actor actor) {
                 playBackScreen.getUiGroup().addActor(playBackScreen.getUpperColorPicker().fadeIn());
             }
         });

         color2Button.addListener(new ChangeListener() {
             @Override
             public void changed(ChangeEvent event, Actor actor) {
                 playBackScreen.getUiGroup().addActor(playBackScreen.getLowerColorPicker().fadeIn());
             }
         });

        buttonList.add(preset1);
        buttonList.add(preset2);
        buttonList.add(preset3);
        buttonList.add(custom);

        preset1.setChecked(true);
        disableOthers(0);

        final ColorTools tools = this;
        preset1.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                if (preset1.isChecked()){
                    disableOthers(0);
                    playBackScreen.setUpperColor(Color.ROYAL);
                    playBackScreen.setLowerColor(Color.CLEAR);
                }
            }
        });
        preset2.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                if (preset2.isChecked()){
                    disableOthers(1);
                    playBackScreen.setUpperColor(new Color(0.454f, 0.149f, 0.878f, 1));
                    playBackScreen.setLowerColor(new Color(0.556f, 0.101f, 0.858f, 1));

                }            }
        });
        preset3.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                if (preset3.isChecked()){
                    disableOthers(2);
                    playBackScreen.setUpperColor(Color.ROYAL);
                    playBackScreen.setLowerColor(Color.CLEAR);

                }            }
        });
        custom.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                if (custom.isChecked()){
                    disableOthers(3);

                }            }
        });


        table.defaults().left();

        table.row();
        table.add(new VisLabel("Color presets")).padBottom(10).padTop(20).expandX().fillX().row();
        table.add(preset1);
        table.row();
        table.add(preset2);
        table.row();
        table.add(preset3);
        table.row();
        table.add(new Separator()).padTop(20).fillX().expandX().row();
        table.row();
        table.add(custom).padTop(10).padBottom(10).expandX().fillX().row();



        table.add(color1Label);
        table.add(color1Button).padRight(20);
        table.row();
        table.add(color2Label);
        table.add(color2Button);
        pack();
    }

    public void disableOthers(int thisOne) {
        for (int i = 0; i < 4; i++) {
            if (i != thisOne) {
                buttonList.get(i).setChecked(false);
            }else {
                buttonList.get(i).setChecked(true);
            }
        }

        if (thisOne!=3){
            color1Button.setDisabled(true);
            color2Button.setDisabled(true);
            color1Label.setColor(Color.LIGHT_GRAY);
            color2Label.setColor(Color.LIGHT_GRAY);

        }else {
            color1Button.setDisabled(false);
            color2Button.setDisabled(false);
            color1Label.setColor(Color.WHITE);
            color2Label.setColor(Color.WHITE);
        }
    }

}


