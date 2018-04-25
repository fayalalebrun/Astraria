package com.mygdx.game.simulation.ui.windows;

import com.badlogic.gdx.scenes.scene2d.Actor;
import com.badlogic.gdx.scenes.scene2d.utils.ChangeListener;
import com.kotcrab.vis.ui.util.TableUtils;
import com.kotcrab.vis.ui.widget.VisLabel;
import com.kotcrab.vis.ui.widget.VisTextButton;
import com.kotcrab.vis.ui.widget.VisWindow;

/**
 * Created by fraayala19 on 4/25/18.
 */
public class ControlsWindow extends VisWindow {
    public ControlsWindow() {
        super("Controls");
        TableUtils.setSpacingDefaults(this);
        addWidgets();
        pack();
    }

    private void addWidgets(){
        add(createLabel("WASD - Movement keys"));
        row();
        add(createLabel("Space/Shift - Move Up/Down"));
        row();
        add(createLabel("Q/E - Roll"));
        row();
        add(createLabel("Scrollwheel - Change camera speed"));
        row();
        add(createLabel("Up/Down arrows - Change camera speed"));
        row();
        add(createLabel("Left/Right arrows - Change simulation speed"));
        row();

        VisTextButton closeButton = new VisTextButton("Close");

        closeButton.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                close();
            }
        });

        add(closeButton);
        row();
    }

    public VisLabel createLabel(String text){
        return new VisLabel(text);
    }
}
