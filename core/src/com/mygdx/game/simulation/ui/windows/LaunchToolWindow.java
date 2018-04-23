package com.mygdx.game.simulation.ui.windows;

import com.badlogic.gdx.scenes.scene2d.Actor;
import com.badlogic.gdx.scenes.scene2d.utils.ChangeListener;
import com.kotcrab.vis.ui.util.TableUtils;
import com.kotcrab.vis.ui.widget.VisLabel;
import com.kotcrab.vis.ui.widget.VisTextButton;
import com.kotcrab.vis.ui.widget.VisValidatableTextField;
import com.kotcrab.vis.ui.widget.VisWindow;
import com.mygdx.game.simulation.SimulationObject;

/**
 * Created by Fran on 4/22/2018.
 */
public class LaunchToolWindow extends VisWindow{
    private SimulationObject simulationObject;

    private final VisLabel speedLabel = new VisLabel("Object Speed(km/s): ");
    private final VisValidatableTextField speedField = new VisValidatableTextField();

    private final VisTextButton closeButton = new VisTextButton("Close");

    private PlacementWindow placementWindow;

    public LaunchToolWindow() {
        super("Launch Tool");
        setVisible(false);
        TableUtils.setSpacingDefaults(this);

        closeButton.addListener(new ChangeListener() {
            @Override
            public void changed(ChangeEvent event, Actor actor) {
                placementWindow.setVisible(true);
                setVisible(false);
            }
        });

        add(speedLabel).width(150f);
        add(speedField).width(100f);
        row();



        add(closeButton).right();
        row();
        pack();
    }

    public void setSimulationObject(SimulationObject simulationObject) {
        this.simulationObject = simulationObject;
    }

    public void setPlacementWindow(PlacementWindow placementWindow) {
        this.placementWindow = placementWindow;
    }
}
