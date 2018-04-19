package com.mygdx.game.simulation.ui.windows;

import com.kotcrab.vis.ui.util.TableUtils;
import com.kotcrab.vis.ui.widget.ListView;
import com.kotcrab.vis.ui.widget.VisScrollPane;
import com.kotcrab.vis.ui.widget.VisWindow;
import com.mygdx.game.simulation.SimulationObject;
import com.mygdx.game.simulation.SimulationScreen;
import com.mygdx.game.simulation.ui.adapters.ObjectAdapter;

import java.util.ArrayList;

/**
 * Created by Fran on 4/18/2018.
 */
public class ObjectListWindow extends VisWindow{

    private SimulationScreen simulationScreen;
    private ObjectAdapter objectAdapter;
    private final StatsWindow statsWindow;

    public ObjectListWindow(SimulationScreen simulationScreen, StatsWindow statsWindow) {
        super("Object List");
        this.simulationScreen = simulationScreen;
        this.statsWindow = statsWindow;
        TableUtils.setSpacingDefaults(this);
        addWidgets();
        setSize(200,270);

    }

    private void addWidgets(){
        objectAdapter = new ObjectAdapter(new ArrayList<SimulationObject>(),simulationScreen, statsWindow);
        ListView<SimulationObject> view = new ListView<SimulationObject>(objectAdapter);
        VisScrollPane scrollPane = new VisScrollPane(view.getMainTable());
        scrollPane.setFlickScroll(false);
        scrollPane.setFadeScrollBars(false);
        add(scrollPane).spaceTop(30).growX().width(205f).height(220f);
    }

    public void addObject(SimulationObject simulationObject){
        objectAdapter.add(simulationObject);
    }

    public void removeobject(SimulationObject simulationObject){
        objectAdapter.remove(simulationObject);
    }
}
