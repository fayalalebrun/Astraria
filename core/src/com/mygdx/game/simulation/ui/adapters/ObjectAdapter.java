package com.mygdx.game.simulation.ui.adapters;

import com.badlogic.gdx.scenes.scene2d.utils.Drawable;
import com.kotcrab.vis.ui.VisUI;
import com.kotcrab.vis.ui.util.adapter.ArrayListAdapter;
import com.kotcrab.vis.ui.widget.VisLabel;
import com.kotcrab.vis.ui.widget.VisTable;
import com.mygdx.game.simulation.SimulationObject;
import com.mygdx.game.simulation.SimulationScreen;
import com.mygdx.game.simulation.ui.windows.StatsWindow;

import java.util.ArrayList;

/**
 * Created by Fran on 4/18/2018.
 */
public class ObjectAdapter extends ArrayListAdapter<SimulationObject, VisTable>{

    private final Drawable bg = VisUI.getSkin().getDrawable("window-bg");
    private final Drawable selection = VisUI.getSkin().getDrawable("list-selection");

    private SimulationScreen simulationScreen;
    private final StatsWindow statsWindow;

    public ObjectAdapter(ArrayList<SimulationObject> array, SimulationScreen simulationScreen, StatsWindow statsWindow) {
        super(array);
        this.simulationScreen = simulationScreen;
        this.statsWindow = statsWindow;
        setSelectionMode(SelectionMode.SINGLE);
    }

    @Override
    protected VisTable createView(SimulationObject item) {
        VisTable table = new VisTable();
        table.setUserObject(item);
        table.add(new VisLabel(item.getName()));

        return table;
    }

    @Override
    protected void selectView(VisTable view) {
        simulationScreen.getRenderer().getCamera().setLock((SimulationObject)view.getUserObject());
        statsWindow.setSimObjectTracker(((SimulationObject)view.getUserObject()).getTracker());
        view.setBackground(selection);
    }

    @Override
    protected void deselectView(VisTable view) {
        view.setBackground(bg);
    }
}
