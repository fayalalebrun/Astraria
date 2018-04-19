package com.mygdx.game.simulation.ui.adapters;

import com.badlogic.gdx.scenes.scene2d.utils.Drawable;
import com.kotcrab.vis.ui.VisUI;
import com.kotcrab.vis.ui.util.adapter.ArrayListAdapter;
import com.kotcrab.vis.ui.widget.VisLabel;
import com.kotcrab.vis.ui.widget.VisTable;
import com.mygdx.game.simulation.SimulationObject;
import com.mygdx.game.simulation.SimulationScreen;
import com.mygdx.game.simulation.ui.windows.StatsWindow;
import net.dermetfan.utils.Pair;

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
        VisLabel label = new VisLabel(item.getName());
        table.setUserObject(new Pair<SimulationObject,VisLabel>(item,label));
        table.add(label);

        return table;
    }

    @Override
    protected void selectView(VisTable view) {
        Pair<SimulationObject,VisLabel> pair = (Pair<SimulationObject,VisLabel>)view.getUserObject();
        simulationScreen.getRenderer().getCamera().setLock(pair.getKey());
        statsWindow.setSimObjectTracker(pair.getKey().getTracker());
        view.setBackground(selection);
    }

    public void update(){
        for(VisTable view : this.getViews().values()){
            Pair<SimulationObject,VisLabel> pair = (Pair<SimulationObject,VisLabel>)view.getUserObject();
            pair.getValue().setText(pair.getKey().getName());
        }
    }

    @Override
    protected void deselectView(VisTable view) {
        view.setBackground(bg);
    }
}
