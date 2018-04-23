package com.mygdx.game.simulation.ui.windows;

import com.badlogic.gdx.scenes.scene2d.Actor;
import com.badlogic.gdx.scenes.scene2d.utils.ChangeListener;
import com.kotcrab.vis.ui.util.TableUtils;
import com.kotcrab.vis.ui.util.dialog.Dialogs;
import com.kotcrab.vis.ui.util.form.SimpleFormValidator;
import com.kotcrab.vis.ui.widget.VisLabel;
import com.kotcrab.vis.ui.widget.VisTextButton;
import com.kotcrab.vis.ui.widget.VisValidatableTextField;
import com.kotcrab.vis.ui.widget.VisWindow;
import com.mygdx.game.simulation.PlacementManager;
import com.mygdx.game.simulation.SimulationObject;

/**
 * Created by Fran on 4/22/2018.
 */
public class LaunchToolWindow extends VisWindow{
    private SimulationObject simulationObject;

    private final VisLabel speedLabel = new VisLabel("Object Speed(km/s): ");
    private final VisValidatableTextField speedField = new VisValidatableTextField();

    private final VisTextButton closeButton = new VisTextButton("Close");

    private final VisTextButton tracker = new VisTextButton("Tracker");

    private PlacementWindow placementWindow;

    private PlacementManager placementManager;

    public LaunchToolWindow(PlacementManager placementManager) {
        super("Launch Tool");
        this.placementManager = placementManager;
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

        SimpleFormValidator validator = new SimpleFormValidator(tracker);
        validator.floatNumber(speedField,"");
        validator.notEmpty(speedField,"");

        add(closeButton).right();
        row();
        pack();
    }

    public void setSimulationObject(SimulationObject simulationObject) {
        placementManager.setActive(true,simulationObject);
        this.simulationObject = simulationObject;
    }

    public double getSpeed(){
        if(tracker.isDisabled()){
            Dialogs.showErrorDialog(getStage(), "Invalid speed");
            return -1;
        }
        return Double.parseDouble(speedField.getText());
    }

    @Override
    public void setVisible(boolean visible) {
        super.setVisible(visible);
        if (!visible){
            placementManager.setActive(false,null);
        }
    }

    public void setPlacementWindow(PlacementWindow placementWindow) {
        this.placementWindow = placementWindow;
    }
}
