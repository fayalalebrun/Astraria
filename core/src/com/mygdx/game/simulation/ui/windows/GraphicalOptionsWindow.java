package com.mygdx.game.simulation.ui.windows;

import com.badlogic.gdx.scenes.scene2d.Actor;
import com.badlogic.gdx.scenes.scene2d.utils.ChangeListener;
import com.kotcrab.vis.ui.util.TableUtils;
import com.kotcrab.vis.ui.widget.VisCheckBox;
import com.kotcrab.vis.ui.widget.VisTextButton;
import com.kotcrab.vis.ui.widget.VisWindow;
import com.mygdx.game.simulation.Options;
import com.mygdx.game.simulation.SimulationScreen;

/**
 * Created by Fran on 4/23/2018.
 */
public class GraphicalOptionsWindow extends VisWindow{
    private SimulationScreen simulationScreen;

    public GraphicalOptionsWindow(SimulationScreen simulationScreen) {
        super("Visual Options");
        this.simulationScreen = simulationScreen;
        TableUtils.setSpacingDefaults(this);
        addWidgets();
        pack();
    }

    private void addWidgets(){
        final VisCheckBox orbitCheckBox = new VisCheckBox("Show orbits", Options.drawOrbits);
        final VisCheckBox lensGlowCheckBox = new VisCheckBox("Show Lens Glow", Options.drawLensGlow);
        final VisCheckBox labelsCheckBox = new VisCheckBox("Show labels", Options.drawLabels);
        final VisTextButton fullscreenButton = new VisTextButton("Toggle Fullscreen");
        final VisTextButton closeButton = new VisTextButton("Close");

        add(orbitCheckBox).left();
        row();

        add(lensGlowCheckBox).left();
        row();

        add(labelsCheckBox).left();
        row();

        add(fullscreenButton).left();
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

        labelsCheckBox.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                Options.drawLabels = labelsCheckBox.isChecked();
            }
        });

        fullscreenButton.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                simulationScreen.toggleFullscreen();
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
