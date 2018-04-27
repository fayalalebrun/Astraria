package com.mygdx.game.playback.ui;

import com.badlogic.gdx.graphics.Color;
import com.badlogic.gdx.scenes.scene2d.Actor;
import com.badlogic.gdx.scenes.scene2d.ui.Table;
import com.badlogic.gdx.scenes.scene2d.utils.ChangeListener;
import com.badlogic.gdx.scenes.scene2d.utils.ChangeListener.ChangeEvent;
import com.kotcrab.vis.ui.widget.Separator;
import com.kotcrab.vis.ui.widget.VisLabel;
import com.kotcrab.vis.ui.widget.VisRadioButton;
import com.kotcrab.vis.ui.widget.VisTextButton;
import com.kotcrab.vis.ui.widget.VisWindow;
import com.mygdx.game.playback.PlayBackScreen;
import java.util.ArrayList;

public class ColorTools extends VisWindow {
    private ArrayList<VisRadioButton> buttonList;
    private VisTextButton color1Button;
    private VisTextButton color2Button;
    private VisLabel color1Label;
    private VisLabel color2Label;

    public ColorTools(final PlayBackScreen playBackScreen) {
        super("Color tools");
        this.setPosition(0.0F, 201.0F);
        Table table = new Table();
        table.setFillParent(true);
        this.add(table);
        this.buttonList = new ArrayList();
        final VisRadioButton preset1 = new VisRadioButton("Galaxy collision (F)");
        final VisRadioButton preset2 = new VisRadioButton("Galaxy collision (T)");
        final VisRadioButton preset3 = new VisRadioButton("Spiral galaxy");
        final VisRadioButton preset4 = new VisRadioButton("Galaxy cluster 1");
        final VisRadioButton preset5 = new VisRadioButton("Galaxy cluster 2");
        final VisRadioButton preset6 = new VisRadioButton("Milky way");
        final VisRadioButton custom = new VisRadioButton("Custom colors");
        this.color1Button = new VisTextButton("Set");
        this.color2Button = new VisTextButton("Set");
        this.color1Label = new VisLabel("Upper gradient");
        this.color2Label = new VisLabel("Lower gradient");
        this.color1Button.addListener(new ChangeListener() {
            public void changed(ChangeEvent event, Actor actor) {
                playBackScreen.getUiGroup().addActor(playBackScreen.getUpperColorPicker().fadeIn());
            }
        });
        this.color2Button.addListener(new ChangeListener() {
            public void changed(ChangeEvent event, Actor actor) {
                playBackScreen.getUiGroup().addActor(playBackScreen.getLowerColorPicker().fadeIn());
            }
        });
        this.buttonList.add(preset1);
        this.buttonList.add(preset2);
        this.buttonList.add(preset3);
        this.buttonList.add(preset4);
        this.buttonList.add(preset5);
        this.buttonList.add(preset6);
        this.buttonList.add(custom);
        preset1.setChecked(true);
        this.disableOthers(0);
        preset1.addListener(new ChangeListener() {
            public void changed(ChangeEvent event, Actor actor) {
                if(preset1.isChecked()) {
                    ColorTools.this.disableOthers(0);
                    playBackScreen.setUpperColor(new Color(0.80784315F, 0.9607843F, 0.30588236F, 1.0F));
                    playBackScreen.setLowerColor(new Color(0.8666667F, 0.9607843F, 0.49803922F, 1.0F));
                }

            }
        });
        preset2.addListener(new ChangeListener() {
            public void changed(ChangeEvent event, Actor actor) {
                if(preset2.isChecked()) {
                    ColorTools.this.disableOthers(1);
                    playBackScreen.setUpperColor(new Color(Color.ROYAL));
                    playBackScreen.setLowerColor(new Color(Color.CLEAR));
                }

            }
        });
        preset3.addListener(new ChangeListener() {
            public void changed(ChangeEvent event, Actor actor) {
                if(preset3.isChecked()) {
                    ColorTools.this.disableOthers(2);
                    playBackScreen.setUpperColor(new Color(0.39215687F, 1.0F, 1.0F, 1.0F));
                    playBackScreen.setLowerColor(new Color(1.0F, 1.0F, 1.0F, 1.0F));
                }

            }
        });
        preset4.addListener(new ChangeListener() {
            public void changed(ChangeEvent event, Actor actor) {
                if(preset4.isChecked()) {
                    ColorTools.this.disableOthers(3);
                    playBackScreen.setUpperColor(Color.ROYAL);
                    playBackScreen.setLowerColor(Color.CLEAR);
                }

            }
        });
        preset5.addListener(new ChangeListener() {
            public void changed(ChangeEvent event, Actor actor) {
                if(preset5.isChecked()) {
                    ColorTools.this.disableOthers(4);
                    playBackScreen.setUpperColor(new Color(1.0F, 0.7058824F, 0.34901962F, 1.0F));
                    playBackScreen.setLowerColor(new Color(1.0F, 1.0F, 0.36862746F, 1.0F));
                }

            }
        });
        preset6.addListener(new ChangeListener() {
            public void changed(ChangeEvent event, Actor actor) {
                if(preset6.isChecked()) {
                    ColorTools.this.disableOthers(5);
                    playBackScreen.setUpperColor(new Color(0.26666668F, 0.4862745F, 0.5803922F, 1.0F));
                    playBackScreen.setLowerColor(new Color(0.41568628F, 0.4509804F, 0.4509804F, 1.0F));
                }

            }
        });
        custom.addListener(new ChangeListener() {
            public void changed(ChangeEvent event, Actor actor) {
                if(custom.isChecked()) {
                    ColorTools.this.disableOthers(6);
                }

            }
        });
        VisTextButton closeButton = new VisTextButton("Close");
        closeButton.addListener(new ChangeListener() {
            public void changed(ChangeEvent event, Actor actor) {
                ColorTools.this.setVisible(false);
            }
        });
        table.defaults().left();
        table.row();
        table.add(new VisLabel("Color presets")).padBottom(10.0F).padTop(20.0F).expandX().fillX().row();
        table.add(preset1);
        table.row();
        table.add(preset2);
        table.row();
        table.add(preset3);
        table.row();
        table.add(preset4).row();
        table.add(preset5).row();
        table.add(preset6).row();
        table.add(new Separator()).padTop(20.0F).fillX().expandX().row();
        table.row();
        table.add(custom).padTop(10.0F).padBottom(10.0F).expandX().fillX().row();
        table.add(this.color1Label);
        table.add(this.color1Button).padRight(20.0F);
        table.row();
        table.add(this.color2Label);
        table.add(this.color2Button);
        table.row();
        table.add(closeButton).padLeft(60.0F).padTop(20.0F);
        this.pack();
    }

    public void disableOthers(int thisOne) {
        for(int i = 0; i < 7; ++i) {
            if(i != thisOne) {
                ((VisRadioButton)this.buttonList.get(i)).setChecked(false);
            } else {
                ((VisRadioButton)this.buttonList.get(i)).setChecked(true);
            }
        }

        if(thisOne != 6) {
            this.color1Button.setDisabled(true);
            this.color2Button.setDisabled(true);
            this.color1Label.setColor(Color.GRAY);
            this.color2Label.setColor(Color.GRAY);
        } else {
            this.color1Button.setDisabled(false);
            this.color2Button.setDisabled(false);
            this.color1Label.setColor(Color.WHITE);
            this.color2Label.setColor(Color.WHITE);
        }
    }
}