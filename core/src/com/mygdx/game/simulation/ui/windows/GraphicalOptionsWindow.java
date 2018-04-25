package com.mygdx.game.simulation.ui.windows;

import com.badlogic.gdx.scenes.scene2d.Actor;
import com.badlogic.gdx.scenes.scene2d.utils.ChangeListener;
import com.kotcrab.vis.ui.util.TableUtils;
import com.kotcrab.vis.ui.widget.VisCheckBox;
import com.kotcrab.vis.ui.widget.VisTextButton;
import com.kotcrab.vis.ui.widget.VisWindow;
import com.mygdx.game.simulation.Options;
import com.mygdx.game.simulation.SimulationScreen;
import com.mygdx.game.simulation.ui.SimulationScreenInterface;

/**
 * Created by Fran on 4/23/2018.
 */
public class GraphicalOptionsWindow extends VisWindow{
    private SimulationScreen simulationScreen;

    private SimulationScreenInterface simulationScreenInterface;

    public GraphicalOptionsWindow(SimulationScreen simulationScreen, SimulationScreenInterface simulationScreenInterface) {
        super("Visual Options");
        this.simulationScreen = simulationScreen;
        this.simulationScreenInterface = simulationScreenInterface;
        TableUtils.setSpacingDefaults(this);
        addWidgets();
        pack();
    }

    private void addWidgets(){
        final VisCheckBox orbitCheckBox = new VisCheckBox("Show orbits", Options.drawOrbits);
        final VisCheckBox lensGlowCheckBox = new VisCheckBox("Show Lens Glow", Options.drawLensGlow);
        final VisCheckBox labelsCheckBox = new VisCheckBox("Show labels", Options.drawLabels);
        final VisCheckBox uiCheckBox = new VisCheckBox("Show UI(Press H)", true);
        final VisTextButton fullscreenButton = new VisTextButton("Toggle Fullscreen");
        final VisTextButton closeButton = new VisTextButton("Close");

        add(orbitCheckBox).left();
        row();

        add(lensGlowCheckBox).left();
        row();

        add(labelsCheckBox).left();
        row();

        add(uiCheckBox).left();
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

        uiCheckBox.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                simulationScreenInterface.setUIVisible(uiCheckBox.isChecked());
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
