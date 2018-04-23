package com.mygdx.game.simulation.ui.windows;

import com.kotcrab.vis.ui.widget.VisWindow;
import com.mygdx.game.simulation.SimulationObject;

/**
 * Created by Fran on 4/22/2018.
 */
public class LaunchToolWindow extends VisWindow{
    private SimulationObject simulationObject;

    public LaunchToolWindow() {
        super("Launch Tool");
        setVisible(false);
    }

    public void setSimulationObject(SimulationObject simulationObject) {
        this.simulationObject = simulationObject;
    }
}
