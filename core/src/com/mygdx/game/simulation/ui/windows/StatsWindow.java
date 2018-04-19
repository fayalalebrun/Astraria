package com.mygdx.game.simulation.ui.windows;

import com.kotcrab.vis.ui.util.TableUtils;
import com.kotcrab.vis.ui.widget.VisScrollPane;
import com.kotcrab.vis.ui.widget.VisTable;
import com.kotcrab.vis.ui.widget.VisWindow;
import com.mygdx.game.simulation.renderer.Camera;
import com.mygdx.game.simulation.ui.stat_trackers.SimObjectTracker;

/**
 * Created by fraayala19 on 4/19/18.
 */
public class StatsWindow extends VisWindow{

    SimObjectTracker simObjectTracker = null;

    VisTable mainTable;

    public StatsWindow() {
        super("Planet Statistics");
        TableUtils.setSpacingDefaults(this);
        mainTable = new VisTable();
        TableUtils.setSpacingDefaults(mainTable);
        addWidgets();
        setSize(250,270);
    }

    private void addWidgets(){
        VisScrollPane scrollPane = new VisScrollPane(mainTable);
        scrollPane.setFadeScrollBars(false);
        scrollPane.setFlickScroll(false);

        add(scrollPane).spaceTop(30).growX().width(245).height(220f);
    }

    public void setSimObjectTracker(SimObjectTracker simObjectTracker){
        this.simObjectTracker = simObjectTracker;
        mainTable.clear();
        simObjectTracker.addToTable(mainTable);
    }

    public void update(){
        if(simObjectTracker!=null) {
            this.simObjectTracker.update();
        }
    }
}
