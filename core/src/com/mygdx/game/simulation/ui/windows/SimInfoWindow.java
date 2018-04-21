package com.mygdx.game.simulation.ui.windows;

import com.kotcrab.vis.ui.util.TableUtils;
import com.kotcrab.vis.ui.widget.VisLabel;
import com.kotcrab.vis.ui.widget.VisTable;
import com.kotcrab.vis.ui.widget.VisWindow;
import com.mygdx.game.simulation.logic.algorithms.NBodyAlgorithm;

/**
 * Created by Fran on 4/20/2018.
 */
public class SimInfoWindow extends VisWindow{
    private NBodyAlgorithm nBodyAlgorithm;
    private VisLabel calcPerSecLabel;

    public SimInfoWindow(NBodyAlgorithm nBodyAlgorithm) {
        super("Simulation Information");
        this.nBodyAlgorithm = nBodyAlgorithm;
        TableUtils.setSpacingDefaults(this);
        addWidgets();
        pack();
    }

    private void addWidgets(){
        VisTable mainTable = new VisTable();

        calcPerSecLabel = new VisLabel();

        mainTable.add(calcPerSecLabel).width(300f);

        add(mainTable);
    }

    @Override
    public void act(float delta) {
        super.act(delta);
        calcPerSecLabel.setText("Calculations per second: " + nBodyAlgorithm.getCalcSec());
    }
}
