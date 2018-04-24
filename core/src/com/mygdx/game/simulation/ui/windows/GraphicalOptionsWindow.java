package com.mygdx.game.simulation.ui.windows;

import com.badlogic.gdx.scenes.scene2d.Actor;
import com.badlogic.gdx.scenes.scene2d.utils.ChangeListener;
import com.kotcrab.vis.ui.util.TableUtils;
import com.kotcrab.vis.ui.widget.VisCheckBox;
import com.kotcrab.vis.ui.widget.VisTextButton;
import com.kotcrab.vis.ui.widget.VisWindow;
import com.mygdx.game.simulation.Options;

/**
 * Created by Fran on 4/23/2018.
 */
public class GraphicalOptionsWindow extends VisWindow{
    public GraphicalOptionsWindow() {
        super("Visual Options");
        TableUtils.setSpacingDefaults(this);
        addWidgets();
        pack();
    }

    private void addWidgets(){
        final VisCheckBox orbitCheckBox = new VisCheckBox("Show orbits", Options.drawOrbits);
        final VisCheckBox lensGlowCheckBox = new VisCheckBox("Show Lens Glow", Options.drawLensGlow);
        final VisTextButton closeButton = new VisTextButton("Close");

        add(orbitCheckBox).left();
        row();

        add(lensGlowCheckBox).left();
        row();

        add(closeButton).right();
        row();

        orbitCheckBox.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                Options.drawOrbits = orbitCheckBox.isChecked();
            }
        });

        lensGlowCheckBox.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                Options.drawLensGlow = lensGlowCheckBox.isChecked();
            }
        });

        closeButton.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                close();
            }
        });
    }
}
